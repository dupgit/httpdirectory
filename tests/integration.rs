extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::HttpDirectory, httpdirectory::Sorting, httpdirectoryentry::EntryType,
    httpdirectoryentry::assert_entry,
};
use httpmock::prelude::*;

#[tokio::test]
async fn test_empty_200_status() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/hello");

    let mock = server.mock(|when, then| {
        when.path("/hello");
        then.status(200);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert!(httpdir.is_empty());
    mock.assert();
}

#[tokio::test]
async fn test_empty_404_status() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/hello");

    let mock = server.mock(|when, then| {
        when.path("/hello");
        then.status(404);
    });

    match HttpDirectory::new(&url).await {
        Ok(httpdir) => panic!("This test should return an Error. We got {httpdir:?}"),
        Err(e) => assert_eq!(e.to_string(), format!("Error: Error while retrieving url {url} content: 404 Not Found")),
    };

    mock.assert();
}

#[tokio::test]
async fn test_debian_example() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(r##"
        <meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
        <title>Debian Official Cloud Images -- Getting Debian  - www.debian.org</title>
        <link rel="author" href="mailto:webmaster@debian.org">
        <link href="https://www.debian.org/debian.css" rel="stylesheet" type="text/css">
        <link href="https://www.debian.org/debian-en.css" rel="stylesheet" type="text/css" media="all">


        <div id="header">
          <div id="upperheader">
            <div id="logo">
              <a href="https://www.debian.org/" title="Debian Home"><img src="https://www.debian.org/Pics/openlogo-50.png" alt="Debian" width="50" height="61"></a>
            </div> <!-- end logo -->
            <div id="navbar">
              <p class="hidecss"><a href="#content">Skip Quicknav</a></p>
              <ul>
	<li><a href="https://www.debian.org/intro/about">About Debian</a></li>
	<li><a href="https://www.debian.org/distrib/">Getting Debian</a></li>
	<li><a href="https://www.debian.org/support">Support</a></li>
	<li><a href="https://www.debian.org/devel/">Developers' Corner</a></li>
              </ul>
            </div> <!-- end navbar -->
          </div> <!-- end upperheader -->
          <h1>Debian Official Cloud Images</h1>

          <p>
            In this page you can find the Debian cloud images provided by the Debian Cloud Team for some cloud providers.
            End users do not need to download these images, as they are
            usually provided by their cloud providers.
            For now we are supporting:

            <ul>
              <li><i>Amazon EC2 (amd64, arm64; Also see <a href="https://wiki.debian.org/Cloud/AmazonEC2Image">the wiki</a> and the <a href="https://aws.amazon.com/marketplace/seller-profile?id=4d4d4e5f-c474-49f2-8b18-94de9d43e2c0&ref=dtl_B0859NK4HC">AWS Marketplace listing</a></i>)</li>
              <li><i>Microsoft Azure (amd64; Also see <a href="https://wiki.debian.org/Cloud/MicrosoftAzure">the wiki</a> and <a href="https://azuremarketplace.microsoft.com/en-us/marketplace/apps?search=debian&page=1">The Azure Marketplace</a></i>)</li>
              <li><i>OpenStack (amd64, arm64, ppc64el; two
              flavours <a href="https://cloud.debian.org/cdimage/cloud/OpenStack/">using
              openstack-debian-images</a> and using the <a href="https://cloud.debian.org/cdimage/cloud/bullseye">toolchain</a> from the
              cloud team.
	  Also see <a href="https://wiki.debian.org/OpenStack">the wiki</a></i>)</li>
              <li><i>Plain VM (amd64)</i>, suitable for use with QEMU</li>
            </ul>


            From buster on we provide images for different cloud providers in
            one directory. There we use file names like this:

            <ul>
              <li><tt>debian-11-generic-ppc64el-daily-20210425-618.qcow2</tt></li>
              <li><tt>debian-11-genericcloud-amd64-daily-20210425-618.qcow2</tt></li>
              <li><tt>debian-11-ec2-arm64-daily-20210425-618.tar.xz</tt></li>
            </ul>

            <ul>
          <li><i>azure</i>: Optimized for the Microsoft Azure environment</li>
          <li><i>ec2</i>: Optimized for the Amazon EC2</li>
          <li><i>generic</i>: Should run in any environment using cloud-init,
          for e.g. OpenStack, DigitalOcean and also on bare metal.</li>
          <li><i>genericcloud</i>: Similar to generic. Should run in any
          virtualised environment. Is smaller than `generic` by excluding
          drivers for physical hardware.</li>
          <li><i>nocloud</i>: Mostly useful for testing the build process
           itself. Doesn't have cloud-init installed, but instead allows root
           login without a password. </li>
           </ul>

          </p>

          <h2>How to upload to OpenStack?</h2>

          <p>Once you have downloaded the image, you would typically need to upload it to
          Glance, using a command like this one (example for amd64):</p>

          <pre>openstack image create \
            --container-format bare \
            --disk-format qcow2 \
            --property hw_disk_bus=scsi \
            --property hw_scsi_model=virtio-scsi \
            --property os_type=linux \
            --property os_distro=debian \
            --property os_admin_user=debian \
            --property os_version='10.9.1' \
            --public \
            --file debian-10-generic-amd64-20210329-591.qcow2 \
            debian-10-generic-amd64-20210329-591.qcow2</pre>

          <p>Note that <i>hw_disk_bus=scsi</i> and <i>hw_scsi_model=virtio-scsi</i>
          select the virtio-scsi driver instead of the virtio-blk, which is nicer
          (on older versions of Qemu, virtio-blk doesn't have the FSTRIM feature,
          for example). Also, the properties <i>os_type, os_distro, os_version and
          os_admin_user</i> are OpenStack standards as per
          <a href="https://docs.openstack.org/glance/latest/admin/useful-image-properties.html">this
          document</a>. It is best practice to set them, especially on public clouds,
          to allow your cloud users to filter the image list to search what they need,
          for example using a command like this one:

          <pre>openstack image list --property os_distro=debian</pre>

          <h2>How can I verify my download is correct and exactly what has been
            created by Debian?</h2>

          <p>For the current official images (in the per-distribution
            directories), the safest method is to download the image and
            checksum files over TLS from <tt>cloud.debian.org</tt> or <tt>cdimage.debian.org</tt>.
            These names support DNSSEC, so a validating resolver can ensure
            that a client is connected to a Debian host.  And TLS ensures that
            the data is not manipulated in flight.</p>

          <p>The legacy OpenStack images (in the <tt>OpenStack/</tt> directory)
            provide checksums and signatures.  See SHA512SUMS.sign, etc. For
            more information about the verification steps, read the <a href="https://www.debian.org/CD/verify">verification guide</a></p>

          <p>If you're interested in contributing checksum signatures for the
            current images, please reach us on the list: <b>debian-cloud at lists.debian.org</b>.</p>

          <h2>Other questions?</h2>

          <p>Questions can be forwarded to the Debian Cloud Team: <b>debian-cloud at lists.debian.org</b>.</p>

        </div>
          <table id="indexlist">
           <tr class="indexhead"><th class="indexcolicon"><img src="/icons2/blank.png" alt="[ICO]"></th><th class="indexcolname"><a href="?C=N;O=D">Name</a></th><th class="indexcollastmod"><a href="?C=M;O=A">Last modified</a></th><th class="indexcolsize"><a href="?C=S;O=A">Size</a></th></tr>
           <tr class="indexbreakrow"><th colspan="4"><hr></th></tr>
           <tr class="even"><td class="indexcolicon"><a href="/images/"><img src="/icons2/go-previous.png" alt="[PARENTDIR]"></a></td><td class="indexcolname"><a href="/images/">Parent Directory</a></td><td class="indexcollastmod">&nbsp;</td><td class="indexcolsize">  - </td></tr>
           <tr class="odd"><td class="indexcolicon"><a href="OpenStack/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="OpenStack/">OpenStack/</a></td><td class="indexcollastmod">2024-07-01 23:19  </td><td class="indexcolsize">  - </td></tr>
           <tr class="even"><td class="indexcolicon"><a href="bookworm-backports/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="bookworm-backports/">bookworm-backports/</a></td><td class="indexcollastmod">2025-04-28 21:33  </td><td class="indexcolsize">  - </td></tr>
           <tr class="odd"><td class="indexcolicon"><a href="bookworm/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="bookworm/">bookworm/</a></td><td class="indexcollastmod">2025-04-28 20:53  </td><td class="indexcolsize">  - </td></tr>
           <tr class="even"><td class="indexcolicon"><a href="bullseye-backports/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="bullseye-backports/">bullseye-backports/</a></td><td class="indexcollastmod">2025-05-05 17:45  </td><td class="indexcolsize">  - </td></tr>
           <tr class="odd"><td class="indexcolicon"><a href="bullseye/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="bullseye/">bullseye/</a></td><td class="indexcollastmod">2025-05-05 16:52  </td><td class="indexcolsize">  - </td></tr>
           <tr class="even"><td class="indexcolicon"><a href="buster-backports/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="buster-backports/">buster-backports/</a></td><td class="indexcollastmod">2024-07-03 21:46  </td><td class="indexcolsize">  - </td></tr>
           <tr class="odd"><td class="indexcolicon"><a href="buster/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="buster/">buster/</a></td><td class="indexcollastmod">2024-07-03 21:46  </td><td class="indexcolsize">  - </td></tr>
           <tr class="even"><td class="indexcolicon"><a href="sid/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="sid/">sid/</a></td><td class="indexcollastmod">2024-04-01 14:20  </td><td class="indexcolsize">  - </td></tr>
           <tr class="odd"><td class="indexcolicon"><a href="stretch-backports/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="stretch-backports/">stretch-backports/</a></td><td class="indexcollastmod">2019-07-18 10:40  </td><td class="indexcolsize">  - </td></tr>
           <tr class="even"><td class="indexcolicon"><a href="stretch/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="stretch/">stretch/</a></td><td class="indexcollastmod">2019-07-18 10:40  </td><td class="indexcolsize">  - </td></tr>
           <tr class="odd"><td class="indexcolicon"><a href="trixie/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="trixie/">trixie/</a></td><td class="indexcollastmod">2023-07-25 07:43  </td><td class="indexcolsize">  - </td></tr>
           <tr class="indexbreakrow"><th colspan="4"><hr></th></tr>
        </table>
        <address>Apache/2.4.63 (Unix) Server at cloud.debian.org Port 443</address>"##);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.len(), 12);
    let entries = httpdir.entries();

    assert_eq!(httpdir.get_url().to_string(), url.to_string());
    assert_entry(&entries[0], &EntryType::ParentDirectory, "/images/", 0, "0000-00-00, 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "OpenStack/", 0, "2024-07-01 23:19");
    assert_entry(&entries[2], &EntryType::Directory, "bookworm-backports/", 0, "2025-04-28 21:33");
    assert_entry(&entries[3], &EntryType::Directory, "bookworm/", 0, "2025-04-28 20:53");
    assert_entry(&entries[4], &EntryType::Directory, "bullseye-backports/", 0, "2025-05-05 17:45");
    assert_entry(&entries[5], &EntryType::Directory, "bullseye/", 0, "2025-05-05 16:52");
    assert_entry(&entries[6], &EntryType::Directory, "buster-backports/", 0, "2024-07-03 21:46");
    assert_entry(&entries[7], &EntryType::Directory, "buster/", 0, "2024-07-03 21:46");
    assert_entry(&entries[8], &EntryType::Directory, "sid/", 0, "2024-04-01 14:20");
    assert_entry(&entries[9], &EntryType::Directory, "stretch-backports/", 0, "2019-07-18 10:40");
    assert_entry(&entries[10], &EntryType::Directory, "stretch/", 0, "2019-07-18 10:40");
    assert_entry(&entries[11], &EntryType::Directory, "trixie/", 0, "2023-07-25 07:43");

    let dirs = httpdir.dirs();
    assert_eq!(dirs.len(), 11);

    // unwrap is is fine as we know that "b" regex is
    // a valid regex
    let filtered = dirs.filter_by_name("b").unwrap();
    let entries = filtered.entries();

    assert_eq!(filtered.len(), 7);

    assert_entry(&entries[0], &EntryType::Directory, "bookworm-backports/", 0, "2025-04-28 21:33");
    assert_entry(&entries[1], &EntryType::Directory, "bookworm/", 0, "2025-04-28 20:53");
    assert_entry(&entries[2], &EntryType::Directory, "bullseye-backports/", 0, "2025-05-05 17:45");
    assert_entry(&entries[3], &EntryType::Directory, "bullseye/", 0, "2025-05-05 16:52");
    assert_entry(&entries[4], &EntryType::Directory, "buster-backports/", 0, "2024-07-03 21:46");
    assert_entry(&entries[5], &EntryType::Directory, "buster/", 0, "2024-07-03 21:46");
    assert_entry(&entries[6], &EntryType::Directory, "stretch-backports/", 0, "2019-07-18 10:40");

    mock.assert();
}

// Tests <pre> tag with other formatted columns
#[tokio::test]
async fn test_bsd_example() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/bsd");

    let mock = server.mock(|when, then| {
        when.path("/bsd");
        then.status(200).body(r##"
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
        "##);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    // The library fails to get this directory properly
    assert_eq!(httpdir.len(), 22);
    let entries = httpdir.entries();

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

    let files = httpdir.files();
    assert_eq!(files.len(), 3);

    mock.assert();
}

// Tests <pre> tag with other formatted columns
#[tokio::test]
async fn test_old_bsd_example() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = &server.url("/old-bsd/");

    let mock = server.mock(|when, then| {
        when.path("/old-bsd/");
        then.status(200).body(r##"
            <!DOCTYPE html><html><head><meta http-equiv="content-type" content="text/html; charset=utf-8"><meta name="viewport" content="width=device-width"><style type="text/css">body,html {background:#fff;font-family:"Bitstream Vera Sans","Lucida Grande","Lucida Sans Unicode",Lucidux,Verdana,Lucida,sans-serif;}tr:nth-child(even) {background:#f4f4f4;}th,td {padding:0.1em 0.5em;}th {text-align:left;font-weight:bold;background:#eee;border-bottom:1px solid #aaa;}#list {border:1px solid #aaa;width:100%;}a {color:#a33;}a:hover {color:#e33;}</style>

            <title>Index of /pub/OpenBSD/</title>
            </head><body><h1>Index of /pub/OpenBSD/</h1>
            <table id="list"><thead><tr><th style="width:55%"><a href="?C=N&amp;O=A">File Name</a>&nbsp;<a href="?C=N&amp;O=D">&nbsp;&darr;&nbsp;</a></th><th style="width:20%"><a href="?C=S&amp;O=A">File Size</a>&nbsp;<a href="?C=S&amp;O=D">&nbsp;&darr;&nbsp;</a></th><th style="width:25%"><a href="?C=M&amp;O=A">Date</a>&nbsp;<a href="?C=M&amp;O=D">&nbsp;&darr;&nbsp;</a></th></tr></thead>
            <tbody><tr><td class="link"><a href="../">Parent directory/</a></td><td class="size">-</td><td class="date">-</td></tr>
            <tr><td class="link"><a href="2.0/" title="2.0">2.0/</a></td><td class="size">-</td><td class="date">2001-Jun-04 14:06</td></tr>
            <tr><td class="link"><a href="2.1/" title="2.1">2.1/</a></td><td class="size">-</td><td class="date">2001-Jun-04 15:20</td></tr>
            <tr><td class="link"><a href="2.2/" title="2.2">2.2/</a></td><td class="size">-</td><td class="date">2018-Feb-09 09:55</td></tr>
            <tr><td class="link"><a href="2.3/" title="2.3">2.3/</a></td><td class="size">-</td><td class="date">2001-Jun-04 15:42</td></tr>
            <tr><td class="link"><a href="2.4/" title="2.4">2.4/</a></td><td class="size">-</td><td class="date">2001-Jun-04 15:51</td></tr>
            <tr><td class="link"><a href="2.5/" title="2.5">2.5/</a></td><td class="size">-</td><td class="date">2000-Jul-08 02:57</td></tr>
            <tr><td class="link"><a href="2.6/" title="2.6">2.6/</a></td><td class="size">-</td><td class="date">2001-Jun-07 00:50</td></tr>
            <tr><td class="link"><a href="2.7/" title="2.7">2.7/</a></td><td class="size">-</td><td class="date">2003-Oct-24 19:50</td></tr>
            <tr><td class="link"><a href="2.8/" title="2.8">2.8/</a></td><td class="size">-</td><td class="date">2003-Oct-24 21:17</td></tr>
            <tr><td class="link"><a href="2.9/" title="2.9">2.9/</a></td><td class="size">-</td><td class="date">2002-Oct-30 14:41</td></tr>
            <tr><td class="link"><a href="3.0/" title="3.0">3.0/</a></td><td class="size">-</td><td class="date">2005-Oct-20 19:58</td></tr>
            <tr><td class="link"><a href="3.1/" title="3.1">3.1/</a></td><td class="size">-</td><td class="date">2005-Oct-20 20:00</td></tr>
            <tr><td class="link"><a href="3.2/" title="3.2">3.2/</a></td><td class="size">-</td><td class="date">2005-Oct-20 20:01</td></tr>
            <tr><td class="link"><a href="3.3/" title="3.3">3.3/</a></td><td class="size">-</td><td class="date">2004-Nov-06 17:21</td></tr>
            <tr><td class="link"><a href="3.4/" title="3.4">3.4/</a></td><td class="size">-</td><td class="date">2004-Nov-06 17:15</td></tr>
            <tr><td class="link"><a href="3.5/" title="3.5">3.5/</a></td><td class="size">-</td><td class="date">2005-Oct-20 20:03</td></tr>
            <tr><td class="link"><a href="3.6/" title="3.6">3.6/</a></td><td class="size">-</td><td class="date">2004-Nov-12 18:36</td></tr>
            <tr><td class="link"><a href="3.7/" title="3.7">3.7/</a></td><td class="size">-</td><td class="date">2005-Apr-01 05:17</td></tr>
            <tr><td class="link"><a href="3.8/" title="3.8">3.8/</a></td><td class="size">-</td><td class="date">2005-Sep-25 07:30</td></tr>
            <tr><td class="link"><a href="3.9/" title="3.9">3.9/</a></td><td class="size">-</td><td class="date">2006-May-01 10:47</td></tr>
            <tr><td class="link"><a href="4.0/" title="4.0">4.0/</a></td><td class="size">-</td><td class="date">2006-Dec-10 20:53</td></tr>
            <tr><td class="link"><a href="4.1/" title="4.1">4.1/</a></td><td class="size">-</td><td class="date">2007-Apr-30 17:15</td></tr>
            <tr><td class="link"><a href="4.2/" title="4.2">4.2/</a></td><td class="size">-</td><td class="date">2007-Oct-31 22:10</td></tr>
            <tr><td class="link"><a href="4.3/" title="4.3">4.3/</a></td><td class="size">-</td><td class="date">2008-Apr-30 18:58</td></tr>
            <tr><td class="link"><a href="4.4/" title="4.4">4.4/</a></td><td class="size">-</td><td class="date">2008-Sep-04 22:43</td></tr>
            <tr><td class="link"><a href="4.5/" title="4.5">4.5/</a></td><td class="size">-</td><td class="date">2009-Mar-25 16:36</td></tr>
            <tr><td class="link"><a href="4.6/" title="4.6">4.6/</a></td><td class="size">-</td><td class="date">2009-Oct-09 00:26</td></tr>
            <tr><td class="link"><a href="4.7/" title="4.7">4.7/</a></td><td class="size">-</td><td class="date">2010-Apr-04 22:23</td></tr>
            <tr><td class="link"><a href="4.8/" title="4.8">4.8/</a></td><td class="size">-</td><td class="date">2010-Nov-01 16:22</td></tr>
            <tr><td class="link"><a href="4.9/" title="4.9">4.9/</a></td><td class="size">-</td><td class="date">2011-Apr-28 06:01</td></tr>
            <tr><td class="link"><a href="5.0/" title="5.0">5.0/</a></td><td class="size">-</td><td class="date">2011-Oct-31 02:31</td></tr>
            <tr><td class="link"><a href="5.1/" title="5.1">5.1/</a></td><td class="size">-</td><td class="date">2012-May-02 00:20</td></tr>
            <tr><td class="link"><a href="5.2/" title="5.2">5.2/</a></td><td class="size">-</td><td class="date">2013-Mar-05 11:07</td></tr>
            <tr><td class="link"><a href="5.3/" title="5.3">5.3/</a></td><td class="size">-</td><td class="date">2013-Jul-25 14:25</td></tr>
            <tr><td class="link"><a href="5.4/" title="5.4">5.4/</a></td><td class="size">-</td><td class="date">2014-Mar-02 11:06</td></tr>
            <tr><td class="link"><a href="5.5/" title="5.5">5.5/</a></td><td class="size">-</td><td class="date">2014-May-01 05:01</td></tr>
            <tr><td class="link"><a href="5.6/" title="5.6">5.6/</a></td><td class="size">-</td><td class="date">2014-Nov-01 14:09</td></tr>
            <tr><td class="link"><a href="5.7/" title="5.7">5.7/</a></td><td class="size">-</td><td class="date">2015-May-14 10:48</td></tr>
            <tr><td class="link"><a href="5.8/" title="5.8">5.8/</a></td><td class="size">-</td><td class="date">2015-Oct-18 14:54</td></tr>
            <tr><td class="link"><a href="5.9/" title="5.9">5.9/</a></td><td class="size">-</td><td class="date">2016-Mar-27 23:01</td></tr>
            <tr><td class="link"><a href="6.0/" title="6.0">6.0/</a></td><td class="size">-</td><td class="date">2016-Aug-23 22:23</td></tr>
            <tr><td class="link"><a href="6.1/" title="6.1">6.1/</a></td><td class="size">-</td><td class="date">2017-Apr-11 22:57</td></tr>
            <tr><td class="link"><a href="6.2/" title="6.2">6.2/</a></td><td class="size">-</td><td class="date">2017-Oct-11 10:17</td></tr>
            <tr><td class="link"><a href="6.3/" title="6.3">6.3/</a></td><td class="size">-</td><td class="date">2018-Apr-07 00:17</td></tr>
            <tr><td class="link"><a href="6.4/" title="6.4">6.4/</a></td><td class="size">-</td><td class="date">2018-Oct-29 16:29</td></tr>
            <tr><td class="link"><a href="6.5/" title="6.5">6.5/</a></td><td class="size">-</td><td class="date">2019-Aug-11 14:18</td></tr>
            <tr><td class="link"><a href="6.6/" title="6.6">6.6/</a></td><td class="size">-</td><td class="date">2019-Oct-23 16:54</td></tr>
            <tr><td class="link"><a href="6.7/" title="6.7">6.7/</a></td><td class="size">-</td><td class="date">2020-May-18 14:55</td></tr>
            <tr><td class="link"><a href="6.8/" title="6.8">6.8/</a></td><td class="size">-</td><td class="date">2021-Feb-08 01:58</td></tr>
            <tr><td class="link"><a href="6.9/" title="6.9">6.9/</a></td><td class="size">-</td><td class="date">2021-Apr-27 13:54</td></tr>
            <tr><td class="link"><a href="7.0/" title="7.0">7.0/</a></td><td class="size">-</td><td class="date">2021-Oct-15 02:55</td></tr>
            <tr><td class="link"><a href="7.1/" title="7.1">7.1/</a></td><td class="size">-</td><td class="date">2022-Apr-21 13:27</td></tr>
            <tr><td class="link"><a href="7.2/" title="7.2">7.2/</a></td><td class="size">-</td><td class="date">2022-Oct-18 06:11</td></tr>
            <tr><td class="link"><a href="7.3/" title="7.3">7.3/</a></td><td class="size">-</td><td class="date">2023-Apr-08 01:02</td></tr>
            <tr><td class="link"><a href="7.4/" title="7.4">7.4/</a></td><td class="size">-</td><td class="date">2023-Oct-16 15:44</td></tr>
            <tr><td class="link"><a href="7.5/" title="7.5">7.5/</a></td><td class="size">-</td><td class="date">2024-Apr-05 11:59</td></tr>
            <tr><td class="link"><a href="7.6/" title="7.6">7.6/</a></td><td class="size">-</td><td class="date">2024-Oct-08 17:17</td></tr>
            <tr><td class="link"><a href="7.7/" title="7.7">7.7/</a></td><td class="size">-</td><td class="date">2025-Apr-27 17:58</td></tr>
            <tr><td class="link"><a href="Changelogs/" title="Changelogs">Changelogs/</a></td><td class="size">-</td><td class="date">2025-May-16 07:33</td></tr>
            <tr><td class="link"><a href="LibreSSL/" title="LibreSSL">LibreSSL/</a></td><td class="size">-</td><td class="date">2025-Apr-30 07:35</td></tr>
            <tr><td class="link"><a href="OpenBGPD/" title="OpenBGPD">OpenBGPD/</a></td><td class="size">-</td><td class="date">2025-Feb-06 16:49</td></tr>
            <tr><td class="link"><a href="OpenIKED/" title="OpenIKED">OpenIKED/</a></td><td class="size">-</td><td class="date">2025-Apr-10 18:11</td></tr>
            <tr><td class="link"><a href="OpenNTPD/" title="OpenNTPD">OpenNTPD/</a></td><td class="size">-</td><td class="date">2020-Dec-09 14:56</td></tr>
            <tr><td class="link"><a href="OpenSSH/" title="OpenSSH">OpenSSH/</a></td><td class="size">-</td><td class="date">2025-Apr-09 07:54</td></tr>
            <tr><td class="link"><a href="doc/" title="doc">doc/</a></td><td class="size">-</td><td class="date">2013-Apr-28 15:57</td></tr>
            <tr><td class="link"><a href="patches/" title="patches">patches/</a></td><td class="size">-</td><td class="date">2025-May-16 14:07</td></tr>
            <tr><td class="link"><a href="rpki-client/" title="rpki-client">rpki-client/</a></td><td class="size">-</td><td class="date">2025-Apr-11 22:25</td></tr>
            <tr><td class="link"><a href="snapshots/" title="snapshots">snapshots/</a></td><td class="size">-</td><td class="date">2023-Mar-26 14:18</td></tr>
            <tr><td class="link"><a href="songs/" title="songs">songs/</a></td><td class="size">-</td><td class="date">2023-Apr-06 22:15</td></tr>
            <tr><td class="link"><a href="stable/" title="stable">stable/</a></td><td class="size">-</td><td class="date">2022-Jan-18 16:25</td></tr>
            <tr><td class="link"><a href="syspatch/" title="syspatch">syspatch/</a></td><td class="size">-</td><td class="date">2025-Mar-03 16:19</td></tr>
            <tr><td class="link"><a href="tools/" title="tools">tools/</a></td><td class="size">-</td><td class="date">2005-Jan-07 19:40</td></tr>
            <tr><td class="link"><a href="README" title="README">README</a></td><td class="size">               1249</td><td class="date">2021-May-25 20:15</td></tr>
            <tr><td class="link"><a href="ftplist" title="ftplist">ftplist</a></td><td class="size">               4836</td><td class="date">2025-May-16 14:13</td></tr>
            <tr><td class="link"><a href="timestamp" title="timestamp">timestamp</a></td><td class="size">                 11</td><td class="date">2025-May-16 14:13</td></tr>
            </tbody></table><div>This server can also be reached on the Tor network at</div>
            <div><a href="http://lysator7eknrfl47rlyxvgeamrv7ucefgrrlhk7rouv3sna25asetwid.onion/">lysator7eknrfl47rlyxvgeamrv7ucefgrrlhk7rouv3sna25asetwid.onion</a></div>
            <div>Information:</div>
            <div><a href="/datahanteringspolicy.txt">Data handling policy</a></div>
            <div>The mirror administration can be reached at ftp-master (at) lysator.liu.se</div>
        "##);
    });

    let httpdir = match HttpDirectory::new(url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    // The library fails to get this directory properly
    assert_eq!(httpdir.len(), 76);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "2.0/", 0, "2001-06-04 14:06");
    assert_entry(&entries[2], &EntryType::Directory, "2.1/", 0, "2001-06-04 15:20");
    assert_entry(&entries[56], &EntryType::Directory, "7.5/", 0, "2024-04-05 11:59");
    assert_entry(&entries[57], &EntryType::Directory, "7.6/", 0, "2024-10-08 17:17");
    assert_entry(&entries[58], &EntryType::Directory, "7.7/", 0, "2025-04-27 17:58");
    assert_entry(&entries[59], &EntryType::Directory, "Changelogs/", 0, "2025-05-16 07:33");
    assert_entry(&entries[60], &EntryType::Directory, "LibreSSL/", 0, "2025-04-30 07:35");
    assert_entry(&entries[61], &EntryType::Directory, "OpenBGPD/", 0, "2025-02-06 16:49");
    assert_entry(&entries[62], &EntryType::Directory, "OpenIKED/", 0, "2025-04-10 18:11");
    assert_entry(&entries[63], &EntryType::Directory, "OpenNTPD/", 0, "2020-12-09 14:56");
    assert_entry(&entries[64], &EntryType::Directory, "OpenSSH/", 0, "2025-04-09 07:54");
    assert_entry(&entries[65], &EntryType::Directory, "doc/", 0, "2013-04-28 15:57");
    assert_entry(&entries[66], &EntryType::Directory, "patches/", 0, "2025-05-16 14:07");
    assert_entry(&entries[67], &EntryType::Directory, "rpki-client/", 0, "2025-04-11 22:25");
    assert_entry(&entries[68], &EntryType::Directory, "snapshots/", 0, "2023-03-26 14:18");
    assert_entry(&entries[69], &EntryType::Directory, "songs/", 0, "2023-04-06 22:15");
    assert_entry(&entries[70], &EntryType::Directory, "stable/", 0, "2022-01-18 16:25");
    assert_entry(&entries[71], &EntryType::Directory, "syspatch/", 0, "2025-03-03 16:19");
    assert_entry(&entries[72], &EntryType::Directory, "tools/", 0, "2005-01-07 19:40");
    assert_entry(&entries[73], &EntryType::File, "README", 1_249, "2021-05-25 20:15");
    assert_entry(&entries[74], &EntryType::File, "ftplist", 4_836, "2025-05-16 14:13");
    assert_entry(&entries[75], &EntryType::File, "timestamp", 11, "2025-05-16 14:13");

    let httpdir = httpdir.sort_by_date(Sorting::Ascending);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "2.5/", 0, "2000-07-08 02:57");
    assert_entry(&entries[2], &EntryType::Directory, "2.0/", 0, "2001-06-04 14:06");
    assert_entry(&entries[3], &EntryType::Directory, "2.1/", 0, "2001-06-04 15:20");
    assert_entry(&entries[73], &EntryType::Directory, "patches/", 0, "2025-05-16 14:07");
    assert_entry(&entries[74], &EntryType::File, "ftplist", 4_836, "2025-05-16 14:13");
    assert_entry(&entries[75], &EntryType::File, "timestamp", 11, "2025-05-16 14:13");

    let httpdir = httpdir.sort_by_date(Sorting::Descending);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "ftplist", 4_836, "2025-05-16 14:13");
    assert_entry(&entries[2], &EntryType::File, "timestamp", 11, "2025-05-16 14:13");
    assert_entry(&entries[3], &EntryType::Directory, "patches/", 0, "2025-05-16 14:07");
    assert_entry(&entries[73], &EntryType::Directory, "2.1/", 0, "2001-06-04 15:20");
    assert_entry(&entries[74], &EntryType::Directory, "2.0/", 0, "2001-06-04 14:06");
    assert_entry(&entries[75], &EntryType::Directory, "2.5/", 0, "2000-07-08 02:57");

    mock.assert();

    let dir = "tools/";

    let mock = server.mock(|when, then| {
        when.path("/old-bsd/tools/");
        then.status(200).body(r##"
            <!DOCTYPE html><html><head><meta http-equiv="content-type" content="text/html; charset=utf-8"><meta name="viewport" content="width=device-width"><style type="text/css">body,html {background:#fff;font-family:"Bitstream Vera Sans","Lucida Grande","Lucida Sans Unicode",Lucidux,Verdana,Lucida,sans-serif;}tr:nth-child(even) {background:#f4f4f4;}th,td {padding:0.1em 0.5em;}th {text-align:left;font-weight:bold;background:#eee;border-bottom:1px solid #aaa;}#list {border:1px solid #aaa;width:100%;}a {color:#a33;}a:hover {color:#e33;}</style>

            <title>Index of /pub/OpenBSD/tools/</title>
            </head><body><h1>Index of /pub/OpenBSD/tools/</h1>
            <table id="list"><thead><tr><th style="width:55%"><a href="?C=N&amp;O=A">File Name</a>&nbsp;<a href="?C=N&amp;O=D">&nbsp;&darr;&nbsp;</a></th><th style="width:20%"><a href="?C=S&amp;O=A">File Size</a>&nbsp;<a href="?C=S&amp;O=D">&nbsp;&darr;&nbsp;</a></th><th style="width:25%"><a href="?C=M&amp;O=A">Date</a>&nbsp;<a href="?C=M&amp;O=D">&nbsp;&darr;&nbsp;</a></th></tr></thead>
            <tbody><tr><td class="link"><a href="../">Parent directory/</a></td><td class="size">-</td><td class="date">-</td></tr>
            <tr><td class="link"><a href="zenicb.el" title="zenicb.el">zenicb.el</a></td><td class="size">              26902</td><td class="date">1996-Nov-04 07:00</td></tr>
            </tbody></table><div>This server can also be reached on the Tor network at</div>
            <div><a href="http://lysator7eknrfl47rlyxvgeamrv7ucefgrrlhk7rouv3sna25asetwid.onion/">lysator7eknrfl47rlyxvgeamrv7ucefgrrlhk7rouv3sna25asetwid.onion</a></div>
            <div>Information:</div>
            <div><a href="/datahanteringspolicy.txt">Data handling policy</a></div>
            <div>The mirror administration can be reached at ftp-master (at) lysator.liu.se</div>
            "##);
    });

    let httpdir = match httpdir.cd(dir).await {
        Ok(httpdir) => httpdir,
        Err(_) => panic!("This test should return Ok()"),
    };

    assert_eq!(httpdir.len(), 2);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "zenicb.el", 26_902, "1996-11-04 07:00");

    mock.assert();
}

#[tokio::test]
async fn test_pre_img_example() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(r##"
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
            </div></div></body></html>"##);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.len(), 70);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "/noble/", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "MD5SUMS", 5_120, "2025-05-01 16:23");
    assert_entry(&entries[2], &EntryType::File, "MD5SUMS.gpg", 833, "2025-05-01 16:23");
    assert_entry(&entries[3], &EntryType::File, "SHA256SUMS", 7_168, "2025-05-01 16:23");
    assert_entry(&entries[4], &EntryType::File, "SHA256SUMS.gpg", 833, "2025-05-01 16:23");

    assert_entry(&entries[69], &EntryType::Directory, "unpacked/", 0, "2025-05-01 16:23");

    let files = httpdir.files();
    assert_eq!(files.len(), 68);

    // unwrap is is fine as we know that "b" regex is
    // a valid regex
    let filtered = files.filter_by_name("amd64.img").unwrap();
    let entries = filtered.entries();

    assert_eq!(filtered.len(), 1);

    assert_entry(&entries[0], &EntryType::File, "noble-server-cloudimg-amd64.img", 612_368_384, "2025-04-30 13:11");

    mock.assert();
}

#[tokio::test]
async fn test_debian_archive_trafficmanager_net() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(r##"
            <!DOCTYPE html>
            <html>
	<head>
		<title>debian</title>
		<link rel="canonical" href="/debian/"  />
		<meta charset="utf-8">
		<meta name="color-scheme" content="light dark">
		<meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style nonce="1f96551d-4fda-4b07-9f80-f0a947c1a2ad">
            * { padding: 0; margin: 0; box-sizing: border-box; }

            body {
	font-family: Inter, system-ui, sans-serif;
	font-size: 16px;
	text-rendering: optimizespeed;
	background-color: #f3f6f7;
	min-height: 100vh;
            }

            img,
            svg {
	vertical-align: middle;
	z-index: 1;
            }

            img {
	max-width: 100%;
	max-height: 100%;
	border-radius: 5px;
            }

            td img {
	max-width: 1.5em;
	max-height: 2em;
	object-fit: cover;
            }

            body,
            a,
            svg,
            .layout.current,
            .layout.current svg,
            .go-up {
	color: #333;
	text-decoration: none;
            }

            #layout-list, #layout-grid {
	cursor: pointer;
            }

            .wrapper {
	max-width: 1200px;
	margin-left: auto;
	margin-right: auto;
            }

            header,
            .meta {
	padding-left: 5%;
	padding-right: 5%;
            }

            td a {
	color: #006ed3;
	text-decoration: none;
            }

            td a:hover {
	color: #0095e4;
            }

            td a:visited {
	color: #800080;
            }

            td a:visited:hover {
	color: #b900b9;
            }

            th:first-child,
            td:first-child {
	width: 5%;
            }

            th:last-child,
            td:last-child {
	width: 5%;
            }

            .size,
            .timestamp {
	font-size: 14px;
            }

            .grid .size {
	font-size: 12px;
	margin-top: .5em;
	color: #496a84;
            }

            header {
	padding-top: 15px;
	padding-bottom: 15px;
	box-shadow: 0px 0px 20px 0px rgb(0 0 0 / 10%);
            }

            .breadcrumbs {
	text-transform: uppercase;
	font-size: 10px;
	letter-spacing: 1px;
	color: #939393;
	margin-bottom: 5px;
	padding-left: 3px;
            }

            h1 {
	font-size: 20px;
	font-family: Poppins, system-ui, sans-serif;
	font-weight: normal;
	white-space: nowrap;
	overflow-x: hidden;
	text-overflow: ellipsis;
	color: #c5c5c5;
            }

            h1 a,
            th a {
	color: #000;
            }

            h1 a {
	padding: 0 3px;
	margin: 0 1px;
            }

            h1 a:hover {
	background: #ffffc4;
            }

            h1 a:first-child {
	margin: 0;
            }

            header,
            main {
	background-color: white;
            }

            main {
	margin: 3em auto 0;
	border-radius: 5px;
	box-shadow: 0 2px 5px 1px rgb(0 0 0 / 5%);
            }

            .meta {
	display: flex;
	gap: 1em;
	font-size: 14px;
	border-bottom: 1px solid #e5e9ea;
	padding-top: 1em;
	padding-bottom: 1em;
            }

            #summary {
	display: flex;
	gap: 1em;
	align-items: center;
	margin-right: auto;
            }

            .filter-container {
	position: relative;
	display: inline-block;
	margin-left: 1em;
            }

            #search-icon {
	color: #777;
	position: absolute;
	height: 1em;
	top: .6em;
	left: .5em;
            }

            #filter {
	padding: .5em 1em .5em 2.5em;
	border: none;
	border: 1px solid #CCC;
	border-radius: 5px;
	font-family: inherit;
	position: relative;
	z-index: 2;
	background: none;
            }

            .layout,
            .layout svg {
	color: #9a9a9a;
            }

            table {
	width: 100%;
	border-collapse: collapse;
            }

            tbody tr,
            tbody tr a,
            .entry a {
	transition: all .15s;
            }

            tbody tr:hover,
            .grid .entry a:hover {
	background-color: #f4f9fd;
            }

            th,
            td {
	text-align: left;
            }

            th {
	position: sticky;
	top: 0;
	background: white;
	white-space: nowrap;
	z-index: 2;
	text-transform: uppercase;
	font-size: 14px;
	letter-spacing: 1px;
	padding: .75em 0;
            }

            td {
	white-space: nowrap;
            }

            td:nth-child(2) {
	width: 75%;
            }

            td:nth-child(2) a {
	padding: 1em 0;
	display: block;
            }

            td:nth-child(3),
            th:nth-child(3) {
	padding: 0 20px 0 20px;
	min-width: 150px;
            }

            td .go-up {
	text-transform: uppercase;
	font-size: 12px;
	font-weight: bold;
            }

            .name,
            .go-up {
	word-break: break-all;
	overflow-wrap: break-word;
	white-space: pre-wrap;
            }

            .listing .icon-tabler {
	color: #454545;
            }

            .listing .icon-tabler-folder-filled {
	color: #ffb900 !important;
            }

            .sizebar {
	position: relative;
	padding: 0.25rem 0.5rem;
	display: flex;
            }

            .sizebar-bar {
	background-color: #dbeeff;
	position: absolute;
	top: 0;
	right: 0;
	bottom: 0;
	left: 0;
	z-index: 0;
	height: 100%;
	pointer-events: none;
            }

            .sizebar-text {
	position: relative;
	z-index: 1;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
            }

            .grid {
	display: grid;
	grid-template-columns: repeat(auto-fill, minmax(16em, 1fr));
	gap: 2px;
            }

            .grid .entry {
	position: relative;
	width: 100%;
            }

            .grid .entry a {
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	padding: 1.5em;
	height: 100%;
            }

            .grid .entry svg {
	width: 75px;
	height: 75px;
            }

            .grid .entry img {
	max-height: 200px;
	object-fit: cover;
            }

            .grid .entry .name {
	margin-top: 1em;
            }

            footer {
	padding: 40px 20px;
	font-size: 12px;
	text-align: center;
            }

            .caddy-logo {
	display: inline-block;
	height: 2.5em;
	margin: 0 auto;
            }

            @media (max-width: 600px) {
	.hideable {
		display: none;
	}

	td:nth-child(2) {
		width: auto;
	}

	th:nth-child(3),
	td:nth-child(3) {
		padding-right: 5%;
		text-align: right;
	}

	h1 {
		color: #000;
	}

	h1 a {
		margin: 0;
	}

	#filter {
		max-width: 100px;
	}

	.grid .entry {
		max-width: initial;
	}
            }


            @media (prefers-color-scheme: dark) {
	html {
		background: black; /* overscroll */
	}

	body {
		background: linear-gradient(180deg, rgb(34 50 66) 0%, rgb(26 31 38) 100%);
		background-attachment: fixed;
	}

	body,
	a,
	svg,
	.layout.current,
	.layout.current svg,
	.go-up {
		color: #ccc;
	}

	h1 a,
	th a {
		color: white;
	}

	h1 {
		color: white;
	}

	h1 a:hover {
		background: hsl(213deg 100% 73% / 20%);
	}

	header,
	main,
	.grid .entry {
		background-color: #101720;
	}

	tbody tr:hover,
	.grid .entry a:hover {
		background-color: #162030;
		color: #fff;
	}

	th {
		background-color: #18212c;
	}

	td a,
	.listing .icon-tabler {
		color: #abc8e3;
	}

	td a:hover,
	td a:hover .icon-tabler {
		color: white;
	}

	td a:visited {
		color: #cd53cd;
	}

	td a:visited:hover {
		color: #f676f6;
	}

	#search-icon {
		color: #7798c4;
	}

	#filter {
		color: #ffffff;
		border: 1px solid #29435c;
	}

	.meta {
		border-bottom: 1px solid #222e3b;
	}

	.sizebar-bar {
		background-color: #1f3549;
	}

	.grid .entry a {
		background-color: #080b0f;
	}

	#Wordmark path,
	#R path {
		fill: #ccc !important;
	}
	#R circle {
		stroke: #ccc !important;
	}
            }

            </style>
            </head>
            <body>
	<header>
		<div class="wrapper">
			<div class="breadcrumbs">Folder Path</div>
				<h1>
					<a href="../">/</a><a href="">debian</a>/
				</h1>
			</div>
		</header>
		<div class="wrapper">
			<main>
				<div class="meta">
					<div id="summary">
						<span class="meta-item">
							<b>7</b> directories
						</span>
						<span class="meta-item">
							<b>7</b> files
						</span>
						<span class="meta-item">
							<b>15 MiB</b> total
						</span>
					</div>
					<a id="layout-list" class='layoutcurrent'>
						<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-layout-list" width="16" height="16" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
							<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
							<path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"/>
							<path d="M4 14m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"/>
						</svg>
						List
					</a>
					<a id="layout-grid" class='layout'>
						<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-layout-grid" width="16" height="16" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
							<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
							<path d="M4 4m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"/>
							<path d="M14 4m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"/>
							<path d="M4 14m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"/>
							<path d="M14 14m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"/>
						</svg>
						Grid
					</a>
				</div>
				<div class='listing'>
				<table aria-describedby="summary">
					<thead>
					<tr>
						<th></th>
						<th>
							<a href="?sort=namedirfirst&order=desc" class="icon">
								<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-caret-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
									<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
									<path d="M18 14l-6 -6l-6 6h12"/>
								</svg>
							</a>
							<a href="?sort=name&order=asc">
								Name
							</a>

							<div class="filter-container">
								<svg id="search-icon" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-search" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
									<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
									<path d="M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0"/>
									<path d="M21 21l-6 -6"/>
								</svg>
								<input type="search" placeholder="Search" id="filter">
							</div>
						</th>
						<th>
							<a href="?sort=size&order=asc">
								Size
							</a>
						</th>
						<th class="hideable">
							<a href="?sort=time&order=asc">
								Modified
							</a>
						</th>
						<th class="hideable"></th>
					</tr>
					</thead>
					<tbody>
					<tr>
						<td></td>
						<td>
							<a href="..">
								<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-corner-left-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
									<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
									<path d="M18 18h-6a3 3 0 0 1 -3 -3v-10l-4 4m8 0l-4 -4"/>
								</svg>
								<span class="go-up">Up</span>
							</a>
						</td>
						<td></td>
						<td class="hideable"></td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./dists/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">dists/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-17T08:29:25Z">05/17/2025 08:29:25 AM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./doc/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">doc/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-31T13:54:45Z">05/31/2025 01:54:45 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./indices/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">indices/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-31T14:25:33Z">05/31/2025 02:25:33 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./pool/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">pool/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2022-10-05T17:09:12Z">10/05/2022 05:09:12 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./project/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">project/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2008-11-17T23:05:07Z">11/17/2008 11:05:07 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./tools/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">tools/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2012-10-10T16:29:08Z">10/10/2012 04:29:08 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./zzz-dists/">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M9 3a1 1 0 0 1 .608 .206l.1 .087l2.706 2.707h6.586a3 3 0 0 1 2.995 2.824l.005 .176v8a3 3 0 0 1 -2.824 2.995l-.176 .005h-14a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-11a3 3 0 0 1 2.824 -2.995l.176 -.005h4z" stroke-width="0" fill="currentColor"/>
		</svg>
								<span class="name">zzz-dists/</span>
							</a>
						</td>
						<td>&mdash;</td>
						<td class="timestamp hideable">
							<time datetime="2023-10-07T11:07:16Z">10/07/2023 11:07:16 AM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./extrafiles">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M14 3v4a1 1 0 0 0 1 1h4"/>
			<path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z"/>
		</svg>
								<span class="name">extrafiles</span>
							</a>
						</td>
						<td class="size" data-size="203786">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									199 KiB
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-31T14:26:12Z">05/31/2025 02:26:12 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./ls-lR.gz">

	<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file-zip" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
		<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
		<path d="M6 20.735a2 2 0 0 1 -1 -1.735v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-1"/>
		<path d="M11 17a2 2 0 0 1 2 2v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a2 2 0 0 1 2 -2z"/>
		<path d="M11 5l-1 0"/>
		<path d="M13 7l-1 0"/>
		<path d="M11 9l-1 0"/>
		<path d="M13 11l-1 0"/>
		<path d="M11 13l-1 0"/>
		<path d="M13 15l-1 0"/>
	</svg>
								<span class="name">ls-lR.gz</span>
							</a>
						</td>
						<td class="size" data-size="15777274">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									15 MiB
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-31T14:18:05Z">05/31/2025 02:18:05 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./README">

	<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-license" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
		<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
		<path d="M15 21h-9a3 3 0 0 1 -3 -3v-1h10v2a2 2 0 0 0 4 0v-14a2 2 0 1 1 2 2h-2m2 -4h-11a3 3 0 0 0 -3 3v11"/>
		<path d="M9 7l4 0"/>
		<path d="M9 11l4 0"/>
	</svg>
								<span class="name">README</span>
							</a>
						</td>
						<td class="size" data-size="1200">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									1.2 KiB
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-17T08:29:22Z">05/17/2025 08:29:22 AM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./README.CD-manufacture">

		<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
			<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
			<path d="M14 3v4a1 1 0 0 0 1 1h4"/>
			<path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z"/>
		</svg>
								<span class="name">README.CD-manufacture</span>
							</a>
						</td>
						<td class="size" data-size="1290">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									1.3 KiB
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2010-06-26T09:52:47Z">06/26/2010 09:52:47 AM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./README.html">

	<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file-type-html" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
		<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
		<path d="M14 3v4a1 1 0 0 0 1 1h4"/>
		<path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4"/>
		<path d="M2 21v-6"/>
		<path d="M5 15v6"/>
		<path d="M2 18h3"/>
		<path d="M20 15v6h2"/>
		<path d="M13 21v-6l2 3l2 -3v6"/>
		<path d="M7.5 15h3"/>
		<path d="M9 15v6"/>
	</svg>
								<span class="name">README.html</span>
							</a>
						</td>
						<td class="size" data-size="2917">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									2.8 KiB
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2025-05-17T08:29:25Z">05/17/2025 08:29:25 AM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./README.mirrors.html">

	<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file-type-html" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
		<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
		<path d="M14 3v4a1 1 0 0 0 1 1h4"/>
		<path d="M5 12v-7a2 2 0 0 1 2 -2h7l5 5v4"/>
		<path d="M2 21v-6"/>
		<path d="M5 15v6"/>
		<path d="M2 18h3"/>
		<path d="M20 15v6h2"/>
		<path d="M13 21v-6l2 3l2 -3v6"/>
		<path d="M7.5 15h3"/>
		<path d="M9 15v6"/>
	</svg>
								<span class="name">README.mirrors.html</span>
							</a>
						</td>
						<td class="size" data-size="291">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									291 B
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2017-03-04T20:08:01Z">03/04/2017 08:08:01 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					<tr class="file">
						<td></td>
						<td>
							<a href="./README.mirrors.txt">

	<svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file-text" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
		<path stroke="none" d="M0 0h24v24H0z" fill="none"/>
		<path d="M14 3v4a1 1 0 0 0 1 1h4"/>
		<path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z"/>
		<path d="M9 9l1 0"/>
		<path d="M9 13l6 0"/>
		<path d="M9 17l6 0"/>
	</svg>
								<span class="name">README.mirrors.txt</span>
							</a>
						</td>
						<td class="size" data-size="86">
							<div class="sizebar">
								<div class="sizebar-bar"></div>
								<div class="sizebar-text">
									86 B
								</div>
							</div>
						</td>
						<td class="timestamp hideable">
							<time datetime="2017-03-04T20:08:51Z">03/04/2017 08:08:51 PM +00:00</time>
						</td>
						<td class="hideable"></td>
					</tr>
					</tbody>
				</table>
			</div>
			</main>
		</div>
		<footer>
			Served with
			<a rel="noopener noreferrer" href="https://caddyserver.com">
				<svg class="caddy-logo" viewBox="0 0 379 114" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" xmlns:serif="http://www.serif.com/" fill-rule="evenodd" clip-rule="evenodd" stroke-linecap="round" stroke-linejoin="round">
					<g transform="matrix(1,0,0,1,-1982.99,-530.985)">
						<g transform="matrix(1.16548,0,0,1.10195,1823.12,393.466)">
							<g transform="matrix(1,0,0,1,0.233052,1.17986)">
								<g id="Icon" transform="matrix(0.858013,0,0,0.907485,-3224.99,-1435.83)">
									<g>
										<g transform="matrix(-0.191794,-0.715786,0.715786,-0.191794,4329.14,4673.64)">
											<path d="M3901.56,610.734C3893.53,610.261 3886.06,608.1 3879.2,604.877C3872.24,601.608 3866.04,597.093 3860.8,591.633C3858.71,589.457 3856.76,587.149 3854.97,584.709C3853.2,582.281 3851.57,579.733 3850.13,577.066C3845.89,569.224 3843.21,560.381 3842.89,550.868C3842.57,543.321 3843.64,536.055 3845.94,529.307C3848.37,522.203 3852.08,515.696 3856.83,510.049L3855.79,509.095C3850.39,514.54 3846.02,520.981 3842.9,528.125C3839.84,535.125 3838.03,542.781 3837.68,550.868C3837.34,561.391 3839.51,571.425 3843.79,580.306C3845.27,583.38 3847.03,586.304 3849.01,589.049C3851.01,591.806 3853.24,594.39 3855.69,596.742C3861.75,602.568 3869,607.19 3877.03,610.1C3884.66,612.867 3892.96,614.059 3901.56,613.552L3901.56,610.734Z" fill="rgb(0,144,221)"/>
										</g>
										<g transform="matrix(-0.191794,-0.715786,0.715786,-0.191794,4329.14,4673.64)">
											<path d="M3875.69,496.573C3879.62,494.538 3883.8,492.897 3888.2,491.786C3892.49,490.704 3896.96,490.124 3901.56,490.032C3903.82,490.13 3906.03,490.332 3908.21,490.688C3917.13,492.147 3925.19,495.814 3932.31,500.683C3936.13,503.294 3939.59,506.335 3942.81,509.619C3947.09,513.98 3950.89,518.816 3953.85,524.232C3958.2,532.197 3960.96,541.186 3961.32,550.868C3961.61,558.748 3960.46,566.345 3957.88,573.322C3956.09,578.169 3953.7,582.753 3950.66,586.838C3947.22,591.461 3942.96,595.427 3938.27,598.769C3933.66,602.055 3928.53,604.619 3923.09,606.478C3922.37,606.721 3921.6,606.805 3920.93,607.167C3920.42,607.448 3920.14,607.854 3919.69,608.224L3920.37,610.389C3920.98,610.432 3921.47,610.573 3922.07,610.474C3922.86,610.344 3923.55,609.883 3924.28,609.566C3931.99,606.216 3938.82,601.355 3944.57,595.428C3947.02,592.903 3949.25,590.174 3951.31,587.319C3953.59,584.168 3955.66,580.853 3957.43,577.348C3961.47,569.34 3964.01,560.422 3964.36,550.868C3964.74,540.511 3962.66,530.628 3958.48,521.868C3955.57,515.775 3951.72,510.163 3946.95,505.478C3943.37,501.962 3939.26,498.99 3934.84,496.562C3926.88,492.192 3917.87,489.76 3908.37,489.229C3906.12,489.104 3903.86,489.054 3901.56,489.154C3896.87,489.06 3892.3,489.519 3887.89,490.397C3883.3,491.309 3878.89,492.683 3874.71,494.525L3875.69,496.573Z" fill="rgb(0,144,221)"/>
										</g>
									</g>
									<g>
										<g transform="matrix(-3.37109,-0.514565,0.514565,-3.37109,4078.07,1806.88)">
											<path d="M22,12C22,10.903 21.097,10 20,10C19.421,10 18.897,10.251 18.53,10.649C18.202,11.006 18,11.481 18,12C18,13.097 18.903,14 20,14C21.097,14 22,13.097 22,12Z" fill="none" fill-rule="nonzero" stroke="rgb(0,144,221)" stroke-width="1.05px"/>
										</g>
										<g transform="matrix(-5.33921,-5.26159,-3.12106,-6.96393,4073.87,1861.55)">
											<path d="M10.315,5.333C10.315,5.333 9.748,5.921 9.03,6.673C7.768,7.995 6.054,9.805 6.054,9.805L6.237,9.86C6.237,9.86 8.045,8.077 9.36,6.771C10.107,6.028 10.689,5.444 10.689,5.444L10.315,5.333Z" fill="rgb(0,144,221)"/>
										</g>
									</g>
									<g id="Padlock" transform="matrix(3.11426,0,0,3.11426,3938.31,1737.25)">
										<g>
											<path d="M9.876,21L18.162,21C18.625,21 19,20.625 19,20.162L19,11.838C19,11.375 18.625,11 18.162,11L5.838,11C5.375,11 5,11.375 5,11.838L5,16.758" fill="none" stroke="rgb(34,182,56)" stroke-width="1.89px" stroke-linecap="butt" stroke-linejoin="miter"/>
											<path d="M8,11L8,7C8,4.806 9.806,3 12,3C14.194,3 16,4.806 16,7L16,11" fill="none" fill-rule="nonzero" stroke="rgb(34,182,56)" stroke-width="1.89px"/>
										</g>
									</g>
									<g>
										<g transform="matrix(5.30977,0.697415,-0.697415,5.30977,3852.72,1727.97)">
											<path d="M22,12C22,11.659 21.913,11.337 21.76,11.055C21.421,10.429 20.756,10 20,10C18.903,10 18,10.903 18,12C18,13.097 18.903,14 20,14C21.097,14 22,13.097 22,12Z" fill="none" fill-rule="nonzero" stroke="rgb(0,144,221)" stroke-width="0.98px"/>
										</g>
										<g transform="matrix(4.93114,2.49604,1.11018,5.44847,3921.41,1726.72)">
											<path d="M8.902,6.77C8.902,6.77 7.235,8.253 6.027,9.366C5.343,9.996 4.819,10.502 4.819,10.502L5.52,11.164C5.52,11.164 6.021,10.637 6.646,9.951C7.749,8.739 9.219,7.068 9.219,7.068L8.902,6.77Z" fill="rgb(0,144,221)"/>
										</g>
									</g>
								</g>
								<g id="Text">
									<g id="Wordmark" transform="matrix(1.32271,0,0,2.60848,-899.259,-791.691)">
										<g id="y" transform="matrix(0.50291,0,0,0.281607,905.533,304.987)">
											<path d="M192.152,286.875L202.629,268.64C187.804,270.106 183.397,265.779 180.143,263.391C176.888,261.004 174.362,257.99 172.563,254.347C170.765,250.705 169.866,246.691 169.866,242.305L169.866,208.107L183.21,208.107L183.21,242.213C183.21,245.188 183.896,247.822 185.268,250.116C186.64,252.41 188.465,254.197 190.743,255.475C193.022,256.754 195.501,257.393 198.182,257.393C200.894,257.393 203.393,256.75 205.68,255.463C207.966,254.177 209.799,252.391 211.178,250.105C212.558,247.818 213.248,245.188 213.248,242.213L213.248,208.107L226.545,208.107L226.545,242.305C226.545,246.707 225.378,258.46 218.079,268.64C215.735,271.909 207.835,286.875 207.835,286.875L192.152,286.875Z" fill="rgb(47,47,47)" fill-rule="nonzero"/>
										</g>
										<g id="add" transform="matrix(0.525075,0,0,0.281607,801.871,304.987)">
											<g transform="matrix(116.242,0,0,116.242,161.846,267.39)">
												<path d="M0.276,0.012C0.227,0.012 0.186,0 0.15,-0.024C0.115,-0.048 0.088,-0.08 0.069,-0.12C0.05,-0.161 0.04,-0.205 0.04,-0.254C0.04,-0.305 0.051,-0.35 0.072,-0.39C0.094,-0.431 0.125,-0.463 0.165,-0.487C0.205,-0.51 0.254,-0.522 0.31,-0.522C0.366,-0.522 0.413,-0.51 0.452,-0.486C0.491,-0.463 0.521,-0.431 0.542,-0.39C0.562,-0.35 0.573,-0.305 0.573,-0.256L0.573,-0L0.458,-0L0.458,-0.095L0.456,-0.095C0.446,-0.076 0.433,-0.058 0.417,-0.042C0.401,-0.026 0.381,-0.013 0.358,-0.003C0.335,0.007 0.307,0.012 0.276,0.012ZM0.307,-0.086C0.337,-0.086 0.363,-0.093 0.386,-0.108C0.408,-0.123 0.426,-0.144 0.438,-0.17C0.45,-0.195 0.456,-0.224 0.456,-0.256C0.456,-0.288 0.45,-0.317 0.438,-0.342C0.426,-0.367 0.409,-0.387 0.387,-0.402C0.365,-0.417 0.338,-0.424 0.308,-0.424C0.276,-0.424 0.249,-0.417 0.226,-0.402C0.204,-0.387 0.186,-0.366 0.174,-0.341C0.162,-0.315 0.156,-0.287 0.156,-0.255C0.156,-0.224 0.162,-0.195 0.174,-0.169C0.186,-0.144 0.203,-0.123 0.226,-0.108C0.248,-0.093 0.275,-0.086 0.307,-0.086Z" fill="rgb(47,47,47)" fill-rule="nonzero"/>
											</g>
											<g transform="matrix(116.242,0,0,116.242,226.592,267.39)">
												<path d="M0.306,0.012C0.265,0.012 0.229,0.006 0.196,-0.008C0.163,-0.021 0.135,-0.039 0.112,-0.064C0.089,-0.088 0.071,-0.117 0.059,-0.151C0.046,-0.185 0.04,-0.222 0.04,-0.263C0.04,-0.315 0.051,-0.36 0.072,-0.399C0.093,-0.437 0.122,-0.468 0.159,-0.489C0.196,-0.511 0.239,-0.522 0.287,-0.522C0.311,-0.522 0.333,-0.518 0.355,-0.511C0.377,-0.504 0.396,-0.493 0.413,-0.48C0.431,-0.466 0.445,-0.451 0.455,-0.433L0.456,-0.433L0.456,-0.73L0.571,-0.73L0.571,-0.261C0.571,-0.205 0.56,-0.156 0.537,-0.115C0.515,-0.074 0.484,-0.043 0.444,-0.021C0.405,0.001 0.358,0.012 0.306,0.012ZM0.306,-0.086C0.335,-0.086 0.361,-0.093 0.384,-0.107C0.406,-0.122 0.423,-0.141 0.436,-0.167C0.448,-0.192 0.455,-0.221 0.455,-0.255C0.455,-0.288 0.448,-0.317 0.436,-0.343C0.423,-0.368 0.406,-0.388 0.383,-0.402C0.361,-0.417 0.335,-0.424 0.305,-0.424C0.276,-0.424 0.251,-0.417 0.228,-0.402C0.206,-0.387 0.188,-0.368 0.175,-0.342C0.163,-0.317 0.156,-0.288 0.156,-0.255C0.156,-0.222 0.163,-0.193 0.175,-0.167C0.188,-0.142 0.206,-0.122 0.229,-0.108C0.251,-0.093 0.277,-0.086 0.306,-0.086Z" fill="rgb(47,47,47)" fill-rule="nonzero"/>
											</g>
											<g transform="matrix(116.242,0,0,116.242,290.293,267.39)">
												<path d="M0.306,0.012C0.265,0.012 0.229,0.006 0.196,-0.008C0.163,-0.021 0.135,-0.039 0.112,-0.064C0.089,-0.088 0.071,-0.117 0.059,-0.151C0.046,-0.185 0.04,-0.222 0.04,-0.263C0.04,-0.315 0.051,-0.36 0.072,-0.399C0.093,-0.437 0.122,-0.468 0.159,-0.489C0.196,-0.511 0.239,-0.522 0.287,-0.522C0.311,-0.522 0.333,-0.518 0.355,-0.511C0.377,-0.504 0.396,-0.493 0.413,-0.48C0.431,-0.466 0.445,-0.451 0.455,-0.433L0.456,-0.433L0.456,-0.73L0.571,-0.73L0.571,-0.261C0.571,-0.205 0.56,-0.156 0.537,-0.115C0.515,-0.074 0.484,-0.043 0.444,-0.021C0.405,0.001 0.358,0.012 0.306,0.012ZM0.306,-0.086C0.335,-0.086 0.361,-0.093 0.384,-0.107C0.406,-0.122 0.423,-0.141 0.436,-0.167C0.448,-0.192 0.455,-0.221 0.455,-0.255C0.455,-0.288 0.448,-0.317 0.436,-0.343C0.423,-0.368 0.406,-0.388 0.383,-0.402C0.361,-0.417 0.335,-0.424 0.305,-0.424C0.276,-0.424 0.251,-0.417 0.228,-0.402C0.206,-0.387 0.188,-0.368 0.175,-0.342C0.163,-0.317 0.156,-0.288 0.156,-0.255C0.156,-0.222 0.163,-0.193 0.175,-0.167C0.188,-0.142 0.206,-0.122 0.229,-0.108C0.251,-0.093 0.277,-0.086 0.306,-0.086Z" fill="rgb(47,47,47)" fill-rule="nonzero"/>
											</g>
										</g>
										<g id="c" transform="matrix(-0.0716462,0.31304,-0.583685,-0.0384251,1489.76,-444.051)">
											<path d="M2668.11,700.4C2666.79,703.699 2666.12,707.216 2666.12,710.766C2666.12,726.268 2678.71,738.854 2694.21,738.854C2709.71,738.854 2722.3,726.268 2722.3,710.766C2722.3,704.111 2719.93,697.672 2715.63,692.597L2707.63,699.378C2710.33,702.559 2711.57,706.602 2711.81,710.766C2712.2,717.38 2706.61,724.52 2697.27,726.637C2683.9,728.581 2676.61,720.482 2676.61,710.766C2676.61,708.541 2677.03,706.336 2677.85,704.269L2668.11,700.4Z" fill="rgb(46,46,46)"/>
										</g>
									</g>
									<g id="R" transform="matrix(0.426446,0,0,0.451034,-1192.44,-722.167)">
										<g transform="matrix(1,0,0,1,-0.10786,0.450801)">
											<g transform="matrix(12.1247,0,0,12.1247,3862.61,1929.9)">
												<path d="M0.073,-0L0.073,-0.7L0.383,-0.7C0.428,-0.7 0.469,-0.69 0.506,-0.67C0.543,-0.651 0.572,-0.623 0.594,-0.588C0.616,-0.553 0.627,-0.512 0.627,-0.465C0.627,-0.418 0.615,-0.377 0.592,-0.342C0.569,-0.306 0.539,-0.279 0.501,-0.259L0.57,-0.128C0.574,-0.12 0.579,-0.115 0.584,-0.111C0.59,-0.107 0.596,-0.106 0.605,-0.106L0.664,-0.106L0.664,-0L0.587,-0C0.56,-0 0.535,-0.007 0.514,-0.02C0.493,-0.034 0.476,-0.052 0.463,-0.075L0.381,-0.232C0.375,-0.231 0.368,-0.231 0.361,-0.231C0.354,-0.231 0.347,-0.231 0.34,-0.231L0.192,-0.231L0.192,-0L0.073,-0ZM0.192,-0.336L0.368,-0.336C0.394,-0.336 0.417,-0.341 0.438,-0.351C0.459,-0.361 0.476,-0.376 0.489,-0.396C0.501,-0.415 0.507,-0.438 0.507,-0.465C0.507,-0.492 0.501,-0.516 0.488,-0.535C0.475,-0.554 0.459,-0.569 0.438,-0.579C0.417,-0.59 0.394,-0.595 0.369,-0.595L0.192,-0.595L0.192,-0.336Z" fill="rgb(46,46,46)" fill-rule="nonzero"/>
											</g>
										</g>
										<g transform="matrix(1,0,0,1,0.278569,0.101881)">
											<circle cx="3866.43" cy="1926.14" r="8.923" fill="none" stroke="rgb(46,46,46)" stroke-width="2px" stroke-linecap="butt" stroke-linejoin="miter"/>
										</g>
									</g>
								</g>
							</g>
						</g>
					</g>
				</svg>
			</a>
		</footer>

		<script nonce="1f96551d-4fda-4b07-9f80-f0a947c1a2ad">
			const filterEl = document.getElementById('filter');
			filterEl?.focus({ preventScroll: true });

			function initPage() {
// populate and evaluate filter
				if (!filterEl?.value) {
					const filterParam = new URL(window.location.href).searchParams.get('filter');
					if (filterParam) {
						filterEl.value = filterParam;
					}
				}
				filter();

// fill in size bars
				let largest = 0;
				document.querySelectorAll('.size').forEach(el => {
					largest = Math.max(largest, Number(el.dataset.size));
				});
				document.querySelectorAll('.size').forEach(el => {
					const size = Number(el.dataset.size);
					const sizebar = el.querySelector('.sizebar-bar');
					if (sizebar) {
						sizebar.style.width = `${size/largest * 100}%`;
					}
				});
			}

			function filter() {
				if (!filterEl) return;
				const q = filterEl.value.trim().toLowerCase();
				document.querySelectorAll('tr.file').forEach(function(el) {
					if (!q) {
						el.style.display = '';
						return;
					}
					const nameEl = el.querySelector('.name');
					const nameVal = nameEl.textContent.trim().toLowerCase();
					if (nameVal.indexOf(q) !== -1) {
						el.style.display = '';
					} else {
						el.style.display = 'none';
					}
				});
			}

			const filterElem = document.getElementById("filter");
			if (filterElem) {
				filterElem.addEventListener("keyup", filter);
			}

			document.getElementById("layout-list").addEventListener("click", function() {
				queryParam('layout', '');
			});
			document.getElementById("layout-grid").addEventListener("click", function() {
				queryParam('layout', 'grid');
			});

			window.addEventListener("load", initPage);

			function queryParam(k, v) {
				const qs = new URLSearchParams(window.location.search);
				if (!v) {
					qs.delete(k);
				} else {
					qs.set(k, v);
				}
				const qsStr = qs.toString();
				if (qsStr) {
					window.location.search = qsStr;
				} else {
					window.location = window.location.pathname;
				}
			}

			function localizeDatetime(e, index, ar) {
				if (e.textContent === undefined) {
					return;
				}
				var d = new Date(e.getAttribute('datetime'));
				if (isNaN(d)) {
					d = new Date(e.textContent);
					if (isNaN(d)) {
						return;
					}
				}
				e.textContent = d.toLocaleString();
			}
			var timeList = Array.prototype.slice.call(document.getElementsByTagName("time"));
			timeList.forEach(localizeDatetime);
		</script>
	</body>
            </html>
            "##);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.len(), 15);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "..", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "dists/", 0, "2025-05-17 08:29");
    assert_entry(&entries[2], &EntryType::Directory, "doc/", 0, "2025-05-31 13:54");
    assert_entry(&entries[3], &EntryType::Directory, "indices/", 0, "2025-05-31 14:25");
    assert_entry(&entries[4], &EntryType::Directory, "pool/", 0, "2022-10-05 17:09");
    assert_entry(&entries[5], &EntryType::Directory, "project/", 0, "2008-11-17 23:05");
    assert_entry(&entries[6], &EntryType::Directory, "tools/", 0, "2012-10-10 16:29");
    assert_entry(&entries[7], &EntryType::Directory, "zzz-dists/", 0, "2023-10-07 11:07");
    assert_entry(&entries[8], &EntryType::File, "extrafiles", 203776, "2025-05-31 14:26");
    assert_entry(&entries[9], &EntryType::File, "ls-lR.gz", 15728640, "2025-05-31 14:18");
    assert_entry(&entries[10], &EntryType::File, "README", 1228, "2025-05-17 08:29");
    assert_entry(&entries[11], &EntryType::File, "README.CD-manufacture", 1331, "2010-06-26 09:52");
    assert_entry(&entries[12], &EntryType::File, "README.html", 2867, "2025-05-17 08:29");
    assert_entry(&entries[13], &EntryType::File, "README.mirrors.html", 291, "2017-03-04 20:08");
    assert_entry(&entries[14], &EntryType::File, "README.mirrors.txt", 86, "2017-03-04 20:08");

    mock.assert();
}

#[tokio::test]
async fn test_debian_h5ai() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(r##"
            <!DOCTYPE html>
            <html class="no-js" lang="en">
                <head>
                    <meta charset="utf-8">
                    <meta http-equiv="x-ua-compatible" content="ie=edge">
                    <title>index - powered by h5ai v0.29.2 (https://larsjung.de/h5ai/)</title>
                    <meta name="description" content="index - powered by h5ai v0.29.2 (https://larsjung.de/h5ai/)">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                    <link rel="shortcut icon" href="/_h5ai/public/images/favicon/favicon-16-32.ico">
                    <link rel="apple-touch-icon-precomposed" type="image/png" href="/_h5ai/public/images/favicon/favicon-152.png">
                    <link rel="stylesheet" href="/_h5ai/public/css/styles.css">
                    <script src="/_h5ai/public/js/scripts.js" data-module="index"></script>
                    <link rel="stylesheet" href="//fonts.googleapis.com/css?family=Ubuntu:300,400,700%7CUbuntu+Mono:400,700" class="x-head">
                    <style class="x-head">#root,input,select{font-family:"Ubuntu","Roboto","Helvetica","Arial","sans-serif"!important}pre,code{font-family:"Ubuntu Mono","Monaco","Lucida Sans Typewriter","monospace"!important}</style>
                </head>
                <body class="index" id="root">
                    <div id="fallback-hints">
                        <span class="noJsMsg">Works best with JavaScript enabled!</span>
                        <span class="noBrowserMsg">Works best in <a href="http://browsehappy.com">modern browsers</a>!</span>
                        <span class="backlink"><a href="https://larsjung.de/h5ai/" title="h5ai v0.29.2 - Modern HTTP web server index.">powered by h5ai</a></span>
                    </div>
                    <div id="fallback">
                        <table>
                            <tr>
                                <th class="fb-i"></th>
                                <th class="fb-n">
                                    <span>Name</span>
                                </th>
                                <th class="fb-d">
                                    <span>Last modified</span>
                                </th>
                                <th class="fb-s">
                                    <span>Size</span>
                                </th>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder-parent.png" alt="folder-parent"/></td>
                                <td class="fb-n"><a href="..">Parent Directory</a></td>
                                <td class="fb-d"></td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/dists/">dists</a></td>
                                <td class="fb-d">2025-09-06 10:15</td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/doc/">doc</a></td>
                                <td class="fb-d">2025-10-19 14:12</td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/indices/">indices</a></td>
                                <td class="fb-d">2025-10-19 14:59</td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/pool/">pool</a></td>
                                <td class="fb-d">2022-10-05 17:09</td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/project/">project</a></td>
                                <td class="fb-d">2008-11-17 23:05</td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/tools/">tools</a></td>
                                <td class="fb-d">2012-10-10 16:29</td>
                                <td class="fb-s"></td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/folder.png" alt="folder"/></td>
                                <td class="fb-n"><a href="/debian/zzz-dists/">zzz-dists</a></td>
                                <td class="fb-d">2025-08-09 12:48</td>
                                <td class="fb-s">
                                </td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/file.png" alt="file"/></td>
                                <td class="fb-n"><a href="/debian/ls-lR.gz">ls-lR.gz</a></td>
                                <td class="fb-d">2025-10-19 14:49</td>
                                <td class="fb-s">13971 KB</td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/file.png" alt="file"/></td>
                                <td class="fb-n"><a href="/debian/README">README</a></td>
                                <td class="fb-d">2025-09-06 10:15</td>
                                <td class="fb-s">1 KB</td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/file.png" alt="file"/></td>
                                <td class="fb-n"><a href="/debian/README.CD-manufacture">README.CD-manufacture</a></td>
                                <td class="fb-d">2010-06-26 09:52</td>
                                <td class="fb-s">1 KB</td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/file.png" alt="file"/></td>
                                <td class="fb-n"><a href="/debian/README.html">README.html</a></td>
                                <td class="fb-d">2025-09-06 10:15</td>
                                <td class="fb-s">2 KB</td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/file.png" alt="file"/></td>
                                <td class="fb-n"><a href="/debian/README.mirrors.html">README.mirrors.html</a></td>
                                <td class="fb-d">2017-03-04 20:08</td>
                                <td class="fb-s">0 KB</td>
                            </tr>
                            <tr>
                                <td class="fb-i"><img src="/_h5ai/public/images/fallback/file.png" alt="file"/></td>
                                <td class="fb-n"><a href="/debian/README.mirrors.txt">README.mirrors.txt</a></td>
                                <td class="fb-d">2017-03-04 20:08</td>
                                <td class="fb-s">0 KB</td>
                            </tr>
                        </table>
                    </div>
                </body>
            </html><!-- h5ai v0.29.2 - https://larsjung.de/h5ai/ -->
            "##);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.len(), 14);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "..", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "dists", 0, "2025-09-06 10:15");
    assert_entry(&entries[2], &EntryType::Directory, "doc", 0, "2025-10-19 14:12");
    assert_entry(&entries[3], &EntryType::Directory, "indices", 0, "2025-10-19 14:59");
    assert_entry(&entries[4], &EntryType::Directory, "pool", 0, "2022-10-05 17:09");
    assert_entry(&entries[5], &EntryType::Directory, "project", 0, "2008-11-17 23:05");
    assert_entry(&entries[6], &EntryType::Directory, "tools", 0, "2012-10-10 16:29");
    assert_entry(&entries[7], &EntryType::Directory, "zzz-dists", 0, "2025-08-09 12:48");
    assert_entry(&entries[8], &EntryType::File, "ls-lR.gz", 14306304, "2025-10-19 14:49");
    assert_entry(&entries[9], &EntryType::File, "README", 1024, "2025-09-06 10:15");
    assert_entry(&entries[10], &EntryType::File, "README.CD-manufacture", 1024, "2010-06-26 09:52");
    assert_entry(&entries[11], &EntryType::File, "README.html", 2048, "2025-09-06 10:15");
    assert_entry(&entries[12], &EntryType::File, "README.mirrors.html", 0, "2017-03-04 20:08");
    assert_entry(&entries[13], &EntryType::File, "README.mirrors.txt", 0, "2017-03-04 20:08");

    mock.assert();
}

#[tokio::test]
async fn test_debian_ul() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(
            r##"
            <!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final//EN">
            <html>
             <head>
              <title>Index of /debian</title>
             </head>
             <body>
            <h1>Index of /debian</h1>
            <ul><li><a href="/"> Parent Directory</a></li>
            <li><a href="README"> README</a></li>
            <li><a href="README.CD-manufacture"> README.CD-manufacture</a></li>
            <li><a href="README.html"> README.html</a></li>
            <li><a href="README.mirrors.html"> README.mirrors.html</a></li>
            <li><a href="README.mirrors.txt"> README.mirrors.txt</a></li>
            <li><a href="dists/"> dists/</a></li>
            <li><a href="doc/"> doc/</a></li>
            <li><a href="extrafiles"> extrafiles</a></li>
            <li><a href="indices/"> indices/</a></li>
            <li><a href="ls-lR.gz"> ls-lR.gz</a></li>
            <li><a href="pool/"> pool/</a></li>
            <li><a href="project/"> project/</a></li>
            <li><a href="tools/"> tools/</a></li>
            <li><a href="zzz-dists/"> zzz-dists/</a></li>
            </ul>
            </body></html>
            "##,
        );
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.len(), 15);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "Parent Directory", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "README", 0, "0000-00-00 00:00");
    assert_entry(&entries[2], &EntryType::File, "README.CD-manufacture", 0, "0000-00-00 00:00");
    assert_entry(&entries[3], &EntryType::File, "README.html", 0, "0000-00-00 00:00");
    assert_entry(&entries[4], &EntryType::File, "README.mirrors.html", 0, "0000-00-00 00:00");
    assert_entry(&entries[5], &EntryType::File, "README.mirrors.txt", 0, "0000-00-00 00:00");
    assert_entry(&entries[6], &EntryType::Directory, "dists/", 0, "0000-00-00 00:00");
    assert_entry(&entries[7], &EntryType::Directory, "doc/", 0, "0000-00-00 00:00");
    assert_entry(&entries[8], &EntryType::File, "extrafiles", 0, "0000-00-00 00:00");
    assert_entry(&entries[9], &EntryType::Directory, "indices/", 0, "0000-00-00 00:00");
    assert_entry(&entries[10], &EntryType::File, "ls-lR.gz", 0, "0000-00-00 00:00");
    assert_entry(&entries[11], &EntryType::Directory, "pool/", 0, "0000-00-00 00:00");
    assert_entry(&entries[12], &EntryType::Directory, "project/", 0, "0000-00-00 00:00");
    assert_entry(&entries[13], &EntryType::Directory, "tools/", 0, "0000-00-00 00:00");
    assert_entry(&entries[14], &EntryType::Directory, "zzz-dists/", 0, "0000-00-00 00:00");

    mock.assert();
}

#[tokio::test]
async fn test_debian_snt() {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(r##"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta name="generator" content="SNT index generator"/>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
                <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
                <title>Studenten Net Twente - Index of /debian/</title>
                <link rel="shortcut icon" href="https://static.utwente.io/favicon.ico" type="image/x-icon"/>
                <link rel="stylesheet" href="https://static.utwente.io/ibm-plex/css/ibm-plex.min.css"/>
                <link rel="stylesheet" href="https://static.utwente.io/fontawesome/css/solid.css">
                <link rel="stylesheet" href="https://static.utwente.io/fontawesome/css/fontawesome.css">
                <link rel="stylesheet" href="https://static.utwente.io/sntstyle.css"/>
                <style>
                    #content {
                        max-width: 48em;
                    }
                    header #logo {
                        background-image: url("https://static.utwente.io/img/header3.jpg");
                    }
                </style>
            </head>
            <body>
            <header>
                <div id="name">
                    <a href="https://snt.utwente.nl">Studenten Net Twente</a>
                    <span>Mirror service</span>
                </div>
                <div id="logo">
                    <a href="/">
                        <img alt="&nbsp;" src="https://static.utwente.io/snt_logo.svg"/>
                    </a>
                </div>
            </header>
            <div id="content">

                    <div id="title">
                        <h1>Index of /debian/</h1>
                    </div>
                    <main>
                        <nav>
                            <h3>Directories</h3>
                            <ul>

                                    <li>
                                        <a href="../" class="dirup">
                                            <i class="fas fa-level-up-alt"></i>
                                            <i>Parent directory</i>
                                        </a>
                                    </li>
                                <li>
                                        <a href="dists/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            dists
                                        </a>
                                    </li>
                                <li>
                                        <a href="doc/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            doc
                                        </a>
                                    </li>
                                <li>
                                        <a href="indices/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            indices
                                        </a>
                                    </li>
                                <li>
                                        <a href="pool/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            pool
                                        </a>
                                    </li>
                                <li>
                                        <a href="project/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            project
                                        </a>
                                    </li>
                                <li>
                                        <a href="tools/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            tools
                                        </a>
                                    </li>
                                <li>
                                        <a href="zzz-dists/" class="dir">
                                            <i class="fas fa-folder"></i>
                                            zzz-dists
                                        </a>
                                    </li>

                            </ul>
                        </nav>
                        <article>
                            <div id="files">

                                    <table class="listing">
                                        <thead>
                                        <tr class="header">
                                            <th class="name">Filename</th>
                                            <th class="time">Modification time</th>
                                            <th class="size">Size</th>
                                        </tr>
                                        </thead>
                                        <tbody>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="README">
                                                        <i class="fas fa-file"></i>
                                                        README
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2025-09-06T12:15:01&#43;02:00">2025-09-06 12:15 CEST</time>
                                                </td>
                                                <td class="size" title="1201 bytes">1201 B</td>
                                            </tr>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="README.CD-manufacture">
                                                        <i class="fas fa-file"></i>
                                                        README.CD-manufacture
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2010-06-26T11:52:47&#43;02:00">2010-06-26 11:52 CEST</time>
                                                </td>
                                                <td class="size" title="1290 bytes">1290 B</td>
                                            </tr>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="README.html">
                                                        <i class="fas fa-file"></i>
                                                        README.html
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2025-09-06T12:15:01&#43;02:00">2025-09-06 12:15 CEST</time>
                                                </td>
                                                <td class="size" title="2919 bytes">2919 B</td>
                                            </tr>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="README.mirrors.html">
                                                        <i class="fas fa-file"></i>
                                                        README.mirrors.html
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2017-03-04T21:08:01&#43;01:00">2017-03-04 21:08 CET</time>
                                                </td>
                                                <td class="size" title="291 bytes">291 B</td>
                                            </tr>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="README.mirrors.txt">
                                                        <i class="fas fa-file"></i>
                                                        README.mirrors.txt
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2017-03-04T21:08:51&#43;01:00">2017-03-04 21:08 CET</time>
                                                </td>
                                                <td class="size" title="86 bytes">86 B</td>
                                            </tr>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="extrafiles">
                                                        <i class="fas fa-file"></i>
                                                        extrafiles
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2025-10-20T16:23:25&#43;02:00">2025-10-20 16:23 CEST</time>
                                                </td>
                                                <td class="size" title="211915 bytes">207 KiB</td>
                                            </tr>

                                            <tr>
                                                <td class="name file">
                                                    <a rel="nofollow" href="ls-lR.gz">
                                                        <i class="fas fa-file"></i>
                                                        ls-lR.gz
                                                    </a>
                                                </td>
                                                <td class="time">
                                                    <time datetime="2025-10-20T16:17:40&#43;02:00">2025-10-20 16:17 CEST</time>
                                                </td>
                                                <td class="size" title="13983255 bytes">13 MiB</td>
                                            </tr>

                                        </tbody>
                                    </table>

                            </div>

                        </article>
                    </main>

                <footer>
                    <div class="right">
                        Powered by <a class="button" href="https://www.snt.utwente.nl/">SNT</a>
                    </div>
                    <div class="left">
                        <p class="bandwidth">Current bandwidth utilization 229.65
                            Mbit/s</p>
                        <meter class="bandwidth" min="0" max="10000" value="229.64864925016323"></meter>
                    </div>
                </footer>
            </div>
            </body>
            </html>

            "##);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.len(), 15);
    let entries = httpdir.entries();

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::Directory, "dists", 0, "2025-09-06 10:15");
    assert_entry(&entries[2], &EntryType::Directory, "doc", 0, "2025-10-19 14:12");
    assert_entry(&entries[3], &EntryType::Directory, "indices", 0, "2025-10-19 14:59");
    assert_entry(&entries[4], &EntryType::Directory, "pool", 0, "2022-10-05 17:09");
    assert_entry(&entries[5], &EntryType::Directory, "project", 0, "2008-11-17 23:05");
    assert_entry(&entries[6], &EntryType::Directory, "tools", 0, "2012-10-10 16:29");
    assert_entry(&entries[7], &EntryType::Directory, "zzz-dists", 0, "2025-08-09 12:48");
    assert_entry(&entries[8], &EntryType::File, "README", 1201, "2025-09-06 12:15");
    assert_entry(&entries[9], &EntryType::File, "README.CD-manufacture", 1290, "2010-06-26 11:52");
    assert_entry(&entries[10], &EntryType::File, "README.html", 2919, "2025-09-06 12:15");
    assert_entry(&entries[11], &EntryType::File, "README.mirrors.html", 291, "2017-03-04 21:08");
    assert_entry(&entries[12], &EntryType::File, "README.mirrors.txt", 86, "2017-03-04 21:08");
    assert_entry(&entries[13], &EntryType::File, "extrafiles", 211968, "2025-10-20 16:23");
    assert_entry(&entries[14], &EntryType::File, "ls-lR.gz", 13631488, "2025-10-20 16:17");

    mock.assert();
}
