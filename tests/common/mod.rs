extern crate httpdirectory;
use httpdirectory::{httpdirectory::HttpDirectory, httpdirectoryentry::EntryType, httpdirectoryentry::assert_entry};
use httpmock::prelude::*;
use unwrap_unreachable::UnwrapUnreachable;

pub async fn run_debian_example() -> Result<(), Box<dyn std::error::Error>> {
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

    // unreachable is is fine as we know that "b" regex is
    // a valid regex
    let filtered = dirs.filter_by_name("b").unreachable();
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
    Ok(())
}
