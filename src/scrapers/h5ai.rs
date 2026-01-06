use crate::{
    error::{Result, SelectorResultExt},
    httpdirectoryentry::HttpDirectoryEntry,
    scrape::scrape_table,
};
use scraper::{Html, Selector};

// when getting the website's htwl we get a version with
// a fallback because Reqwest does not understand javascript
// and we do not want to do anything with javascript.
// Getting the two divs (`fallback-hints` and `fallback`) and
// decode the table found if any (should be on `fallback` div)
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub(crate) fn scrape_h5ai(body: &str, _version: &str) -> Result<Vec<HttpDirectoryEntry>> {
    let http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let div_selector = Selector::parse("div").with_selector("div")?;
    let table_selector = Selector::parse("table").with_selector("table")?;

    for node in html.select(&div_selector) {
        if let Some(table) = node.select(&table_selector).next() {
            return Ok(scrape_table(&table.html()));
        }
    }
    Ok(http_dir_entry)
}
