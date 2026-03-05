#[macro_export]
macro_rules! err_exit {
    ($msg:expr) => {
        use owo_colors::OwoColorize;
        eprintln!("{} {}", "error:".red(), $msg);
        std::process::exit(1)
    };
}

#[macro_export]
macro_rules! warn_exit {
    ($msg:expr) => {
        use owo_colors::OwoColorize;
        eprintln!("{} {}", "warn:".yellow(), $msg);
        std::process::exit(0)
    };
}
