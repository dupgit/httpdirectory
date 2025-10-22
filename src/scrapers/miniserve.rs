use crate::{
    error::HttpDirError,
    httpdirectoryentry::HttpDirectoryEntry,
    scrape::{are_table_headers_present, remove_empty_cell},
};
use log::trace;
use scraper::{ElementRef, Html, Selector};

pub(crate) fn scrape_miniserve(body: &str, _version: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let div_selector = Selector::parse("div")?;
    let table_selector = Selector::parse("table")?;

    for node in html.select(&div_selector) {
        if let Some(table) = node.select(&table_selector).next() {
            return parse_miniserve_table(table);
        }
    }
    Ok(http_dir_entry)
}

pub(crate) fn parse_miniserve_table(table: ElementRef) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    if are_table_headers_present(table) {
        let row_selector = Selector::parse("tr")?;
        let col_selector = Selector::parse("td")?;
        let link_selector = Selector::parse("a")?;
        for row in table.select(&row_selector) {
            let one_line: Vec<_> = row.select(&col_selector).collect();
            trace!("one_line: {one_line:?}");
            let mut one_line_iter = one_line.iter();

            let mut name = vec![];
            let mut link = "";
            let mut date = vec![];
            let mut size = vec![];

            // First column in the line is the name of
            // the file or directory along with it's link
            if let Some(first_col) = one_line_iter.next() {
                let mut first_col_txt = first_col.text().collect::<Vec<_>>();
                first_col_txt = remove_empty_cell(first_col_txt);
                name = first_col_txt;
                // First href has the link for the file or the directory
                if let Some(href_selected) = first_col.select(&link_selector).next() {
                    link = href_selected.value().attr("href").unwrap_or_default();
                }
                trace!("name: {name:?}, link: {link}");
            }

            // Second column contains the size of the file (' - ' for a
            // directory).
            if let Some(size_col) = one_line_iter.next() {
                size = size_col.text().collect::<Vec<_>>();
                size = remove_empty_cell(size);
            }

            // Third column contains the date of the file or directory
            if let Some(date_col) = one_line_iter.next() {
                date = date_col.text().collect::<Vec<_>>();
                date = remove_empty_cell(date);
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

    Ok(http_dir_entry)
}
