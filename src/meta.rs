pub const AUTHOR: &str = "Valentino Di Giosaffatte (resu-gh), Riccardo Armando Di Prinzio (ricdip)";
pub const VERSION: &str = "0.1.0";
pub const ABOUT: &str = "dmon test executor";

pub const L_FLAG_SHORT: char = 'l';
pub const L_FLAG_HELP: &str = "logging directory name (must not exist)";

pub const R_FLAG_SHORT: char = 'r';
pub const R_FLAG_HELP: &str = "max number of rows for each log file (must be > 0)";

pub const F_FLAG_SHORT: char = 'f';
pub const F_FLAG_HELP: &str = "payload filter type (network, structure, none)";

pub const I_FLAG_SHORT: char = 'i';
pub const I_FLAG_HELP: &str = "redis server ipv4 address (ex.: 0.0.0.0)";

pub const P_FLAG_SHORT: char = 'p';
pub const P_FLAG_HELP: &str = "redis server port number (ex.: 6379)";

pub const W_FLAG_SHORT: char = 'w';
pub const W_FLAG_HELP: &str = "wgen workload file (must be a yaml and must exist)";

pub const A_FLAG_SHORT: char = 'a';
pub const A_FLAG_HELP: &str = "wgen apispec file (must be a yaml and must exist)";

pub const D_FLAG_SHORT: char = 'd';
pub const D_FLAG_HELP: &str = "wgen day length in seconds (must be > 0)";

pub const C_FLAG_SHORT: char = 'c';
pub const C_FLAG_HELP: &str = "colorize output";

pub const IPV4_REGEX: &str = "^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$";
pub const DAYL_REGEX: &str = "^[1-9]{1}[0-9]*[smh]{1}$";
