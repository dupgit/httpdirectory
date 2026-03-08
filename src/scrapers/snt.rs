use crate::{
    error::{Result, SelectorResultExt},
    httpdirectoryentry::HttpDirectoryEntry,
    scrape::scrape_table,
};
use scraper::{Html, Selector};
use tracing::trace;

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub(crate) fn scrape_snt(body: &str) -> Result<Vec<HttpDirectoryEntry>> {
    let html = Html::parse_document(body);

    let nav_selector = Selector::parse("nav").with_selector("nav")?;
    let a_selector = Selector::parse("a").with_selector("a")?;
    let article_selector = Selector::parse("article").with_selector("article")?;
    let table_selector = Selector::parse("table").with_selector("table")?;

    // SNT websites separates directories and files into
    // two different tags: <nav> for a navigation bar that
    // keeps all directories and <article> for all the files

    // Directories from <nav>
    let dirs = html
        .select(&nav_selector)
        .filter(|nav| nav.inner_html().contains("Directories"))
        .flat_map(|nav| nav.select(&a_selector).collect::<Vec<_>>())
        .map(|name_and_link| {
            let link = name_and_link.value().attr("href").unwrap_or_default();
            let name = name_and_link.text().collect::<String>();
            let name = name.trim();
            trace!("New directory: {name}, {link}");
            HttpDirectoryEntry::new(name, "", " - ", link)
        });

    // Files from <article>
    let files =
        html.select(&article_selector).filter_map(|article| article.select(&table_selector).next()).flat_map(|table| {
            let entries = scrape_table(&table.html());
            trace!("New files: {entries:?}");
            entries
        });

    Ok(dirs.chain(files).collect())
}
