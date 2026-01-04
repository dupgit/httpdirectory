extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::{HttpDirectory, get_entries_from_body},
    httpdirectoryentry::{EntryType, HttpDirectoryEntry, assert_entry},
};
use httpmock::prelude::*;

const DEBIAN_SNT_INPUT: &str = r##"
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

"##;

fn assert_debian_snt_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 15);

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
}

#[allow(dead_code)]
pub async fn mock_debian_snt() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/debian");

    let mock = server.mock(|when, then| {
        when.path("/debian");
        then.status(200).body(DEBIAN_SNT_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url, None).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    let entries = httpdir.entries();
    assert_debian_snt_entries(entries);

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_debian_snt() -> Result<(), Box<dyn std::error::Error>> {
    let body = DEBIAN_SNT_INPUT;
    let entries = get_entries_from_body(body);

    assert_debian_snt_entries(&entries);

    Ok(())
}
