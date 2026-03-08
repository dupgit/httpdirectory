use crate::{
    error::{Result, SelectorResultExt},
    httpdirectoryentry::HttpDirectoryEntry,
    scrape::{are_table_headers_present, build_entry, extract_col_text, extract_link, remove_empty_cell},
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
    if !are_table_headers_present(table) {
        return Ok(vec![]);
    }

    let row_selector = Selector::parse("tr").with_selector("tr")?;
    let col_selector = Selector::parse("td").with_selector("td")?;
    let link_selector = Selector::parse("a").with_selector("a")?;

    let entries = table
        .select(&row_selector)
        .filter_map(|row| {
            let one_line: Vec<_> = row.select(&col_selector).collect();
            let mut iter = one_line.iter();

            let (name, link) = iter.next().map(|col| {
                let name = remove_empty_cell(col.text().collect());
                let link = extract_link(col, &link_selector);
                trace!("name: {name:?}, link: {link}");
                (name, link)
            })?;

            let size = extract_col_text(&mut iter);
            let date = extract_col_text(&mut iter);
            trace!("date: {date:?}, size: {size:?}");

            build_entry(&name, &date, &size, link)
        })
        .collect();

    Ok(entries)
}
