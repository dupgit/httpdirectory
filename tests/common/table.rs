extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::HttpDirectory, httpdirectory::Sorting, httpdirectory::get_entries_from_body,
    httpdirectoryentry::EntryType, httpdirectoryentry::HttpDirectoryEntry, httpdirectoryentry::assert_entry,
};
use httpmock::prelude::*;
use unwrap_unreachable::UnwrapUnreachable;

const DEBIAN_EXAMPLE_INPUT: &str = r##"
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
<address>Apache/2.4.63 (Unix) Server at cloud.debian.org Port 443</address>"##;

fn assert_debian_example_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 12);

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
}

#[allow(dead_code)]
pub async fn mock_debian_example() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(DEBIAN_EXAMPLE_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_eq!(httpdir.get_url().to_string(), url.to_string());
    let entries = httpdir.entries();
    assert_debian_example_entries(entries);

    let dirs = httpdir.dirs();
    assert_eq!(dirs.len(), 11);

    // unreachable is is fine as we know that "b" regex is
    // a valid regex
    let filtered = dirs.filter_by_name("b").unreachable();
    let entries = filtered.entries();

    assert_eq!(entries.len(), 7);

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

#[allow(dead_code)]
pub fn run_debian_example() -> Result<(), Box<dyn std::error::Error>> {
    let body = DEBIAN_EXAMPLE_INPUT;
    let entries = get_entries_from_body(body);

    assert_debian_example_entries(&entries);
    Ok(())
}

const FIRST_OLD_BSD_EXAMPLE_INPUT: &str = r##"
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
"##;

fn assert_first_old_bsd_example_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 76);

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
}

const SECOND_OLD_BSD_EXAMPLE_INPUT: &str = r##"
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
"##;

fn assert_second_old_bsd_example_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 2);

    assert_entry(&entries[0], &EntryType::ParentDirectory, "../", 0, "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "zenicb.el", 26_902, "1996-11-04 07:00");
}

#[allow(dead_code)]
pub async fn mock_old_bsd_example() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = &server.url("/old-bsd/");

    let mock = server.mock(|when, then| {
        when.path("/old-bsd/");
        then.status(200).body(FIRST_OLD_BSD_EXAMPLE_INPUT);
    });

    let httpdir = match HttpDirectory::new(url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    // The library fails to get this directory properly
    let entries = httpdir.entries();
    assert_first_old_bsd_example_entries(entries);

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
        then.status(200).body(SECOND_OLD_BSD_EXAMPLE_INPUT);
    });

    let httpdir = match httpdir.cd(dir).await {
        Ok(httpdir) => httpdir,
        Err(_) => panic!("This test should return Ok()"),
    };

    let entries = httpdir.entries();
    assert_second_old_bsd_example_entries(entries);

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_first_old_bsd_example() -> Result<(), Box<dyn std::error::Error>> {
    let body = FIRST_OLD_BSD_EXAMPLE_INPUT;
    let entries = get_entries_from_body(body);
    assert_first_old_bsd_example_entries(&entries);

    Ok(())
}

#[allow(dead_code)]
pub fn run_second_old_bsd_example() -> Result<(), Box<dyn std::error::Error>> {
    let body = SECOND_OLD_BSD_EXAMPLE_INPUT;
    let entries = get_entries_from_body(body);
    assert_second_old_bsd_example_entries(&entries);

    Ok(())
}
