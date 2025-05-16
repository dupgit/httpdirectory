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
    date: Option<NaiveDateTime>,

    /// Apparent size as reported by the HTTP page
    size: String,
}

// Tries to parse a string that should contain a date
// with an array of known formats
fn try_parse_date(date: &str) -> Option<NaiveDateTime> {
    // formats that respectively parses 2023-12-03 17:33, 05-Apr-2024 11:59, 2021-May-25 20:15
    let parse_format = ["%Y-%m-%d %H:%M", "%d-%b-%Y %H:%M", "%Y-%b-%d %H:%M"];

    for pf in parse_format {
        match NaiveDateTime::parse_from_str(date, pf) {
            Ok(d) => {
                trace!("Successfully parsed date ({date}) with format '{pf}'");
                return Some(d);
            }
            Err(e) => trace!("Error while parsing date ({date}) with format '{pf}': {e}"),
        }
    }
    None
}

// Tries to parse a date within date or size fields that
// may be in reverse order in the html page
// reversed is true when we are already testing size instead of date
// so there is no need to reverse twice.
fn get_date_from_inputs<'a>(date: &'a str, size: &'a str, reversed: bool) -> Option<(NaiveDateTime, &'a str)> {
    if let Some(parsed_date) = try_parse_date(date) {
        return Some((parsed_date, size));
    } else if !reversed {
        if let Some((parsed_date, _)) = get_date_from_inputs(size, date, true) {
            return Some((parsed_date, date));
        } else {
            None
        }
    } else {
        None
    }
}

impl Entry {
    // todo: Manage Results and Options !
    pub fn new(name: &str, link: &str, date: &str, size: &str) -> Self {
        trace!("name: {name}, date: {date}, size: {size}, link: {link}");
        let name = name.to_string();
        let link = link.to_string();
        let ndt_date: Option<NaiveDateTime>;
        let guessed_size: &str;

        if let Some((date, parsed_size)) = get_date_from_inputs(date, size, false) {
            ndt_date = Some(date);
            guessed_size = parsed_size;
        } else {
            ndt_date = None;
            guessed_size = size; // size here is assumed to be "correct" somehow
        }
        let date = ndt_date;

        Entry {
            name,
            link,
            date,
            size: guessed_size.to_string(),
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

    pub fn size(&self) -> &str {
        &self.size
    }

    // Returns the name of the file or directory
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> Option<NaiveDateTime> {
        self.date
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.date {
            Some(date) => write!(f, "{:>5}  {}  {}", self.size, date.format("%Y-%m-%d %H:%M"), self.name),
            None => write!(f, "{:>5}  {:>16}  {}", self.size, "", self.name),
        }
    }
}
