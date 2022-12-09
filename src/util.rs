#[macro_export]
macro_rules! kill {
    ($child:ident, $fmt:expr $(, $($arg:tt)*)?) => {
        log::error!($fmt, $($($arg)*)?);
        $child.lock().unwrap().kill().unwrap();
        std::process::exit(1);
    };
    ($fmt:expr $(, $($arg:tt)*)?) => {
        log::error!($fmt, $($($arg)*)?);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! dump_flag {
    ($e: expr) => {
        format!(
            "{} {}",
            format!("--{}", stringify!($e).split(".").last().unwrap()).dimmed(),
            $e.to_string().bold()
        )
    };
}
