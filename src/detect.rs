use regex::Regex;
use std::sync::LazyLock;
use unwrap_unreachable::UnwrapUnreachable;

/// Site type enumeration.
#[derive(Debug, PartialEq, Eq)]
pub enum SiteType {
    NotNamed(PureHtml),
    H5ai(String),      // from https://github.com/lrsjng/h5ai
    Snt,               // SNT index generator from https://snt.utwente.nl/en/
    MiniServe(String), // Miniserv file server from https://crates.io/crates/miniserve
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PureHtml {
    Table,
    Pre,
    Ul,
}

// Regex used to determine `SiteType`
static H5AI_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"powered by h5ai (v\d+.\d+.\d+)").unreachable());

// Some websites are using "Modified" instead of "Last modified"
static TABLE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?msi)<table(.+?<th.+?(Last )?modified.+?</th.+?)</table").unreachable());

// Some other websites are using "Date" instead of "Last modified" or "Modified"
static TABLE_DATE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?msi)<table(.+?<th.+?Date.+?</th.+?)</table").unreachable());

static MINISERVE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<div class="version"><a href="https://github.com/svenstaro/miniserve">miniserve</a>/(\d+.\d+.\d+)</div>"#,
    )
    .unreachable()
});

// <table> detection is considered valid if
// we can match a column name "Modified",
// "Last Modified" or "Date" within the table
#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn detect_table(body: &str) -> bool {
    if TABLE_RE.is_match(body) {
        true
    } else {
        TABLE_DATE_RE.is_match(body)
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn detect_h5ai(body: &str) -> Option<String> {
    if body.contains("powered by h5ai") {
        H5AI_RE.captures(body).map(|value| value[1].to_string())
    } else {
        None
    }
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn detect_snt(body: &str) -> bool {
    body.contains("SNT index generator")
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn detect_miniserve(body: &str) -> Option<String> {
    if body.contains(r#"<div class="version"><a href="https://github.com/svenstaro/miniserve">miniserve</a>/"#) {
        MINISERVE_RE.captures(body).map(|value| value[1].to_string())
    } else {
        None
    }
}

impl SiteType {
    /// Detects the possible type of the site we are
    /// scraping information from by "analyzing" it's
    /// body.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn detect(body: &str) -> Self {
        if let Some(version) = detect_h5ai(body) {
            SiteType::H5ai(version)
        } else if detect_snt(body) {
            SiteType::Snt
        } else if let Some(version) = detect_miniserve(body) {
            SiteType::MiniServe(version)
        } else if detect_table(body) {
            SiteType::NotNamed(PureHtml::Table)
        } else if body.contains("<pre>") {
            SiteType::NotNamed(PureHtml::Pre)
        } else if body.contains("<ul>") {
            SiteType::NotNamed(PureHtml::Ul)
        } else {
            SiteType::None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::{PureHtml, SiteType};

    #[test]
    fn test_empty_body() {
        assert_eq!(SiteType::detect(""), SiteType::None);
    }

    #[test]
    fn test_body_with_table() {
        let body = r#"
              <table id="indexlist">
               <tr class="indexhead"><th class="indexcolicon"><img src="/icons2/blank.png" alt="[ICO]"></th><th class="indexcolname"><a href="?C=N;O=D">Name</a></th><th class="indexcollastmod"><a href="?C=M;O=A">Last modified</a></th><th class="indexcolsize"><a href="?C=S;O=A">Size</a></th></tr>
               <tr class="indexbreakrow"><th colspan="4"><hr></th></tr>
               <tr class="even"><td class="indexcolicon"><a href="/images/"><img src="/icons2/go-previous.png" alt="[PARENTDIR]"></a></td><td class="indexcolname"><a href="/images/">Parent Directory</a></td><td class="indexcollastmod">&nbsp;</td><td class="indexcolsize">  - </td></tr>
               <tr class="odd"><td class="indexcolicon"><a href="OpenStack/"><img src="/icons2/folder.png" alt="[DIR]"></a></td><td class="indexcolname"><a href="OpenStack/">OpenStack/</a></td><td class="indexcollastmod">2024-07-01 23:19  </td><td class="indexcolsize">  - </td></tr>
              </table>
            "#;

        assert_eq!(SiteType::detect(body), SiteType::NotNamed(PureHtml::Table));
    }

    #[test]
    fn test_body_with_pre() {
        let body = r##"
            <h1>Index of /pub/OpenBSD/</h1><hr><pre><a href="../">../</a>
            <a href="7.5/">7.5/</a>                                               05-Apr-2024 11:59                   -
            <a href="7.6/">7.6/</a>                                               08-Oct-2024 17:17                   -
            <a href="7.7/">7.7/</a>                                               27-Apr-2025 17:58                   -
            <a href="Changelogs/">Changelogs/</a>                                        12-May-2025 17:21                   -
            <a href="LibreSSL/">LibreSSL/</a>                                          30-Apr-2025 06:55                   -
            <a href="OpenBGPD/">OpenBGPD/</a>                                          06-Feb-2025 15:30                   -
            <a href="OpenIKED/">OpenIKED/</a>                                          10-Apr-2025 17:10                   -
            <a href="OpenNTPD/">OpenNTPD/</a>                                          09-Dec-2020 14:56                   -
            <a href="OpenSSH/">OpenSSH/</a>                                           09-Apr-2025 07:08                   -
            <a href="doc/">doc/</a>                                               28-Apr-2013 15:57                   -
            <a href="patches/">patches/</a>                                           04-May-2025 21:25                   -
            <a href="rpki-client/">rpki-client/</a>                                       11-Apr-2025 22:09                   -
            <a href="signify/">signify/</a>                                           06-May-2025 15:03                   -
            <a href="snapshots/">snapshots/</a>                                         13-May-2025 04:06                   -
            <a href="songs/">songs/</a>                                             06-Apr-2023 22:15                   -
            <a href="stable/">stable/</a>                                            18-Jan-2022 16:25                   -
            <a href="syspatch/">syspatch/</a>                                          03-Mar-2025 15:17                   -
            <a href="tools/">tools/</a>                                             07-Jan-2005 19:40                   -
            <a href="README">README</a>                                             06-Oct-2017 11:51                1329
            <a href="ftplist">ftplist</a>                                            13-May-2025 03:57                4836
            <a href="timestamp">timestamp</a>                                          13-May-2025 04:00                  11
            </pre><hr>
            "##;

        assert_eq!(SiteType::detect(body), SiteType::NotNamed(PureHtml::Pre));
    }
}
