use crate::{
    error::{Result, SelectorResultExt},
    httpdirectoryentry::HttpDirectoryEntry,
    scrape::{build_entry, get_link_and_name},
};
use scraper::{Html, Selector};
use tracing::debug;

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub(crate) fn scrape_ul(body: &str) -> Result<Vec<HttpDirectoryEntry>> {
    let html = Html::parse_document(body);
    let ul_selector = Selector::parse("ul").with_selector("ul")?;

    let entries = html
        .select(&ul_selector)
        .flat_map(|ul| {
            debug!("{}", ul.html());
            ul.inner_html()
                .lines()
                .map(str::trim)
                .filter_map(|line| {
                    let il = line.split("</li>").next()?.trim();
                    if il.is_empty() {
                        return None;
                    }
                    let name_and_link = il.replace("<li>", "").replace("</a>", "");
                    let (name, link) = get_link_and_name(&name_and_link);
                    debug!("{name} and {link}");

                    // Names ending with '/' are directories others are files
                    let size: &[&str] = if name.ends_with('/') {
                        &[]
                    } else {
                        &[""]
                    };
                    build_entry(&[name], &[], size, link)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Ok(entries)
}
