use crate::{error::HttpDirError, httpdirectoryentry::HttpDirectoryEntry};
use log::{debug, trace};
use scraper::{Html, Selector};

// @todo: add some validation statistics to decide if
// what we have been scraping is real data or not
// This should be done in for loops (table and pre)

// @todo: manage Results and Options ie: remove unwrap()
fn scrape_table(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let table_selector = Selector::parse("table").unwrap();
    let table_iter = html.select(&table_selector);

    for table in table_iter {
        let row_selector = Selector::parse("tr").unwrap();
        let col_selector = Selector::parse("td").unwrap();
        let link_selector = Selector::parse("a").unwrap();
        for row in table.select(&row_selector) {
            let one_line: Vec<_> = row.select(&col_selector).map(|c| c).collect();
            let mut one_line_iter = one_line.iter();

            let mut name = vec![];
            let mut link = "";
            let mut date = vec![];
            let mut size = vec![];

            // First column in the line is the icon that represents the entry
            // (folder, file, parentdir,…) it has no text. Sometimes the website
            // has no icon column but a text one it is likely to be the name of
            // the file or directory along with it's link
            if let Some(first_col) = one_line_iter.next() {
                let first_col_txt = first_col.text().collect::<Vec<_>>();
                trace!("first_col: {first_col_txt:?}",);
                if !first_col_txt.is_empty() {
                    name = first_col_txt;
                    // Text exists so we have a name, now getting the link
                    for link_selected in first_col.select(&link_selector) {
                        link = link_selected.value().attr("href").unwrap();
                    }
                } else {
                    // First column was empty, the name should be in the second one
                    if let Some(name_col) = one_line_iter.next() {
                        // Second column is the name of the file or directory with its link
                        name = name_col.text().collect::<Vec<_>>();
                        for link_selected in name_col.select(&link_selector) {
                            link = link_selected.value().attr("href").unwrap();
                        }
                    }
                }
                trace!("name: {name:?}, link: {link}");
            }

            // Third column contains the date of the file or directory
            // In some case it can be the size of the file Entry::new()
            // handles this
            if let Some(date_col) = one_line_iter.next() {
                date = date_col.text().collect::<Vec<_>>();
            }

            // Fourth column contains the size of the file (' - ' for a
            // directory). In some case it can be the date of the file
            // (Entry::new() handles this
            if let Some(size_col) = one_line_iter.next() {
                size = size_col.text().collect::<Vec<_>>();
            }

            trace!("date: {date:?}, size: {size:?}");
            if name.len() > 0 && date.len() > 0 && size.len() > 0 {
                http_dir_entry.push(HttpDirectoryEntry::new(name[0], date[0], size[0], link));
            }
        }
    }

    Ok(http_dir_entry)
}

fn scrape_pre_with_img(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut should_be_considered_valid = false;
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let pre_selector = Selector::parse("pre").unwrap();
    let pre_iter = html.select(&pre_selector);

    for pre in pre_iter {
        if pre.inner_html().contains("<img") {
            trace!("Analyzing <pre> tag");
            // <img> tag represents the icon at the beginning of the line
            for line in pre.inner_html().split("<img") {
                // Removing the img tag (we know that > exists in line)
                let new_line = strip_until_greater(line);
                if new_line.len() > 0 {
                    // Considering only non empty lines
                    trace!("{new_line}");
                    let href = new_line
                        .split("</a>")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|x| x.trim())
                        .collect::<Vec<&str>>();
                    trace!("{href:?}");
                    if href.len() >= 4 {
                        // Headers with Name, Last modified, Size, Description columns
                        should_be_considered_valid = is_this_a_real_header(href);
                    } else if href.len() >= 2 {
                        // Rows with a link and a name and the rest of the data (date, size and description)
                        let (link, name) = get_link_and_name(href[0]);
                        if name.to_lowercase() == "parent directory" {
                            http_dir_entry.push(HttpDirectoryEntry::ParentDirectory(link.to_string()));
                        } else {
                            let (date, size) = get_date_and_size(href[1]);
                            http_dir_entry.push(HttpDirectoryEntry::new(name, date, size, link));
                        }
                    }
                }
            }
            if should_be_considered_valid {
                // We have analyzed valid entries: no need to inspect other <pre> tags
                return Ok(http_dir_entry);
            } else {
                debug!("Unable to get entry from this body (no headers ?):\n{body}");
            }
        } else {
            debug!("Unable to get entry from this body (no <img> tag):\n{}", pre.inner_html());
        }
    }
    Ok(http_dir_entry)
}

// Forms of the line:
// '2025-04-30 16:31  256M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds'
// '13-May-2025 03:57                4836'  (no description here and date format is bigger)
fn get_date_and_size(line: &str) -> (&str, &str) {
    let index = match line.find(' ') {
        Some(index) => match line[index + 1..].find(' ') {
            Some(index2) => index + index2 + 1,
            None => index,
        },
        None => 0,
    };
    trace!("index: {index}");
    let date = &line[0..index];
    let size_and_description = &line[index..];

    let line_split: Vec<&str> = size_and_description.trim().split(" ").collect();
    let size;
    if line_split.len() >= 2 {
        size = line_split[0];
    } else {
        size = &size_and_description;
    }
    trace!(" -> date: {date}, size: {size}");
    (&date, size)
}

// Form of the line:  '<a href="bionic/">bionic/'
// Returns a tuple with the text of the link and the
// linked text as name. Here : ("bionic/", "bionic/")
fn get_link_and_name(line: &str) -> (&str, &str) {
    match line.find('>') {
        Some(num) => {
            let name = &line[num + 1..];
            let link = match &line[0..num].strip_prefix(r#"<a href=""#) {
                // Removing '<a href="' that prefixes the line
                Some(link) => match link.strip_suffix(r#"""#) {
                    // Removing trailing " if any
                    Some(l) => l,
                    None => link,
                },
                None => match line[0..num].strip_suffix(r#"""#) {
                    // Removing trailing " if any
                    Some(l) => l,
                    None => &line[0..num],
                },
            };
            let link = link.trim();
            let name = name.trim();
            trace!(" -> link: {link}, name: {name}");
            (link.trim(), name.trim())
        }
        None => {
            let name = line.trim();
            trace!(" -> link: , name: {name}");
            ("", name)
        }
    }
}

// Returns true if href vector contains
// Name, Last modified, Size, Description
// in this exact order
fn is_this_a_real_header(href: Vec<&str>) -> bool {
    let name = strip_until_greater(href[0]);
    let date = strip_until_greater(href[1]);
    let size = strip_until_greater(href[2]);
    let description = strip_until_greater(href[3]);
    trace!("This is the header: {}, {}, {}, {}", name, date, size, description);
    name.to_lowercase() == "name"
        && date.to_lowercase() == "last modified"
        && size.to_lowercase() == "size"
        && description.to_lowercase() == "description"
}

// Removes prefix until '>' sign (that we know exists in the line &str)
fn strip_until_greater(line: &str) -> &str {
    match line.find('>') {
        Some(num) => line.strip_prefix(&line[0..=num]).unwrap().trim(),
        None => line.trim(),
    }
}

fn scrape_pre_simple(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let pre_selector = Selector::parse("pre").unwrap();
    let pre_iter = html.select(&pre_selector);

    for pre in pre_iter {
        for line in pre.inner_html().lines() {
            if line.len() > 0 {
                // Considering only non empty lines
                trace!("{line}");
                let href =
                    line.split("</a>").collect::<Vec<&str>>().into_iter().map(|x| x.trim()).collect::<Vec<&str>>();
                trace!("{href:?}");
                if href.len() >= 2 {
                    // Rows with a link and a name and may be the rest of the data (date, size and description)
                    let (link, name) = get_link_and_name(href[0]);
                    if name.to_lowercase() == "../" {
                        http_dir_entry.push(HttpDirectoryEntry::ParentDirectory(link.to_string()));
                    } else {
                        let (date, size) = get_date_and_size(href[1]);
                        http_dir_entry.push(HttpDirectoryEntry::new(name, date, size, link));
                    }
                }
            }
        }
        return Ok(http_dir_entry);
    }

    Ok(http_dir_entry)
}

// @todo: manage Results and Options ie: remove unwrap()
pub fn scrape_body(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    if body.contains("<table") {
        debug!("body has <table> tag, trying this");
        scrape_table(body)
    } else if body.contains("<pre>") {
        debug!("body has <pre> tag, trying this");
        let http_dir_entry = scrape_pre_with_img(body)?;
        if http_dir_entry.is_empty() {
            let http_dir_entry = scrape_pre_simple(body)?;
            Ok(http_dir_entry)
        } else {
            Ok(http_dir_entry)
        }
    } else {
        Ok(vec![])
    }
}
