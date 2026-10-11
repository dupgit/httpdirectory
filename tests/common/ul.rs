extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::{HttpDirectory, get_entries_from_body},
    httpdirectoryentry::{EntryType, HttpDirectoryEntry, assert_entry},
};
use httpmock::prelude::*;

const DEBIAN_UL_INPUT: &str = r##"
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
            "##;

fn assert_debian_ul_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 15);

    assert_entry(&entries[0], &EntryType::ParentDirectory, "Parent Directory", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[1], &EntryType::File, "README", None, "0000-00-00 00:00");
    assert_entry(&entries[2], &EntryType::File, "README.CD-manufacture", None, "0000-00-00 00:00");
    assert_entry(&entries[3], &EntryType::File, "README.html", None, "0000-00-00 00:00");
    assert_entry(&entries[4], &EntryType::File, "README.mirrors.html", None, "0000-00-00 00:00");
    assert_entry(&entries[5], &EntryType::File, "README.mirrors.txt", None, "0000-00-00 00:00");
    assert_entry(&entries[6], &EntryType::Directory, "dists/", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[7], &EntryType::Directory, "doc/", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[8], &EntryType::File, "extrafiles", None, "0000-00-00 00:00");
    assert_entry(&entries[9], &EntryType::Directory, "indices/", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[10], &EntryType::File, "ls-lR.gz", None, "0000-00-00 00:00");
    assert_entry(&entries[11], &EntryType::Directory, "pool/", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[12], &EntryType::Directory, "project/", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[13], &EntryType::Directory, "tools/", Some(0), "0000-00-00 00:00");
    assert_entry(&entries[14], &EntryType::Directory, "zzz-dists/", Some(0), "0000-00-00 00:00");
}

#[allow(dead_code)]
pub fn run_debian_ul() -> Result<(), Box<dyn std::error::Error>> {
    let body = DEBIAN_UL_INPUT;
    let entries = get_entries_from_body(body);

    assert_debian_ul_entries(&entries);

    Ok(())
}

#[allow(dead_code)]
pub async fn mock_debian_ul() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(DEBIAN_UL_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url, None).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    assert_debian_ul_entries(httpdir.entries());

    mock.assert();

    Ok(())
}
