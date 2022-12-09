#[macro_use]
mod util;
mod broker;
mod cli;
mod meta;
mod writer;

use crate::broker::Broker;
use crate::cli::Args;
use crate::cli::Parser;
use crate::writer::dispatcher::Dispatcher;
use colored::Colorize;
use crossbeam_channel::unbounded;
use log::error;
use log::info;
use log::trace;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use std::io::BufRead;
use std::io::BufReader;
use std::process;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h(xdriver {l})} {m}{n}")))
        .build();

    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
    {
        Ok(c) => c,
        Err(_) => {
            error!("unable to init log4rs config");
            process::exit(1);
        }
    };

    if let Err(_) = log4rs::init_config(config) {
        error!("unable to init log4rs");
        process::exit(1);
    }

    let cli_args = Arc::new(Args::parse());
    cli_args.validate();

    info!("{}", dump_flag!(cli_args.log_dirname));
    info!("{}", dump_flag!(cli_args.log_maxrows));
    info!("{}", dump_flag!(cli_args.filter));
    info!("{}", dump_flag!(cli_args.redis_ipv4));
    info!("{}", dump_flag!(cli_args.redis_port));
    info!("{}", dump_flag!(cli_args.wgen_workload_file));
    info!("{}", dump_flag!(cli_args.wgen_apispec_file));
    info!("{}", dump_flag!(cli_args.wgen_day_length));
    info!("{}", dump_flag!(cli_args.colorize));

    let (xtime, xtimeu) = cli_args
        .wgen_day_length
        .split_at(cli_args.wgen_day_length.len() - 1);

    let mut xtime = xtime.parse::<u64>().unwrap();
    let xtimeu = xtimeu.parse::<char>().unwrap();

    if xtimeu == 'm' {
        xtime = xtime * 60;
    } else if xtimeu == 'h' {
        xtime = xtime * 3600;
    };

    let (tx, rx) = unbounded();

    let mut workers = vec![];

    let stime = std::time::Instant::now();

    let wgen_handle = match process::Command::new("wgen")
        .arg("-w")
        .arg(&cli_args.wgen_workload_file)
        .arg("-a")
        .arg(&cli_args.wgen_apispec_file)
        .arg("-d")
        .arg(&cli_args.wgen_day_length.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => Arc::new(Mutex::new(child)),
        Err(_) => {
            kill!("wgen child process failed to start");
        }
    };

    let wgen_handle_stdout = wgen_handle
        .lock()
        .unwrap()
        .stdout
        .take()
        .expect("internal error: stdout pipe");
    workers.push(thread::spawn(move || {
        let lines = BufReader::new(wgen_handle_stdout).lines();
        for line in lines {
            let line = line.unwrap();
            info!("{}", line);
        }
    }));
    let wgen_handle_stderr = wgen_handle
        .lock()
        .unwrap()
        .stderr
        .take()
        .expect("internal error: stderr pipe");
    workers.push(thread::spawn(move || {
        let lines = BufReader::new(wgen_handle_stderr).lines();
        for line in lines {
            let line = line.unwrap();
            info!("{}", line);
        }
    }));

    let cli_args_dispatcher = cli_args.clone();
    let wgen_handle_dispatcher = wgen_handle.clone();
    workers.push(thread::spawn(move || {
        let mut dispatcher = Dispatcher::new(&cli_args_dispatcher, &wgen_handle_dispatcher, &stime);
        dispatcher.dispatch(
            rx,
            &cli_args_dispatcher,
            &wgen_handle_dispatcher,
            xtime as f64,
        );
        trace!("exit from Dispatcher thread");
    }));

    let cli_args_broker = cli_args.clone();
    let wgen_handle_broker = wgen_handle.clone();
    workers.push(thread::spawn(move || {
        let mut broker = Broker::new(&cli_args_broker, &wgen_handle_broker);
        broker.capture(tx, &cli_args_broker, &wgen_handle_broker);
        trace!("exit from Broker thread");
    }));

    for worker in workers {
        if let Err(_) = worker.join() {
            kill!(wgen_handle, "{}", "unable to merge thread");
        }
    }

    trace!("exit from main thread");
}
