use crate::{error::HttpDirError, httpdirectoryentry::HttpDirectoryEntry, scrape::get_link_and_name};
use log::debug;
use scraper::{Html, Selector};

pub(crate) fn scrape_ul(body: &str) -> Result<Vec<HttpDirectoryEntry>, HttpDirError> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let ul_selector = Selector::parse("ul")?;
    for ul in html.select(&ul_selector) {
        debug!("{}", ul.html());
        for line in ul.inner_html().lines() {
            let il = line.split("</li>").collect::<Vec<&str>>().into_iter().map(str::trim).collect::<Vec<&str>>();
            if !il.is_empty() && !il[0].is_empty() {
                let name_and_link = il[0].replace("<li>", "").replace("</a>", "");
                let (name, link) = get_link_and_name(&name_and_link);
                // Names that finishes with a trailing / are directories others are files
                if let Some(last) = name.chars().last() {
                    if last == '/' {
                        if link.to_lowercase() == "parent directory" {
                            http_dir_entry.push(HttpDirectoryEntry::new(name, "", " - ", link));
                        } else {
                            http_dir_entry.push(HttpDirectoryEntry::new(name, "", " - ", link));
                        }
                    } else {
                        http_dir_entry.push(HttpDirectoryEntry::new(name, "", "", link));
                    }
                }

                debug!("{name} and {link}");
            }
        }
        // for il in ul.select(&il_selector) {
        //     debug!("{il:?}");
        // }
    }
    Ok(http_dir_entry)
}
