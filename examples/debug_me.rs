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
        //"https://mirrors.ircam.fr/pub/elrepo/elrepo/el9/SRPMS/",
        //"https://stil.samake.se/demo/",
        //"https://debian.snt.utwente.nl/debian/",
        //"http://security.debian.org/",
        // use `python3 -m http.server -b 127.0.0.1 8888` command to
        // expose the current directory to your local machine port 8888
        "http://127.0.0.1:8888",
    ];

    for url in url_array {
        match HttpDirectory::new(url, Some(30)).await {
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
