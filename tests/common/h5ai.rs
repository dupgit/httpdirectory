extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::{HttpDirectory, get_entries_from_body},
    httpdirectoryentry::{EntryType, HttpDirectoryEntry, assert_entry},
};
use httpmock::prelude::*;

const DEBIAN_H5AI_INPUT: &str = r##"
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
"##;

fn assert_debian_h5ai_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 14);

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
}

#[allow(dead_code)]
pub async fn mock_debian_h5ai() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(DEBIAN_H5AI_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    let entries = httpdir.entries();
    assert_debian_h5ai_entries(entries);

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_debian_h5ai() -> Result<(), Box<dyn std::error::Error>> {
    let body = DEBIAN_H5AI_INPUT;
    let entries = get_entries_from_body(body);

    assert_debian_h5ai_entries(&entries);

    Ok(())
}
