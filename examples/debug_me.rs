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
        // "https://cloud.centos.org/centos/10-stream/x86_64/images/",
        // "https://cloud-images.ubuntu.com/noble/20250430/",
        // "https://mirrors.ircam.fr/pub/fedora/linux/releases",
        // "https://cloud.debian.org/images/cloud/",
        // "https://mirrors.ircam.fr/pub/",
        // "https://ftp.lip6.fr/pub/OpenBSD/",
        // "https://ftp.lysator.liu.se/pub/OpenBSD/",
        // "http://mirror.nju.edu.cn/debian/",
        // "http://debian-archive.trafficmanager.net/debian/",
        // "http://mirrors.coreix.net/debian/",
        // "http://ftp.us.debian.org/debian/",
        // "http://mirror.cov.ukservers.com/debian/",
        // "http://debian.osuosl.org/debian/",
        // "http://mirrors.iu13.net/debian/",
        // "http://debian.mirror.uk.sargasso.net/debian/",
        "http://mirror.twds.com.tw/debian/",
        // "http://debian.ethz.ch/debian/",
        // "http://repository.su/debian/",
        // "http://debian.snt.utwente.nl/debian/",
        // "http://kartolo.sby.datautama.net.id/debian/",
        // "http://ftp.tu-chemnitz.de/debian/",
        // "http://mirrors.zju.edu.cn/debian/",
        // "http://mirrors.tuna.tsinghua.edu.cn/debian/",
        // "http://mirrors.neusoft.edu.cn/debian/",
        // "http://mirror.nyist.edu.cn/debian/",
        // "http://mirror.hnd.cl/debian/",
        // "http://debian.ludost.net/debian/",
        // "http://ftp.belnet.be/debian",
        // "http://mirrors.xtom.au/debian/",
        // "http://mirrors.asnet.am/debian/",
    ];

    for url in url_array {
        match HttpDirectory::new(url).await {
            Ok(httpdir) => println!("{httpdir}"),
            Err(myerr) => println!("{myerr}"),
        }
    }
}
