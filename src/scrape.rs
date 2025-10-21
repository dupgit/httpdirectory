use crate::{
    detect::{PureHtml, SiteType},
    error::HttpDirError,
    httpdirectoryentry::HttpDirectoryEntry,
    scrapers::{h5ai::scrape_h5ai, snt::scrape_snt, ul::scrape_ul},
};
use log::{debug, info, trace, warn};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

// @todo: add some validation statistics to decide if
// what we have been scraping is real data or not
// This should be done in for loops (table and pre)

// Tells whether the table we are inspecting is a table
// that contains the headers that we should find in a
// file list ("last modified", "modified" or "date")
fn are_table_headers_present(table: ElementRef) -> bool {
    let th_selector = Selector::parse("th").unwrap();
    let re = Regex::new(r"(?msi)Last modified|Modified|Date|Modification time").unwrap();

    for th in table.select(&th_selector) {
        let columns: Vec<_> = th.text().collect();
        for column in columns {
            if re.is_match(column) {
                return true;
            }
        }
    }

    warn!("This table does not contain any date header field");
    false
}

// Parses `body` variable to find a table that may
// have icon, name & link, date, size and description.
// We do not mind description field. Sometimes icon
// column (first one) is not empty (it has text) so
// it may be that this is in fact the name & link
// column
pub(crate) fn scrape_table(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let table_selector = Selector::parse("table")?;
    let table_iter = html.select(&table_selector);

    for table in table_iter {
        if are_table_headers_present(table) {
            let row_selector = Selector::parse("tr")?;
            let col_selector = Selector::parse("td")?;
            let link_selector = Selector::parse("a")?;
            for row in table.select(&row_selector) {
                let one_line: Vec<_> = row.select(&col_selector).collect();
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
                    let mut first_col_txt = first_col.text().collect::<Vec<_>>();
                    first_col_txt = remove_empty_cell(first_col_txt);
                    trace!("first_col: {first_col_txt:?}",);
                    if first_col_txt.is_empty() {
                        // First column was empty, the name should be in the second one
                        if let Some(name_col) = one_line_iter.next() {
                            // Second column is the name of the file or directory with its link
                            name = name_col.text().collect::<Vec<_>>();
                            name = remove_empty_cell(name);
                            for link_selected in name_col.select(&link_selector) {
                                link = link_selected.value().attr("href").unwrap_or_default();
                            }
                        }
                    } else {
                        name = first_col_txt;
                        // Text exists so we have a name, now getting the link
                        for link_selected in first_col.select(&link_selector) {
                            link = link_selected.value().attr("href").unwrap_or_default();
                        }
                    }
                    trace!("name: {name:?}, link: {link}");
                }

                // Third column contains the date of the file or directory
                // In some case it can be the size of the file Entry::new()
                // handles this
                if let Some(date_col) = one_line_iter.next() {
                    date = date_col.text().collect::<Vec<_>>();
                    date = remove_empty_cell(date);
                }

                // Fourth column contains the size of the file (' - ' for a
                // directory). In some case it can be the date of the file
                // (Entry::new() handles this
                if let Some(size_col) = one_line_iter.next() {
                    size = size_col.text().collect::<Vec<_>>();
                    size = remove_empty_cell(size);
                }

                trace!("date: {date:?}, size: {size:?}");
                if !name.is_empty() && !date.is_empty() && !size.is_empty() {
                    http_dir_entry.push(HttpDirectoryEntry::new(name[0], date[0], size[0], link));
                } else if !name.is_empty() && !date.is_empty() && size.is_empty() {
                    // size is empty this may be is a directory
                    http_dir_entry.push(HttpDirectoryEntry::new(name[0], date[0], " - ", link));
                } else if !name.is_empty() && date.is_empty() && !size.is_empty() {
                    // date may be empty for a parent directory for instance
                    http_dir_entry.push(HttpDirectoryEntry::new(name[0], "", size[0], link));
                } else if !name.is_empty() && date.is_empty() && size.is_empty() {
                    // date and size may be empty for a parent directory for instance
                    http_dir_entry.push(HttpDirectoryEntry::new(name[0], "", " - ", link));
                }
            }
        }
    }

    Ok(http_dir_entry)
}

// Tries to search in a <pre> formatted table that
// contains <img> tag that represents the icon of
// the file
fn scrape_pre_with_img(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut should_be_considered_valid = false;
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let pre_selector = Selector::parse("pre")?;
    let pre_iter = html.select(&pre_selector);

    for pre in pre_iter {
        if pre.inner_html().contains("<img") {
            debug!("Analyzing <pre> tag with <img> tag");
            // <img> tag represents the icon at the beginning of the line
            for line in pre.inner_html().split("<img") {
                // Removing the img tag (we know that > exists in line)
                let new_line = strip_until_stop(line, "<a", false);
                if !new_line.is_empty() {
                    // Considering only non empty lines
                    trace!("{new_line}");
                    let href =
                        new_line.split("</a>").collect::<Vec<&str>>().into_iter().map(str::trim).collect::<Vec<&str>>();
                    trace!("{href:?}");
                    if href.len() >= 4 {
                        // Headers with Name, Last modified, Size, Description columns
                        should_be_considered_valid = is_this_a_real_header(&href);
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
            }
            trace!("Unable to get entry from this body (no headers ?):\n{body}");
        } else {
            trace!("Unable to get entry from this body (no <img> tag):\n{}", pre.inner_html());
        }
    }
    Ok(http_dir_entry)
}

fn remove_empty_cell(mut vector: Vec<&str>) -> Vec<&str> {
    vector.retain(|v| !v.trim().is_empty());
    vector
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

    let line_split: Vec<&str> = size_and_description.trim().split(' ').collect();
    let size = if line_split.len() >= 2 {
        line_split[0]
    } else {
        size_and_description
    };
    trace!(" -> date: {date}, size: {size}");
    (date, size)
}

// Form of the column:  '<a href="bionic/">bionic/'
// Returns a tuple with the text of the link and the
// linked text as name. Here : ("bionic/", "bionic/")
pub fn get_link_and_name(column: &str) -> (&str, &str) {
    match column.find('>') {
        Some(num) => {
            let name = &column[num + 1..];
            let link = match &column[0..num].strip_prefix(r#"<a href=""#) {
                // Removing '<a href="' that prefixes the line
                Some(link) => match link.strip_suffix(r#"""#) {
                    // Removing trailing " if any
                    Some(l) => l,
                    None => link,
                },
                None => match column[0..num].strip_suffix(r#"""#) {
                    // Removing trailing " if any
                    Some(l) => l,
                    None => &column[0..num],
                },
            };
            let link = link.trim();
            let name = name.trim();
            trace!(" -> link: {link}, name: {name}");
            (link.trim(), name.trim())
        }
        None => {
            let name = column.trim();
            trace!(" -> link: , name: {name}");
            ("", name)
        }
    }
}

// Returns true if href vector contains
// Name, Last modified, Size in this exact order
// Some websites does not provides a description
fn is_this_a_real_header(href: &[&str]) -> bool {
    let name = strip_until_stop(href[0], ">", true);
    let date = strip_until_stop(href[1], ">", true);
    let size = strip_until_stop(href[2], ">", true);
    // let description = strip_until_greater(href[3]);
    trace!("This is the header: {name}, {date}, {size}");

    name.to_lowercase() == "name" && date.to_lowercase() == "last modified" && size.to_lowercase() == "size"
    // && description.to_lowercase() == "description"
}

// Removes prefix until the stop '>' sign that we know
// exists in the line &str.
fn strip_until_stop<'a>(line: &'a str, stop: &str, remove: bool) -> &'a str {
    match line.find(stop) {
        Some(mut num) => {
            if !remove && num >= 1 {
                num = num - 1;
            }
            match line.strip_prefix(&line[0..=num]) {
                Some(line_without_prefix) => line_without_prefix.trim(),
                None => line.trim(),
            }
        }
        None => line.trim(),
    }
}

// Tries to search in a basic <pre> formatted table
// without any <img> tag
fn scrape_pre_simple(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let pre_selector = Selector::parse("pre")?;
    let pre_iter = html.select(&pre_selector);

    for pre in pre_iter {
        debug!("Analyzing <pre> tag");
        for line in pre.inner_html().lines() {
            if !line.is_empty() {
                // Considering only non empty lines
                trace!("{line}");
                let href = line.split("</a>").collect::<Vec<&str>>().into_iter().map(str::trim).collect::<Vec<&str>>();
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
    }

    Ok(http_dir_entry)
}

// Parses `body` that should contain an HTML page / body
// to recognize (if possible) entries of files, directories or
// a parent directory and fill a vector of `HttpDirectoryEntry`
// accordingly
pub fn scrape_body(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    match SiteType::detect(body) {
        SiteType::H5ai(version) => {
            info!("H5ai powered version {version} website detected");
            scrape_h5ai(body, &version)
        }
        SiteType::Snt => {
            info!("SNT index generator website detected");
            scrape_snt(body)
        }
        SiteType::NotNamed(html) => match html {
            PureHtml::Table => {
                info!("Body has <table> tag");
                scrape_table(body)
            }
            PureHtml::Pre => {
                info!("Body has <pre> tag");
                let http_dir_entry = scrape_pre_with_img(body)?;
                if http_dir_entry.is_empty() {
                    let http_dir_entry = scrape_pre_simple(body)?;
                    Ok(http_dir_entry)
                } else {
                    Ok(http_dir_entry)
                }
            }
            PureHtml::Ul => {
                info!("Body has no <table>, nor <pre> but <ul> tag");
                scrape_ul(body)
            }
        },
        SiteType::None => {
            warn!("Site type has not been detected: doing nothing");
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod test {
    use super::is_this_a_real_header;

    #[test]
    fn test_is_this_a_real_header() {
        let href = vec!["Name", "Last modified", "Size", "Description"];
        let header = is_this_a_real_header(&href);
        assert!(header);
    }

    #[test]
    fn test_this_is_not_a_real_header() {
        let href = vec!["Size", "Last modified", "Description", "Name"];
        let header = is_this_a_real_header(&href);
        assert!(!header);
    }
}
