extern crate httpdirectory;
use httpdirectory::{
    httpdirectory::{HttpDirectory, get_entries_from_body},
    httpdirectoryentry::{EntryType, HttpDirectoryEntry, assert_entry},
};
use httpmock::prelude::*;

const SELF_MINISERVE_INPUT: &str = r##"
<html><head><meta charset="utf-8"><meta http-equiv="X-UA-Compatible" content="IE=edge"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="color-scheme" content="dark light"><link rel="icon" type="image/svg+xml" href="/__miniserve_internal/favicon.svg"><link rel="stylesheet" href="/__miniserve_internal/style.css"><title>127.0.0.1:8080</title><script>
                    // updates the color scheme by setting the theme data attribute
                    // on body and saving the new theme to local storage
                    function updateColorScheme(name) {
                        if (name && name != "default") {
                            localStorage.setItem('theme', name);
                            document.body.setAttribute("data-theme", name)
                        } else {
                            localStorage.removeItem('theme');
                            document.body.removeAttribute("data-theme")
                        }
                    }

                    // read theme from local storage and apply it to body
                    function loadColorScheme() {
                        var name = localStorage.getItem('theme');
                        updateColorScheme(name);
                    }

                    // load saved theme on page load
                    addEventListener("load", loadColorScheme);
                    // load saved theme when local storage is changed (synchronize between tabs)
                    addEventListener("storage", loadColorScheme);
                </script><script>const API_ROUTE = '/__miniserve_internal/api';
                    let dirSizeCache = {};

                    // Query the directory size from the miniserve API
                    function fetchDirSize(dir) {
                        return fetch(API_ROUTE, {
                            headers: {
                                'Accept': 'application/json',
                                'Content-Type': 'application/json'
                            },
                            method: 'POST',
                            body: JSON.stringify({
                                DirSize: dir
                            })
                        }).then(resp => resp.ok ? resp.text() : "~")
                    }

                    function updateSizeCells() {
                        const directoryCells = document.querySelectorAll('tr.entry-type-directory .size-cell');

                        directoryCells.forEach(cell => {
                            // Get the dir from the sibling anchor tag.
                            const href = cell.parentNode.querySelector('a').href;
                            const target = new URL(href).pathname;

                            // First check our local cache
                            if (target in dirSizeCache) {
                                cell.dataset.size = dirSizeCache[target];
                            } else {
                                fetchDirSize(target).then(dir_size => {
                                    cell.dataset.size = dir_size;
                                    dirSizeCache[target] = dir_size;
                                })
                                .catch(error => console.error("Error fetching dir size:", error));
                            }
                        })
                    }
                    setInterval(updateSizeCells, 1000);
                </script></head><body id="drop-container" data-theme="monokai"><div class="toolbar_box_group"></div><nav><div><p>Change theme...</p><ul class="theme"><li data-theme="default"><a href="javascript:updateColorScheme(&quot;default&quot;)" title="Switch to Default (light/dark) theme">Default (light/dark)</a></li><li data-theme="squirrel"><a href="javascript:updateColorScheme(&quot;squirrel&quot;)" title="Switch to Squirrel (light) theme">Squirrel (light)</a></li><li data-theme="archlinux"><a href="javascript:updateColorScheme(&quot;archlinux&quot;)" title="Switch to Arch Linux (dark) theme">Arch Linux (dark)</a></li><li data-theme="zenburn"><a href="javascript:updateColorScheme(&quot;zenburn&quot;)" title="Switch to Zenburn (dark) theme">Zenburn (dark)</a></li><li data-theme="monokai"><a href="javascript:updateColorScheme(&quot;monokai&quot;)" title="Switch to Monokai (dark) theme">Monokai (dark)</a></li></ul></div></nav><div class="container"><span id="top"></span><h1 class="title" dir="ltr"><span><bdi>127.0.0.1:8080</bdi></span>/</h1><div class="toolbar"><div class="toolbar_box_group"></div></div><table><thead><tr><th class="name"><span class=""><span class="chevron">▾</span><a href="?sort=name&amp;order=asc" title="Sort by name in ascending order">Name</a></span></th><th class="size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">Size</a></span></th><th class="date"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">Last modification</a></span></th></tr></thead><tbody><tr class="entry-type-directory"><td><p><a class="directory" href="/benches/">benches/</a></p></td><td class="size-cell" data-size="-"></td><td class="date-cell"><span>2025-07-07 21:56:22 +02:00 </span><span class="history">3 months ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/Cargo.lock">Cargo.lock</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">70.5 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">15 hours ago</a></span></span></p></td><td class="size-cell">70.5 KiB</td><td class="date-cell"><span>2025-10-21 23:03:32 +02:00 </span><span class="history">15 hours ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/Cargo.toml">Cargo.toml</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">886 B</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">15 hours ago</a></span></span></p></td><td class="size-cell">886 B</td><td class="date-cell"><span>2025-10-21 23:03:32 +02:00 </span><span class="history">15 hours ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/ChangeLog">ChangeLog</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">972 B</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">16 hours ago</a></span></span></p></td><td class="size-cell">972 B</td><td class="date-cell"><span>2025-10-21 22:23:56 +02:00 </span><span class="history">16 hours ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/cloud_debian.png">cloud_debian.png</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">14.6 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">5 months ago</a></span></span></p></td><td class="size-cell">14.6 KiB</td><td class="date-cell"><span>2025-05-16 22:40:53 +02:00 </span><span class="history">5 months ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/deny.toml">deny.toml</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">34 B</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">5 months ago</a></span></span></p></td><td class="size-cell">34 B</td><td class="date-cell"><span>2025-04-26 00:00:13 +02:00 </span><span class="history">5 months ago</span></td></tr><tr class="entry-type-directory"><td><p><a class="directory" href="/examples/">examples/</a></p></td><td class="size-cell" data-size="-"></td><td class="date-cell"><span>2025-10-21 22:23:56 +02:00 </span><span class="history">16 hours ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/httpdirectory.sbom.spdx.json">httpdirectory.sbom.spdx.json</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">229.8 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">15 hours ago</a></span></span></p></td><td class="size-cell">229.8 KiB</td><td class="date-cell"><span>2025-10-21 23:04:47 +02:00 </span><span class="history">15 hours ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/LICENSE-APACHE">LICENSE-APACHE</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">10.6 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">5 months ago</a></span></span></p></td><td class="size-cell">10.6 KiB</td><td class="date-cell"><span>2025-05-12 23:18:59 +02:00 </span><span class="history">5 months ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/LICENSE-MIT">LICENSE-MIT</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">1.0 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">5 months ago</a></span></span></p></td><td class="size-cell">1.0 KiB</td><td class="date-cell"><span>2025-05-12 23:18:59 +02:00 </span><span class="history">5 months ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/mirror.list">mirror.list</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">14.2 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">4 months ago</a></span></span></p></td><td class="size-cell">14.2 KiB</td><td class="date-cell"><span>2025-05-31 23:10:24 +02:00 </span><span class="history">4 months ago</span></td></tr><tr class="entry-type-directory"><td><p><a class="directory" href="/mutants.out/">mutants.out/</a></p></td><td class="size-cell" data-size="-"></td><td class="date-cell"><span>2025-06-05 20:57:13 +02:00 </span><span class="history">4 months ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/README.md">README.md</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">3.1 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">4 hours ago</a></span></span></p></td><td class="size-cell">3.1 KiB</td><td class="date-cell"><span>2025-10-22 10:06:50 +02:00 </span><span class="history">4 hours ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/release.toml">release.toml</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">198 B</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">5 months ago</a></span></span></p></td><td class="size-cell">198 B</td><td class="date-cell"><span>2025-04-26 00:16:51 +02:00 </span><span class="history">5 months ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/rustfmt.toml">rustfmt.toml</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">45 B</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">4 months ago</a></span></span></p></td><td class="size-cell">45 B</td><td class="date-cell"><span>2025-06-03 22:35:14 +02:00 </span><span class="history">4 months ago</span></td></tr><tr class="entry-type-directory"><td><p><a class="directory" href="/src/">src/</a></p></td><td class="size-cell" data-size="-"></td><td class="date-cell"><span>2025-10-21 22:23:56 +02:00 </span><span class="history">16 hours ago</span></td></tr><tr class="entry-type-directory"><td><p><a class="directory" href="/target/">target/</a></p></td><td class="size-cell" data-size="-"></td><td class="date-cell"><span>2025-10-22 14:45:01 +02:00 </span><span class="history">2 minutes ago</span></td></tr><tr class="entry-type-file"><td><p><a class="file" href="/tarpaulin-report.html">tarpaulin-report.html</a><span class="mobile-info size"><span class=""><span class="chevron">▾</span><a href="?sort=size&amp;order=asc" title="Sort by size in ascending order">772.0 KiB</a></span></span><span class="mobile-info history"><span class=""><span class="chevron">▾</span><a href="?sort=date&amp;order=asc" title="Sort by date in ascending order">a minute ago</a></span></span></p></td><td class="size-cell">772.0 KiB</td><td class="date-cell"><span>2025-10-22 14:45:51 +02:00 </span><span class="history">a minute ago</span></td></tr><tr class="entry-type-directory"><td><p><a class="directory" href="/tests/">tests/</a></p></td><td class="size-cell" data-size="-"></td><td class="date-cell"><span>2025-05-31 23:10:24 +02:00 </span><span class="history">4 months ago</span></td></tr></tbody></table><a class="back" href="#top">⇪</a><div class="footer"><div class="version"><a href="https://github.com/svenstaro/miniserve">miniserve</a>/0.32.0</div></div></div><div class="upload_area" id="upload_area"><template id="upload_file_item"><li class="upload_file_item"><div class="upload_file_container"><div class="upload_file_text"><span class="file_upload_percent"></span> - <span class="file_size"></span> - <span class="file_name"></span></div><button class="file_cancel_upload">✖</button></div><div class="file_progress_bar"></div></li></template><div class="upload_container"><div class="upload_header"><h4 style="margin:0px" id="upload_title"></h4><svg id="upload-toggle" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"><path stroke-linecap="round" stroke-linejoin="round" d="m4.5 15.75 7.5-7.5 7.5 7.5"></path></svg></div><div class="upload_action"><p id="upload_action_text">Starting upload...</p><button class="upload_cancel" id="upload_cancel">CANCEL</button></div><div class="upload_files"><ul class="upload_file_list" id="upload_file_list"></ul></div></div></div></body></html>
"##;

fn assert_self_miniserve_entries(entries: &Vec<HttpDirectoryEntry>) {
    assert_eq!(entries.len(), 19);

    assert_entry(&entries[0], &EntryType::Directory, "benches/", 0, "2025-07-07 21:56");
    assert_entry(&entries[1], &EntryType::File, "Cargo.lock", 72192, "2025-10-21 23:03");
    assert_entry(&entries[2], &EntryType::File, "Cargo.toml", 886, "2025-10-21 23:03");
    assert_entry(&entries[3], &EntryType::File, "ChangeLog", 972, "2025-10-21 22:23");
    assert_entry(&entries[4], &EntryType::File, "cloud_debian.png", 14950, "2025-05-16 22:40");
    assert_entry(&entries[5], &EntryType::File, "deny.toml", 34, "2025-04-26 00:00");
    assert_entry(&entries[6], &EntryType::Directory, "examples/", 0, "2025-10-21 22:23");
    assert_entry(&entries[7], &EntryType::File, "httpdirectory.sbom.spdx.json", 235315, "2025-10-21 23:04");
    assert_entry(&entries[8], &EntryType::File, "LICENSE-APACHE", 10854, "2025-05-12 23:18");
    assert_entry(&entries[9], &EntryType::File, "LICENSE-MIT", 1024, "2025-05-12 23:18");
    assert_entry(&entries[10], &EntryType::File, "mirror.list", 14540, "2025-05-31 23:10");
    assert_entry(&entries[11], &EntryType::Directory, "mutants.out/", 0, "2025-06-05 20:57");
    assert_entry(&entries[12], &EntryType::File, "README.md", 3174, "2025-10-22 10:06");
    assert_entry(&entries[13], &EntryType::File, "release.toml", 198, "2025-04-26 00:16");
    assert_entry(&entries[14], &EntryType::File, "rustfmt.toml", 45, "2025-06-03 22:35");
    assert_entry(&entries[15], &EntryType::Directory, "src/", 0, "2025-10-21 22:23");
    assert_entry(&entries[16], &EntryType::Directory, "target/", 0, "2025-10-22 14:45");
    assert_entry(&entries[17], &EntryType::File, "tarpaulin-report.html", 790528, "2025-10-22 14:45");
    assert_entry(&entries[18], &EntryType::Directory, "tests/", 0, "2025-05-31 23:10");
}

#[allow(dead_code)]
pub async fn mock_self_miniserve() -> Result<(), Box<dyn std::error::Error>> {
    // Start a lightweight mock server.
    let server = MockServer::start();
    let url = server.url("/miniserve");

    let mock = server.mock(|when, then| {
        when.path("/miniserve");
        then.status(200).body(SELF_MINISERVE_INPUT);
    });

    let httpdir = match HttpDirectory::new(&url).await {
        Ok(httpdir) => httpdir,
        Err(e) => panic!("{e}"),
    };

    let entries = httpdir.entries();
    assert_self_miniserve_entries(entries);

    mock.assert();

    Ok(())
}

#[allow(dead_code)]
pub fn run_self_miniserve() -> Result<(), Box<dyn std::error::Error>> {
    let body = SELF_MINISERVE_INPUT;
    let entries = get_entries_from_body(body);

    assert_self_miniserve_entries(&entries);

    Ok(())
}
