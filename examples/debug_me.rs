use httpdirectory::httpdirectory::HttpDirectory;
mod common;

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
async fn main() {
    // Logging system initialization with NO_COLOR compliance
    common::setup_logging_system();

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
                let sorted = httpdir.sort_by_size(true);
                println!("{}", sorted);
                println!("Len: {}", sorted.len());
            }
            Err(myerr) => println!("{myerr}"),
        }
    }
}
