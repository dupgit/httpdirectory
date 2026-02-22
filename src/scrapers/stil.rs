use crate::{
    error::{Result, SelectorResultExt},
    httpdirectoryentry::HttpDirectoryEntry,
};
use scraper::{Html, Selector};
use tracing::trace;

fn extract<'a>(html: &'a str, opening_tag: &str, closing_tag: &str) -> (&'a str, Option<&'a str>) {
    if let Some(opening) = html.find(opening_tag) {
        if let Some(closing) = html.find(closing_tag) {
            if opening <= closing {
                let extract = &html[opening + opening_tag.len()..closing];
                let (_, html) = html.split_at(closing + closing_tag.len());
                (html, Some(extract))
            } else {
                (html, None)
            }
        } else {
            (html, None)
        }
    } else {
        (html, None)
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub(crate) fn scrape_stil(body: &str) -> Result<Vec<HttpDirectoryEntry>> {
    let mut http_dir_entry = vec![];

    let html = Html::parse_document(body);
    let main_selector = Selector::parse("main").with_selector("main")?;
    let main: Vec<_> = html.select(&main_selector).collect();

    let mut file: Option<&str>;
    let name: Option<&str>;
    let ftype: Option<&str>;
    let mut filename: Option<&str>;
    let mut link: Option<&str>;
    let mut dir: Option<&str>;
    let mut date: Option<&str>;
    let mut size: Option<&str>;

    if main.len() == 1 {
        let main_html = main[0].html();
        trace!("main: {main_html}");
        let (_, without_main) = extract(&main_html, "<main>", "</main>");
        let mut html;
        if let Some(without_main_html) = without_main {
            html = without_main_html;
        } else {
            return Ok(http_dir_entry);
        }
        (html, ftype) = extract(html, "<b>", "</b>");
        trace!("type: {:?}", ftype);
        (html, name) = extract(html, "<b>", "</b>");
        trace!("name: {:?}", name);
        (html, date) = extract(html, "<b>", "</b>");
        trace!("date: {:?}", date);
        (html, size) = extract(html, "<b>", "</b>");
        trace!("size: {:?}", size);

        if ftype == Some("Type") && name == Some("Name") && date == Some("Last modified") && size == Some("Size") {
            trace!("This is a Stil header");
        } else {
            return Ok(http_dir_entry);
        }

        while !html.is_empty() {
            let mut is_a_file = false;
            (html, file) = extract(html, r#"<span class="file">"#, "</span>");
            if file == Some("") {
                is_a_file = true;
                trace!("This is a file ");
            }
            if !is_a_file {
                (html, dir) = extract(html, r#"<span class="dir">"#, "</span>");

                if let Some(d) = dir {
                    if !d.is_empty() {
                        // It contains an icon character
                        trace!("This is a directory");
                    }
                } else {
                    trace!("Something is wrong (not a file nor a directory)");
                }
            }
            (html, filename) = extract(html, "<a href=", "</a>");
            let leaves;
            if filename.is_some() {
                (leaves, link) = extract(filename.unwrap(), r#"""#, r#"">"#);
                filename = Some(leaves);
            } else {
                link = None;
            }
            (html, date) = extract(html, "<span>", "</span>");
            (html, size) = extract(html, "<span>", "</span>");
            {
                let filename = filename.unwrap_or_default();
                let link = link.unwrap_or_default();
                let date = date.unwrap_or_default();
                let size = size.unwrap_or_default();
                http_dir_entry.push(HttpDirectoryEntry::new(filename, date, size, link));
            }
        }
    }

    Ok(http_dir_entry)
}
