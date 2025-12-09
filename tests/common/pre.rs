extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::{HttpDirectory, get_entries_from_body},
    httpdirectoryentry::{EntryType, HttpDirectoryEntry, assert_entry},
};
use httpmock::prelude::*;
use unwrap_unreachable::UnwrapUnreachable;

const BSD_EXAMPLE_INPUT: &str = r##"
<h1>Index of /pub/OpenBSD/</h1><hr><pre><a href="../">../</a>
<a href="7.5/">7.5/</a>                                               05-Apr-2024 11:59                   -
<a href="7.6/">7.6/</a>                                               08-Oct-2024 17:17                   -
<a href="7.7/">7.7/</a>                                               27-Apr-2025 17:58                   -
<a href="Changelogs/">Changelogs/</a>                                        12-May-2025 17:21                   -
<a href="LibreSSL/">LibreSSL/</a>                                          30-Apr-2025 06:55                   -
<a href="OpenBGPD/">OpenBGPD/</a>                                          06-Feb-2025 15:30                   -
<a href="OpenIKED/">OpenIKED/</a>                                          10-Apr-2025 17:10                   -
<a href="OpenNTPD/">OpenNTPD/</a>                                          09-Dec-2020 14:56                   -
<a href="OpenSSH/">OpenSSH/</a>                                           09-Apr-2025 07:08                   -
<a href="doc/">doc/</a>                                               28-Apr-2013 15:57                   -
<a href="patches/">patches/</a>                                           04-May-2025 21:25                   -
<a href="rpki-client/">rpki-client/</a>                                       11-Apr-2025 22:09                   -
<a href="signify/">signify/</a>                                           06-May-2025 15:03                   -
<a href="snapshots/">snapshots/</a>                                         13-May-2025 04:06                   -
<a href="songs/">songs/</a>                                             06-Apr-2023 22:15                   -
<a href="stable/">stable/</a>                                            18-Jan-2022 16:25                   -
<a href="syspatch/">syspatch/</a>                                          03-Mar-2025 15:17                   -
<a href="tools/">tools/</a>                                             07-Jan-2005 19:40                   -
<a href="README">README</a>                                             06-Oct-2017 11:51                1329
<a href="ftplist">ftplist</a>                                            13-May-2025 03:57                4836
<a href="timestamp">timestamp</a>                                          13-May-2025 04:00                  11
</pre><hr>
"##;

fn assert_bsd_example_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 22);

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "7.5/", 0, "2024-04-05 11:59");
    assert_entry(&entries[2], &EntryType::Directory, "7.6/", 0, "2024-10-08 17:17");
    assert_entry(&entries[3], &EntryType::Directory, "7.7/", 0, "2025-04-27 17:58");
    assert_entry(&entries[4], &EntryType::Directory, "Changelogs/", 0, "2025-05-12 17:21");
    assert_entry(&entries[5], &EntryType::Directory, "LibreSSL/", 0, "2025-04-30 06:55");
    assert_entry(&entries[6], &EntryType::Directory, "OpenBGPD/", 0, "2025-02-06 15:30");
    assert_entry(&entries[7], &EntryType::Directory, "OpenIKED/", 0, "2025-04-10 17:10");
    assert_entry(&entries[8], &EntryType::Directory, "OpenNTPD/", 0, "2020-12-09 14:56");
    assert_entry(&entries[9], &EntryType::Directory, "OpenSSH/", 0, "2025-04-09 07:08");
    assert_entry(&entries[10], &EntryType::Directory, "doc/", 0, "2013-04-28 15:57");
    assert_entry(&entries[11], &EntryType::Directory, "patches/", 0, "2025-05-04 21:25");
    assert_entry(&entries[12], &EntryType::Directory, "rpki-client/", 0, "2025-04-11 22:09");
    assert_entry(&entries[13], &EntryType::Directory, "signify/", 0, "2025-05-06 15:03");
    assert_entry(&entries[14], &EntryType::Directory, "snapshots/", 0, "2025-05-13 04:06");
    assert_entry(&entries[15], &EntryType::Directory, "songs/", 0, "2023-04-06 22:15");
    assert_entry(&entries[16], &EntryType::Directory, "stable/", 0, "2022-01-18 16:25");
    assert_entry(&entries[17], &EntryType::Directory, "syspatch/", 0, "2025-03-03 15:17");
    assert_entry(&entries[18], &EntryType::Directory, "tools/", 0, "2005-01-07 19:40");
    assert_entry(&entries[19], &EntryType::File, "README", 1329, "2017-10-06 11:51");
    assert_entry(&entries[20], &EntryType::File, "ftplist", 4836, "2025-05-13 03:57");
    assert_entry(&entries[21], &EntryType::File, "timestamp", 11, "2025-05-13 04:00");
}

#[allow(dead_code)]
pub async fn mock_bsd_example() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/bsd");

    let mock = server.mock(|when, then| {
        when.path("/bsd");
        then.status(200).body(BSD_EXAMPLE_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    let entries = httpdir.entries();
    assert_bsd_example_entries(entries);

    let files = httpdir.files();
    assert_eq!(files.len(), 3);

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_bsd_example() -> Result<(), Box<dyn std::error::Error>> {
    let body = BSD_EXAMPLE_INPUT;
    let entries = get_entries_from_body(body);

    assert_bsd_example_entries(&entries);

    Ok(())
}

const PRE_IMG_EXAMPLE_INPUT: &str = r##"
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN"
 "http://www.w3.org/TR/html4/strict.dtd">
<html>
 <head>
   <title>Ubuntu 24.04 LTS (Noble Numbat) daily [20250430]</title>
  <!-- Main style sheets for CSS2 capable browsers -->
  <style type="text/css" media="screen">
  @import url('https://cloud-images.ubuntu.com/include/style.css');
  pre { background: none; }
  body { margin: 2em; }
  table {
     margin: 0.5em 0;
     border-collapse: collapse;
  }
  td {
     padding: 0.25em;
     border: 1pt solid #C1B496; /* ubuntu dark tan */
  }
  td p {
     margin: 0;
     padding: 0;
  }
  </style>
 </head>
 <body><div id="pageWrapper">

<div id="header"><a href="http://www.ubuntu.com/"></a></div>

<div id="main">

<h1>Ubuntu 24.04 LTS (Noble Numbat) daily [20250430]</h1>

<p>The Ubuntu Cloud image can be run on your personal
<a class="http" href="http://www.ubuntu.com/business/cloud/overview">Ubuntu
Cloud</a>, or on <a class="http" href="http://www.ubuntu.com/cloud/public-cloud">public clouds that provide Ubuntu Certified Images.</a></p>

<p><h3>To find a listing of our public images on supported Clouds, please use the Cloud Image Locator:</h3></p>
<ul>
	<li><a class="http" href="https://cloud-images.ubuntu.com/locator/">Released Image locator</a>
	<li><a class="http" href="https://cloud-images.ubuntu.com/locator/daily/">Daily Image Locator</a>
</ul>
<p>
Cloud image specific bugs should be filed in the <a class="http" href="https://bugs.launchpad.net/cloud-images/+filebug">cloud-images</a> project on Launchpad.net.
</p>

<h2>Launching Ubuntu</h2>
<h3>KVM</h3>
<p>
When launching the download image from KVM, you will need to specify the virtio network driver.
</p>
<h3>LXD</h3>
<p>
First add the new Ubuntu images simplestreams endpoint:
</p>

<pre>

    lxc remote add --protocol simplestreams ubuntu-daily https://cloud-images.ubuntu.com/

</pre>

<p>
Launch the noble image:
</p>
<pre>

    lxc launch ubuntu-daily:noble

</pre><pre><img src="/icons/blank.gif" alt="Icon " width="22" height="22"> <a href="?C=N;O=D">Name</a>                                                                         <a href="?C=M;O=A">Last modified</a>      <a href="?C=S;O=A">Size</a>  <a href="?C=D;O=A">Description</a><hr><img src="/icons/back.gif" alt="[PARENTDIR]" width="22" height="22"> <a href="/noble/">Parent Directory</a>                                                                                  -
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="MD5SUMS">MD5SUMS</a>                                                                      2025-05-01 16:23  5.0K
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="MD5SUMS.gpg">MD5SUMS.gpg</a>                                                                  2025-05-01 16:23  833
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="SHA256SUMS">SHA256SUMS</a>                                                                   2025-05-01 16:23  7.0K
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="SHA256SUMS.gpg">SHA256SUMS.gpg</a>                                                               2025-05-01 16:23  833
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-amd64-azure.vhd.manifest">noble-server-cloudimg-amd64-azure.vhd.manifest</a>                               2025-04-30 04:21   20K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64-azure.vhd.tar.gz">noble-server-cloudimg-amd64-azure.vhd.tar.gz</a>                                 2025-04-30 04:21  557M  File system image and Kernel packed
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64-lxd.tar.xz">noble-server-cloudimg-amd64-lxd.tar.xz</a>                                       2025-04-30 13:12  408   Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-amd64-root.manifest">noble-server-cloudimg-amd64-root.manifest</a>                                    2025-04-30 13:12   18K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64-root.tar.xz">noble-server-cloudimg-amd64-root.tar.xz</a>                                      2025-04-30 13:12  209M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.daily.20250425.20250430.image_changelog.json">noble-server-cloudimg-amd64.daily.20250425.20250430.image_changelog.json</a>     2025-04-30 13:15  240K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.img">noble-server-cloudimg-amd64.img</a>                                              2025-04-30 13:11  584M  QCow2 UEFI/GPT Bootable disk image
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.manifest">noble-server-cloudimg-amd64.manifest</a>                                         2025-04-30 13:11   20K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.ova">noble-server-cloudimg-amd64.ova</a>                                              2025-04-30 13:12  555M  VMware/Virtualbox OVA
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.release.20250425.20250430.image_changelog.json">noble-server-cloudimg-amd64.release.20250425.20250430.image_changelog.json</a>   2025-04-30 13:15  240K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.squashfs">noble-server-cloudimg-amd64.squashfs</a>                                         2025-04-30 13:12  258M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.squashfs.manifest">noble-server-cloudimg-amd64.squashfs.manifest</a>                                2025-04-30 13:12   18K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.tar.gz">noble-server-cloudimg-amd64.tar.gz</a>                                           2025-04-30 13:13  509M  File system image and Kernel packed
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-amd64.vmdk">noble-server-cloudimg-amd64.vmdk</a>                                             2025-04-30 13:12  555M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64-lxd.tar.xz">noble-server-cloudimg-arm64-lxd.tar.xz</a>                                       2025-04-30 13:21  408   Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-arm64-root.manifest">noble-server-cloudimg-arm64-root.manifest</a>                                    2025-04-30 13:21   18K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64-root.tar.xz">noble-server-cloudimg-arm64-root.tar.xz</a>                                      2025-04-30 13:21  198M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.daily.20250425.20250430.image_changelog.json">noble-server-cloudimg-arm64.daily.20250425.20250430.image_changelog.json</a>     2025-04-30 13:24  200K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.img">noble-server-cloudimg-arm64.img</a>                                              2025-04-30 13:21  577M  QCow2 UEFI/GPT Bootable disk image
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.manifest">noble-server-cloudimg-arm64.manifest</a>                                         2025-04-30 13:21   20K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.release.20250425.20250430.image_changelog.json">noble-server-cloudimg-arm64.release.20250425.20250430.image_changelog.json</a>   2025-04-30 13:24  200K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.squashfs">noble-server-cloudimg-arm64.squashfs</a>                                         2025-04-30 13:21  246M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.squashfs.manifest">noble-server-cloudimg-arm64.squashfs.manifest</a>                                2025-04-30 13:21   18K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-arm64.tar.gz">noble-server-cloudimg-arm64.tar.gz</a>                                           2025-04-30 13:22  498M  File system image and Kernel packed
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf-lxd.tar.xz">noble-server-cloudimg-armhf-lxd.tar.xz</a>                                       2025-04-30 13:20  412   Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-armhf-root.manifest">noble-server-cloudimg-armhf-root.manifest</a>                                    2025-04-30 13:20   18K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf-root.tar.xz">noble-server-cloudimg-armhf-root.tar.xz</a>                                      2025-04-30 13:20  189M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.daily.20250425.20250430.image_changelog.json">noble-server-cloudimg-armhf.daily.20250425.20250430.image_changelog.json</a>     2025-04-30 13:22  198K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.img">noble-server-cloudimg-armhf.img</a>                                              2025-04-30 13:20  614M  QCow2 UEFI/GPT Bootable disk image
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.manifest">noble-server-cloudimg-armhf.manifest</a>                                         2025-04-30 13:20   20K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.release.20250425.20250430.image_changelog.json">noble-server-cloudimg-armhf.release.20250425.20250430.image_changelog.json</a>   2025-04-30 13:22  198K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.squashfs">noble-server-cloudimg-armhf.squashfs</a>                                         2025-04-30 13:20  240M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.squashfs.manifest">noble-server-cloudimg-armhf.squashfs.manifest</a>                                2025-04-30 13:20   18K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-armhf.tar.gz">noble-server-cloudimg-armhf.tar.gz</a>                                           2025-04-30 13:21  551M  File system image and Kernel packed
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el-lxd.tar.xz">noble-server-cloudimg-ppc64el-lxd.tar.xz</a>                                     2025-04-30 13:22  412   Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el-root.manifest">noble-server-cloudimg-ppc64el-root.manifest</a>                                  2025-04-30 13:22   18K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el-root.tar.xz">noble-server-cloudimg-ppc64el-root.tar.xz</a>                                    2025-04-30 13:22  212M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.daily.20250425.20250430.image_changelog.json">noble-server-cloudimg-ppc64el.daily.20250425.20250430.image_changelog.json</a>   2025-04-30 13:25  198K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.img">noble-server-cloudimg-ppc64el.img</a>                                            2025-04-30 13:22  603M  QCow2 UEFI/GPT Bootable disk image
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.manifest">noble-server-cloudimg-ppc64el.manifest</a>                                       2025-04-30 13:22   20K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.release.20250425.20250430.image_changelog.json">noble-server-cloudimg-ppc64el.release.20250425.20250430.image_changelog.json</a> 2025-04-30 13:25  198K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.squashfs">noble-server-cloudimg-ppc64el.squashfs</a>                                       2025-04-30 13:22  265M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.squashfs.manifest">noble-server-cloudimg-ppc64el.squashfs.manifest</a>                              2025-04-30 13:22   18K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-ppc64el.tar.gz">noble-server-cloudimg-ppc64el.tar.gz</a>                                         2025-04-30 13:23  538M  File system image and Kernel packed
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64-lxd.tar.xz">noble-server-cloudimg-riscv64-lxd.tar.xz</a>                                     2025-04-30 16:31  412   Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64-root.manifest">noble-server-cloudimg-riscv64-root.manifest</a>                                  2025-04-30 16:31   18K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64-root.tar.xz">noble-server-cloudimg-riscv64-root.tar.xz</a>                                    2025-04-30 16:31  203M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.daily.20250425.20250430.image_changelog.json">noble-server-cloudimg-riscv64.daily.20250425.20250430.image_changelog.json</a>   2025-04-30 16:33  198K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.img">noble-server-cloudimg-riscv64.img</a>                                            2025-04-30 16:31  604M  QCow2 UEFI/GPT Bootable disk image
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.manifest">noble-server-cloudimg-riscv64.manifest</a>                                       2025-04-30 16:31   19K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.release.20250425.20250430.image_changelog.json">noble-server-cloudimg-riscv64.release.20250425.20250430.image_changelog.json</a> 2025-04-30 16:33  198K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.squashfs">noble-server-cloudimg-riscv64.squashfs</a>                                       2025-04-30 16:31  256M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.squashfs.manifest">noble-server-cloudimg-riscv64.squashfs.manifest</a>                              2025-04-30 16:31   18K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-riscv64.tar.gz">noble-server-cloudimg-riscv64.tar.gz</a>                                         2025-04-30 16:32  537M  File system image and Kernel packed
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x-lxd.tar.xz">noble-server-cloudimg-s390x-lxd.tar.xz</a>                                       2025-04-30 13:11  408   Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-s390x-root.manifest">noble-server-cloudimg-s390x-root.manifest</a>                                    2025-04-30 13:11   18K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x-root.tar.xz">noble-server-cloudimg-s390x-root.tar.xz</a>                                      2025-04-30 13:11  203M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.daily.20250425.20250430.image_changelog.json">noble-server-cloudimg-s390x.daily.20250425.20250430.image_changelog.json</a>     2025-04-30 13:13   38K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.img">noble-server-cloudimg-s390x.img</a>                                              2025-04-30 13:11  554M  QCow2 UEFI/GPT Bootable disk image
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.manifest">noble-server-cloudimg-s390x.manifest</a>                                         2025-04-30 13:11   19K  Package manifest file
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.release.20250425.20250430.image_changelog.json">noble-server-cloudimg-s390x.release.20250425.20250430.image_changelog.json</a>   2025-04-30 13:13   38K  Image Changelog
<img src="/icons/unknown.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.squashfs">noble-server-cloudimg-s390x.squashfs</a>                                         2025-04-30 13:11  251M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds
<img src="/icons/text.gif" alt="[TXT]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.squashfs.manifest">noble-server-cloudimg-s390x.squashfs.manifest</a>                                2025-04-30 13:11   18K  Package manifest file
<img src="/icons/compressed.gif" alt="[   ]" width="22" height="22"> <a href="noble-server-cloudimg-s390x.tar.gz">noble-server-cloudimg-s390x.tar.gz</a>                                           2025-04-30 13:12  498M  File system image and Kernel packed
<img src="../../../../cdicons/folder.png" alt="[DIR]" width="22" height="22"> <a href="unpacked/">unpacked/</a>                                                                    2025-05-01 16:23    -
<hr></pre>
</div></div></body></html>"##;

fn assert_pre_img_example_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 70);

    assert_entry(&entries[0], &EntryType::ParentDirectory, "/noble/", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "MD5SUMS", 5_120, "2025-05-01 16:23");
    assert_entry(&entries[2], &EntryType::File, "MD5SUMS.gpg", 833, "2025-05-01 16:23");
    assert_entry(&entries[3], &EntryType::File, "SHA256SUMS", 7_168, "2025-05-01 16:23");
    assert_entry(&entries[4], &EntryType::File, "SHA256SUMS.gpg", 833, "2025-05-01 16:23");

    assert_entry(&entries[69], &EntryType::Directory, "unpacked/", 0, "2025-05-01 16:23");
}

#[allow(dead_code)]
pub async fn mock_pre_img_example() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(PRE_IMG_EXAMPLE_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    let entries = httpdir.entries();
    assert_pre_img_example_entries(entries);

    let files = httpdir.files();
    assert_eq!(files.len(), 68);

    // unreachable is is fine as we know that "b" regex is
    // a valid regex
    let filtered = files.filter_by_name("amd64.img").unreachable();
    let entries = filtered.entries();

    assert_eq!(filtered.len(), 1);

    assert_entry(&entries[0], &EntryType::File, "noble-server-cloudimg-amd64.img", 612_368_384, "2025-04-30 13:11");

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_pre_img_example() -> Result<(), Box<dyn std::error::Error>> {
    let body = PRE_IMG_EXAMPLE_INPUT;
    let entries = get_entries_from_body(body);
    assert_pre_img_example_entries(&entries);

    Ok(())
}
