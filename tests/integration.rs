extern crate httpdirectory;
use chrono::NaiveDate;
use httpdirectory::{httpdirectory::HttpDirectory, httpdirectoryentry::HttpDirectoryEntry};
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

// Helper function to assert a directory entry is what is expected
fn assert_entry(
    dir_entry: &HttpDirectoryEntry,
    parent: bool,
    dir: bool,
    file: bool,
    name: &str,
    size: usize,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minutes: u32,
) {
    // Use cargo t -- --show-output
    println!("{dir_entry:?}, {parent}, {dir}, {file}, {name}, {size}, {year}, {month}, {day}, {hour}, {minutes}");
    match dir_entry {
        HttpDirectoryEntry::Directory(entry) => {
            assert_eq!(dir, true);
            assert_eq!(entry.apparent_size(), size);
            assert_eq!(entry.name(), name);
            assert_eq!(
                entry.date(),
                Some(NaiveDate::from_ymd_opt(year, month, day).unwrap().and_hms_opt(hour, minutes, 0).unwrap())
            );
        }
        HttpDirectoryEntry::File(entry) => {
            assert_eq!(file, true);
            assert_eq!(entry.apparent_size(), size);
            assert_eq!(entry.name(), name);
            assert_eq!(
                entry.date(),
                Some(NaiveDate::from_ymd_opt(year, month, day).unwrap().and_hms_opt(hour, minutes, 0).unwrap())
            );
        }
        HttpDirectoryEntry::ParentDirectory(link) => {
            assert_eq!(parent, true);
            assert_eq!(link, name);
        }
    }
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

    assert_entry(&entries[0], true, false, false, "/images/", 0, 0, 0, 0, 0, 0);
    assert_entry(&entries[1], false, true, false, "OpenStack/", 0, 2024, 7, 1, 23, 19);
    assert_entry(&entries[2], false, true, false, "bookworm-backports/", 0, 2025, 4, 28, 21, 33);
    assert_entry(&entries[3], false, true, false, "bookworm/", 0, 2025, 4, 28, 20, 53);
    assert_entry(&entries[4], false, true, false, "bullseye-backports/", 0, 2025, 5, 5, 17, 45);
    assert_entry(&entries[5], false, true, false, "bullseye/", 0, 2025, 5, 5, 16, 52);
    assert_entry(&entries[6], false, true, false, "buster-backports/", 0, 2024, 7, 3, 21, 46);
    assert_entry(&entries[7], false, true, false, "buster/", 0, 2024, 7, 3, 21, 46);
    assert_entry(&entries[8], false, true, false, "sid/", 0, 2024, 4, 1, 14, 20);
    assert_entry(&entries[9], false, true, false, "stretch-backports/", 0, 2019, 7, 18, 10, 40);
    assert_entry(&entries[10], false, true, false, "stretch/", 0, 2019, 7, 18, 10, 40);
    assert_entry(&entries[11], false, true, false, "trixie/", 0, 2023, 7, 25, 7, 43);

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

    assert_entry(&entries[0], true, false, false, "../", 0, 0, 0, 0, 0, 0);
    assert_entry(&entries[1], false, true, false, "7.5/", 0, 2024, 4, 05, 11, 59);
    assert_entry(&entries[2], false, true, false, "7.6/", 0, 2024, 10, 08, 17, 17);
    assert_entry(&entries[3], false, true, false, "7.7/", 0, 2025, 4, 27, 17, 58);
    assert_entry(&entries[4], false, true, false, "Changelogs/", 0, 2025, 5, 12, 17, 21);
    assert_entry(&entries[5], false, true, false, "LibreSSL/", 0, 2025, 04, 30, 06, 55);
    assert_entry(&entries[6], false, true, false, "OpenBGPD/", 0, 2025, 02, 06, 15, 30);
    assert_entry(&entries[7], false, true, false, "OpenIKED/", 0, 2025, 04, 10, 17, 10);
    assert_entry(&entries[8], false, true, false, "OpenNTPD/", 0, 2020, 12, 09, 14, 56);
    assert_entry(&entries[9], false, true, false, "OpenSSH/", 0, 2025, 04, 09, 07, 08);
    assert_entry(&entries[10], false, true, false, "doc/", 0, 2013, 04, 28, 15, 57);
    assert_entry(&entries[11], false, true, false, "patches/", 0, 2025, 05, 04, 21, 25);
    assert_entry(&entries[12], false, true, false, "rpki-client/", 0, 2025, 04, 11, 22, 09);
    assert_entry(&entries[13], false, true, false, "signify/", 0, 2025, 05, 06, 15, 03);
    assert_entry(&entries[14], false, true, false, "snapshots/", 0, 2025, 05, 13, 04, 06);
    assert_entry(&entries[15], false, true, false, "songs/", 0, 2023, 04, 06, 22, 15);
    assert_entry(&entries[16], false, true, false, "stable/", 0, 2022, 01, 18, 16, 25);
    assert_entry(&entries[17], false, true, false, "syspatch/", 0, 2025, 03, 03, 15, 17);
    assert_entry(&entries[18], false, true, false, "tools/", 0, 2005, 01, 07, 19, 40);
    assert_entry(&entries[19], false, false, true, "README", 1329, 2017, 10, 06, 11, 51);
    assert_entry(&entries[20], false, false, true, "ftplist", 4836, 2025, 05, 13, 03, 57);
    assert_entry(&entries[21], false, false, true, "timestamp", 11, 2025, 05, 13, 04, 00);

    mock.assert();
}
