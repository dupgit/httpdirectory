use httpdirectory::httpdirectory::HttpDirectory;
mod common;

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
async fn main() {
    // Logging system initialization with NO_COLOR compliance
    common::setup_logging_system();

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
                                    println!("{}", bookworm.dirs().sort_by_date(true));
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
