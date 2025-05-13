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
            // First column in the line is the icon that represents the entry (folder, file, parentdir,…)
            let _ = one_line_iter.next();
            let mut name = vec![];
            let mut link = "";
            let mut date = vec![];
            let mut size = vec![];

            // Second column is the name of the file or directory with its link
            if let Some(name_col) = one_line_iter.next() {
                name = name_col.text().collect::<Vec<_>>();
                for link_selected in name_col.select(&link_selector) {
                    link = link_selected.value().attr("href").unwrap();
                }
            }

            // Third column contains the date of the file or directory
            if let Some(date_col) = one_line_iter.next() {
                date = date_col.text().collect::<Vec<_>>();
            }

            // Fourth column contains the size of the file (' - ' for a directory)
            if let Some(size_col) = one_line_iter.next() {
                size = size_col.text().collect::<Vec<_>>();
            }

            if name.len() > 0 && date.len() > 0 && size.len() > 0 {
                http_dir_entry.push(HttpDirectoryEntry::new(name[0], date[0], size[0], link));
            }
        }
    }

    Ok(http_dir_entry)
}

fn scrape_pre(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
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
            debug!("Unable to get entry from this body (no <img> tag):\n{body}");
        }
    }

    Ok(http_dir_entry)
}

// Form of the line:
// '2025-04-30 16:31  256M  Ubuntu Server 24.04 LTS (Noble Numbat) daily builds'
fn get_date_and_size(line: &str) -> (&str, &str) {
    let (date, size_and_description) = line.split_at(16);
    let line_split: Vec<&str> = size_and_description.trim().split(" ").collect();
    let size;
    if line_split.len() >= 2 {
        size = line_split[0];
    } else {
        size = size_and_description;
    }
    trace!(" -> date: {date}, size: {size}");
    (date, size)
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
            return (link.trim(), name.trim());
        }
        None => {
            let name = line.trim();
            trace!(" -> link: , name: {name}");
            return ("", name);
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

// @todo: manage Results and Options ie: remove unwrap()
pub fn scrape_body(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    if body.contains("<table") {
        debug!("body has <table> tag, trying this");
        return scrape_table(body);
    } else if body.contains("<pre>") {
        debug!("body has <pre> tag, trying this");
        return scrape_pre(body);
    } else {
        return Ok(vec![]);
    }
}
