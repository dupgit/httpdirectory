use crate::{
    error::{Result, SelectorResultExt},
    httpdirectoryentry::HttpDirectoryEntry,
    scrape::{are_table_headers_present, remove_empty_cell},
};
use scraper::{ElementRef, Html, Selector};
use tracing::trace;

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub(crate) fn scrape_miniserve(body: &str, _version: &str) -> Result<Vec<HttpDirectoryEntry>> {
    let http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let div_selector = Selector::parse("div").with_selector("div")?;
    let table_selector = Selector::parse("table").with_selector("table")?;

    for node in html.select(&div_selector) {
        if let Some(table) = node.select(&table_selector).next() {
            return parse_miniserve_table(table);
        }
    }
    Ok(http_dir_entry)
}

pub(crate) fn parse_miniserve_table(table: ElementRef) -> Result<Vec<HttpDirectoryEntry>> {
    let mut http_dir_entry = vec![];

    if are_table_headers_present(table) {
        let row_selector = Selector::parse("tr").with_selector("tr")?;
        let col_selector = Selector::parse("td").with_selector("td")?;
        let link_selector = Selector::parse("a").with_selector("a")?;
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

            if let Some(name) = name.first() {
                let (date, size) = match (date.first(), size.first()) {
                    (Some(&d), Some(&s)) => (d, s), // date and size both exists
                    (Some(&d), None) => (d, " - "), // size is empty this may be a directory
                    (None, Some(&s)) => ("", s),    // date may be empty for a parent directory for instance
                    (None, None) => ("", " - "),    // date and size may be empty for a parent directory for instance
                };
                http_dir_entry.push(HttpDirectoryEntry::new(name, date, size, link));
            }
        }
    }

    Ok(http_dir_entry)
}
