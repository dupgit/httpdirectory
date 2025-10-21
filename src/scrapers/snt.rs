use crate::{error::HttpDirError, httpdirectoryentry::HttpDirectoryEntry, scrape::scrape_table};
use log::trace;
use scraper::{Html, Selector};

pub(crate) fn scrape_snt(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);

    // SNT websites separates directories and files into
    // two different tags: <nav> for a navigation bar that
    // keeps all directories and <article> for all the files

    let nav_selector = Selector::parse("nav")?;
    for nav in html.select(&nav_selector) {
        if nav.inner_html().contains("Directories") {
            let a_selector = Selector::parse("a")?;
            for name_and_link in nav.select(&a_selector) {
                let link = name_and_link.value().attr("href").unwrap_or_default();
                let name = name_and_link.text().collect::<String>().trim().to_string();
                trace!("New directory: {name}, {link}");
                http_dir_entry.push(HttpDirectoryEntry::new(&name, "", " - ", link));
            }
        }
    }

    let article_selector = Selector::parse("article")?;
    let table_selector = Selector::parse("table")?;
    for article in html.select(&article_selector) {
        if let Some(table) = article.select(&table_selector).next() {
            if let Ok(file_http_dir_entries) = scrape_table(&table.html()) {
                for file in file_http_dir_entries {
                    trace!("New file: {file:?}");
                    http_dir_entry.push(file);
                }
            }
        }
    }

    Ok(http_dir_entry)
}
