use super::util;
use super::util::Writable;
use crate::cli::Args;
use colored::Colorize;
use log::info;
use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

#[derive(Debug)]
pub struct SerDes {
    logname: String,
    logfile_row: u64,
    logfile_num: u64,
    logfile_path: PathBuf,
    logfile: File,
}

impl SerDes {
    pub fn new(cli_args: &Arc<Args>, wgen_handle: &Arc<Mutex<Child>>, logname: String) -> Self {
        let (logfile_row, logfile_num) = (0, 0);
        let logdir_path = PathBuf::from(&cli_args.log_dirname);
        if let Err(_) = fs::create_dir_all(logdir_path.as_path()) {
            kill!(
                wgen_handle,
                "unable to create path `{}`",
                logdir_path.display()
            );
        }
        let logfile_path = logdir_path.join(format!("{}.{}.json", logfile_num, logname));
        let mut logfile = util::try_open(&logfile_path);

        if let Err(_) = write!(logfile, "[") {
            kill!(
                wgen_handle,
                "unable to write data to `{}`",
                logfile_path.display()
            );
        }

        Self {
            logname,
            logfile_row,
            logfile_num,
            logfile_path,
            logfile,
        }
    }
    pub fn log<T>(
        &mut self,
        data: &String,
        cli_args: &Arc<Args>,
        wgen_handle: &Arc<Mutex<Child>>,
        etime: &Duration,
        xtime: f64,
    ) where
        T: Writable + for<'a> Deserialize<'a>,
    {
        if let Ok(json_data) = util::try_deserialize::<T>(&data) {
            if self.logfile_row != 0 {
                write!(self.logfile, ",\n").unwrap();
            }
            info!(
                "{} {} {}",
                format!(
                    "+{:.2?} {:.2}%",
                    etime,
                    (etime.as_secs_f64() / xtime) * 100.0
                )
                .yellow(),
                format!(
                    "{} @ {}",
                    self.logfile_path.file_name().unwrap().to_str().unwrap(),
                    self.logfile_row,
                )
                .blue(),
                json_data.fmt(),
            );

            if let Err(_) = write!(self.logfile, "{}", json_data.log().unwrap()) {
                kill!(
                    wgen_handle,
                    "unable to write data to `{}`",
                    self.logfile_path.display()
                );
            }

            self.logfile_row = (self.logfile_row + 1) % cli_args.log_maxrows;
            if self.logfile_row == 0 {
                self.logfile_num += 1;
                self.logfile_path.pop();
                self.logfile_path = self
                    .logfile_path
                    .join(format!("{}.{}.json", self.logfile_num, self.logname));
                if let Err(_) = write!(self.logfile, "]") {
                    kill!(
                        wgen_handle,
                        "unable to write data to `{}`",
                        self.logfile_path.display()
                    );
                }
                self.logfile = util::try_open(&self.logfile_path);
                if let Err(_) = write!(self.logfile, "[") {
                    kill!(
                        wgen_handle,
                        "unable to write data to `{}`",
                        self.logfile_path.display()
                    );
                }
            }
        }
    }
    pub fn close(&mut self, wgen_handle: &Arc<Mutex<Child>>) {
        if let Err(_) = write!(self.logfile, "]") {
            kill!(
                wgen_handle,
                "unable to write data to `{}`",
                self.logfile_path.display()
            );
        }
    }
}
