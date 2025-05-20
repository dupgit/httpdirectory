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
        "https://ftp.lip6.fr/pub/OpenBSD/",
        "https://ftp.lysator.liu.se/pub/OpenBSD/",
    ];

    for url in url_array {
        match HttpDirectory::new(url).await {
            Ok(httpdir) => println!("{httpdir}"),
            Err(myerr) => println!("{myerr}"),
        }
    }

    if let Ok(httpdir) = HttpDirectory::new("https://cloud.debian.org/images/cloud/").await {
        match httpdir.dirs().filter_by_name("bookworm/") {
            Ok(mut httpdir) => {
                if httpdir.len() > 0 {
                    let entries = httpdir.entries();
                    match &entries[0].dirname() {
                        Some(dir) => {
                            let dir = dir.to_string();
                            match httpdir.cd(&dir).await {
                                Ok(bookworm) => println!("{bookworm}"),
                                Err(e) => println!("{e}"),
                            }
                        }
                        _ => println!("Not a directory !"),
                    }
                }
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}
