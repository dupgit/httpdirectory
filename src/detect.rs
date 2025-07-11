/// Site type enumeration.
#[derive(Debug, PartialEq, Eq)]
pub enum SiteType {
    Table,
    Pre,
    None,
}

impl SiteType {
    /// Detects the possible type of the site we are
    /// scraping information from by "analyzing" it's
    /// body.
    pub fn detect(body: &str) -> Self {
        // `<table>` tag may contain attributes such as
        // `id="indexlist"` for instance so we need to
        // search without the closing `>` tag sign
        if body.contains("<table") {
            SiteType::Table
        } else if body.contains("<pre>") {
            SiteType::Pre
        } else {
            SiteType::None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::detect::SiteType;

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

        assert_eq!(SiteType::detect(body), SiteType::Table);
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

        assert_eq!(SiteType::detect(body), SiteType::Pre);
    }
}
