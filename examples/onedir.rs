use env_logger::{Env, WriteStyle};
use httpdirectory::httpdirectory::HttpDirectory;
use std::env::var;

#[tokio::main]
async fn main() {
    let no_color_compliance = match var("NO_COLOR").is_ok() {
        true => WriteStyle::Never,
        false => WriteStyle::Auto,
    };

    // Replace default_filter_or("") by default_filter_or("debug") to see debug message by default
    // One may want to directly use RUST_LOG=httpdirectory=debug instead
    env_logger::Builder::from_env(Env::default().default_filter_or("")).write_style(no_color_compliance).init();

    let url_array = [
        "https://cloud.centos.org/centos/10-stream/x86_64/images/",
        "https://cloud-images.ubuntu.com/noble/20250430/",
        "https://mirrors.ircam.fr/pub/fedora/linux/releases",
        "https://cloud.debian.org/images/cloud/",
        "https://mirrors.ircam.fr/pub/",
    ];

    for url in url_array {
        match HttpDirectory::new(url).await {
            Ok(httpdir) => println!("{httpdir}"),
            Err(myerr) => println!("{myerr}"),
        }
    }
}
