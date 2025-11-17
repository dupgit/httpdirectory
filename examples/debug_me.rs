use env_logger::{Env, WriteStyle};
use httpdirectory::httpdirectory::HttpDirectory;
use httpdirectory::httpdirectory::Sorting;
use std::env::var;

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]), flavor = "current_thread")]
async fn main() {
    let no_color_compliance = match var("NO_COLOR").is_ok() {
        true => WriteStyle::Never,
        false => WriteStyle::Auto,
    };

    // Replace default_filter_or("") by default_filter_or("debug") to see debug message by default
    // One may want to directly use RUST_LOG=httpdirectory=debug instead
    env_logger::Builder::from_env(Env::default().default_filter_or("")).write_style(no_color_compliance).init();

    let url_array = [
        // "https://cloud.centos.org/centos/10-stream/x86_64/images/",
        // "https://cloud-images.ubuntu.com/noble/20250430/",
        // "https://mirrors.ircam.fr/pub/fedora/linux/releases",
        // "https://cloud.debian.org/images/cloud/",
        //"http://prodata.swmed.edu/download/pub",
        "https://mirrors.ircam.fr/pub/elrepo/elrepo/el9/SRPMS/",
        // "http://127.0.0.1:8080",
    ];

    for url in url_array {
        match HttpDirectory::new(url).await {
            Ok(httpdir) => {
                println!("{httpdir:#?}");
                println!("{httpdir}");
                let sorted = httpdir.sort_by_size(Sorting::Ascending);
                println!("{}", sorted);
                println!("Len: {}", sorted.len());
            }
            Err(myerr) => println!("{myerr}"),
        }
    }
}
