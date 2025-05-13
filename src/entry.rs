use chrono::{NaiveDate, NaiveDateTime, NaiveTime, format::ParseErrorKind};
use log::{error, trace};
use std::fmt;

/// Defines an Entry for a file or a directory
#[derive(Debug)]
pub struct Entry {
    /// Name of file or directory
    name: String,

    /// Link to that file or directory (Generally identical to name)
    link: String,

    /// Date of that file or directory
    date: NaiveDateTime,

    /// Apparent size as reported by the HTTP page
    size: String,
}

impl Entry {
    // todo: Manage Results and Options !
    pub fn new(name: &str, link: &str, date: &str, size: &str) -> Self {
        trace!("name: {name}, date: {date}, size: {size}, link: {link}");
        let name = name.to_string();
        let link = link.to_string();
        let ndt_date: NaiveDateTime;

        // Trying ISO like format (2023-12-03 17:33)
        if let Ok(ndt) = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M") {
            ndt_date = ndt;
        } else {
            // Trying with abbreviated month name (05-Apr-2024 11:59)
            ndt_date = match NaiveDateTime::parse_from_str(date, "%d-%b-%Y %H:%M") {
                Ok(ndt) => ndt,
                Err(e) => {
                    error!("Error while parsing date: {:?}. Using 1970-01-01 08:00", e.kind());
                    let d = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                    let t = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
                    NaiveDateTime::new(d, t)
                }
            };
        }
        let date = ndt_date;

        Entry {
            name,
            link,
            date,
            size: size.to_string(),
        }
    }

    /// Returns the apparent size as a usize number.
    /// It is not an accurate size as 42K results in
    /// 42 * 1024 = 43008 (the real size in bytes may
    /// be a bit greater or a bit lower to this)
    pub fn apparent_size(&self) -> usize {
        let real_size: usize;
        let new_size;
        if self.size.contains('-') {
            // Directory
            real_size = 0;
            new_size = self.size.to_string();
        } else if self.size.contains('K') {
            real_size = 1024;
            new_size = self.size.replace('K', "");
        } else if self.size.contains('M') {
            real_size = 1_048_576;
            new_size = self.size.replace('M', "");
        } else if self.size.contains('G') {
            real_size = 1_073_741_824;
            new_size = self.size.replace('G', "");
        } else if self.size.contains('T') {
            real_size = 1_099_511_627_776;
            new_size = self.size.replace('T', "");
        } else if self.size.contains('P') {
            real_size = 1_125_899_906_842_624;
            new_size = self.size.replace('P', "");
        } else {
            // size may not have any modifier and be expressed
            // directly in bytes
            real_size = 1;
            new_size = self.size.to_string();
        }

        match new_size.parse::<usize>() {
            Ok(number) => real_size * number,
            Err(_) => 0,
        }
    }

    // Returns the name of the file or directory
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> NaiveDateTime {
        self.date
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>5}  {}  {}", self.size, self.date.format("%Y-%m-%d %H:%M"), self.name)
    }
}
