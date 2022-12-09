use crate::meta;
pub use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author = meta::AUTHOR, version = meta::VERSION, about = meta::ABOUT)]
pub struct Args {
    #[arg(short = meta::L_FLAG_SHORT, long, help = meta::L_FLAG_HELP)]
    pub log_dirname: String,
    #[arg(short = meta::R_FLAG_SHORT, long, help = meta::R_FLAG_HELP)]
    pub log_maxrows: u64,
    #[arg(short = meta::F_FLAG_SHORT, long, help = meta::F_FLAG_HELP)]
    pub filter: String,
    #[arg(short = meta::I_FLAG_SHORT, long, help = meta::I_FLAG_HELP)]
    pub redis_ipv4: String,
    #[arg(short = meta::P_FLAG_SHORT, long, help = meta::P_FLAG_HELP)]
    pub redis_port: u16,
    #[arg(short = meta::W_FLAG_SHORT, long, help = meta::W_FLAG_HELP)]
    pub wgen_workload_file: String,
    #[arg(short = meta::A_FLAG_SHORT, long, help = meta::A_FLAG_HELP)]
    pub wgen_apispec_file: String,
    #[arg(short = meta::D_FLAG_SHORT, long, help = meta::D_FLAG_HELP)]
    pub wgen_day_length: String,
    #[arg(short = meta::C_FLAG_SHORT, long, help = meta::C_FLAG_HELP, action)]
    pub colorize: bool,
}

impl Args {
    pub fn validate(&self) {
        colored::control::set_override(self.colorize);
        if Path::new(&self.log_dirname).is_dir() {
            kill!("{}, folder already exists", dump_flag!(self.log_dirname));
        }
        if self.log_maxrows <= 0 {
            kill!("{}, must be > 0", dump_flag!(self.log_maxrows));
        }
        let filters = vec!["network", "structure", "none"];
        if !filters.into_iter().any(|c| c == &self.filter) {
            kill!("{}, invalid filter", dump_flag!(self.filter));
        }
        let ipre = Regex::new(meta::IPV4_REGEX).unwrap();
        if !ipre.is_match(&self.redis_ipv4) {
            kill!("{}, invalid IPv4 address", dump_flag!(self.redis_ipv4));
        }
        if self.redis_port <= 0 {
            kill!("{}, must be > 0", dump_flag!(self.redis_port));
        }
        if !Path::new(&self.wgen_workload_file).is_file() {
            kill!("{}, file not found", dump_flag!(self.wgen_workload_file));
        }
        if !Path::new(&self.wgen_apispec_file).is_file() {
            kill!("{}, file not found", dump_flag!(self.wgen_apispec_file));
        }
        let dlre = Regex::new(meta::DAYL_REGEX).unwrap();
        if !dlre.is_match(&self.wgen_day_length) {
            kill!("{}, invalid day length", dump_flag!(self.wgen_day_length));
        }
    }
}
