use crate::{error::HttpDirError, httpdirectoryentry::HttpDirectoryEntry};
// use log::debug;
use scraper::{Html, Selector};

// @todo: manage Results and Options ie: remove unwrap()
pub fn scrape_body(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
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
            let _ = one_line_iter.next(); // This is the icon
            let mut name = vec![];
            let mut link = "";
            let mut date = vec![];
            let mut size = vec![];

            if let Some(name_col) = one_line_iter.next() {
                name = name_col.text().collect::<Vec<_>>();
                for link_selected in name_col.select(&link_selector) {
                    link = link_selected.value().attr("href").unwrap();
                }
            }
            if let Some(date_col) = one_line_iter.next() {
                date = date_col.text().collect::<Vec<_>>();
            }
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
