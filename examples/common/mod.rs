use env_logger::{Env, WriteStyle};
use std::env::var;

fn no_color_compliance() -> WriteStyle {
    if var("NO_COLOR").is_ok() {
        WriteStyle::Never
    } else {
        WriteStyle::Auto
    }
}

pub fn setup_logging_system() {
    let no_color_compliance = no_color_compliance();

    // Replace default_filter_or("") by default_filter_or("debug") to see debug message by default
    // One may want to directly use RUST_LOG=httpdirectory=debug instead
    env_logger::Builder::from_env(Env::default().default_filter_or("")).write_style(no_color_compliance).init();
}
