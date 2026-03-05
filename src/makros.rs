#[macro_export]
macro_rules! err_exit {
    ($msg:expr) => {
        use owo_colors::OwoColorize;
        eprintln!("{} {}", "error:".red(), $msg);
        std::process::exit(1)
    };
}
