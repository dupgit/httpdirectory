use env_logger::{Env, WriteStyle};
use httpdirectory::httpdirectory::{HttpDirectory, Sorting};
use std::env::var;

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
async fn main() {
    let no_color_compliance = match var("NO_COLOR").is_ok() {
        true => WriteStyle::Never,
        false => WriteStyle::Auto,
    };

    // Replace default_filter_or("") by default_filter_or("debug") to see debug message by default
    // One may want to directly use RUST_LOG=httpdirectory=debug instead
    env_logger::Builder::from_env(Env::default().default_filter_or("")).write_style(no_color_compliance).init();

    if let Ok(httpdir) = HttpDirectory::new("https://cloud.debian.org/images/cloud/").await {
        match httpdir.dirs().filter_by_name("trixie/") {
            Ok(httpdir) => {
                if httpdir.len() > 0 {
                    let entries = httpdir.entries();
                    match &entries[0].name() {
                        Some(dir) => {
                            let dir = dir.to_string();
                            match httpdir.cd(&dir).await {
                                Ok(bookworm) => {
                                    println!("{bookworm}");
                                    println!("Directories (if any)");
                                    println!("{}", bookworm.dirs().sort_by_date(Sorting::Ascending));
                                    println!("Files (if any)");
                                    println!("{}", bookworm.files());
                                }
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
