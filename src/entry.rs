use crate::httpdirectory::Sorting;
use chrono::NaiveDateTime;
use log::{error, trace};
use regex::Regex;
use std::{cmp::Ordering, fmt};

/// Defines an Entry for a file or a directory
#[derive(Debug, Clone)]
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
    if date.len() > 3 {
        // Format to try to parse dates
        let parse_format = [
            "%Y-%m-%d %H:%M",    // 2023-12-03 17:33
            "%d-%b-%Y %H:%M",    // 05-Apr-2024 11:59
            "%Y-%b-%d %H:%M",    // 2021-May-25 20:15
            "%Y-%m-%d %H:%M:%S", // 2023-12-03 17:33:19
            "%d-%b-%Y %H:%M:%S", // 05-Apr-2024 11:59:30
            "%Y-%b-%d %H:%M:%S", // 2021-May-25 20:15:46
            "%m/%d/%Y %r %:z",   // 05/31/2025 01:54:45 PM +00:00
            "%Y-%m-%dT%H:%MZ",   // 2025-10-20T14:17Z
            "%d-%m-%Y | %H:%M",  // 20-10-2025 | 13:52
            "%Y-%m-%d %H:%M %Z", // 2025-10-20 16:17 CEST
        ];

        for pf in parse_format {
            match NaiveDateTime::parse_from_str(date, pf) {
                Ok(d) => {
                    trace!("Successfully parsed date ({date}) with format '{pf}'");
                    return Some(d);
                }
                Err(e) => trace!("Error while parsing date ({date}) with format '{pf}': {e}"),
            }
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
        Some((parsed_date, size))
    } else if !reversed {
        if let Some((parsed_date, _)) = get_date_from_inputs(size, date, true) {
            Some((parsed_date, date))
        } else {
            None
        }
    } else {
        None
    }
}

fn is_bytes(size: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\d*\.?\d*)\s*b|(\d*\.?\d*)\s*ib").unwrap();
    match re.captures(size) {
        Some(value) => {
            if let Some(captured) = value.get(1) {
                let captured = captured.as_str().to_string();
                if captured.is_empty() {
                    None
                } else {
                    Some(captured)
                }
            } else {
                None
            }
        }
        None => None,
    }
}

// @todo: be more accurate with the modifier that should
// be 1000 for Kb and 1024 for KiB ?
fn is_kilo_bytes(size: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\d*\.?\d*)\s*k|(\d*\.?\d*)\s*kb|(\d*\.?\d*)\s*kib").unwrap();
    match re.captures(size) {
        Some(value) => Some(value[1].to_string()),
        None => None,
    }
}

fn is_mega_bytes(size: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\d*\.?\d*)\s*m|(\d*\.?\d*)\s*mb|(\d*\.?\d*)\s*mib").unwrap();
    match re.captures(size) {
        Some(value) => Some(value[1].to_string()),
        None => None,
    }
}

fn is_giga_bytes(size: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\d*\.?\d*)\s*g|(\d*\.?\d*)\s*gb|(\d*\.?\d*)\s*gib").unwrap();
    match re.captures(size) {
        Some(value) => Some(value[1].to_string()),
        None => None,
    }
}

fn is_tera_bytes(size: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\d*\.?\d*)\s*t|(\d*\.?\d*)\s*tb|(\d*\.?\d*)\s*tib").unwrap();
    match re.captures(size) {
        Some(value) => Some(value[1].to_string()),
        None => None,
    }
}

fn is_peta_bytes(size: &str) -> Option<String> {
    let re = Regex::new(r"(?i)(\d*\.?\d*)\s*p|(\d*\.?\d*)\s*pb|(\d*\.?\d*)\s*pib").unwrap();
    match re.captures(size) {
        Some(value) => Some(value[1].to_string()),
        None => None,
    }
}

impl Entry {
    /// Creates a new Entry
    #[must_use]
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
        trace!("name: {name}, date: {date:?}, size: {guessed_size}, link: {link}");

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
    /// In case the size is greater than `usize::MAX`
    /// it may be truncated to that value
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn apparent_size(&self) -> usize {
        let real_size: usize;
        let new_size;
        let my_size = self.size().to_lowercase();
        if my_size.contains('-') {
            // Directory
            real_size = 0;
            new_size = my_size.to_string();
        } else if let Some(captured_size) = is_bytes(&my_size) {
            trace!("Bytes detected: {captured_size}");
            real_size = 1;
            new_size = captured_size;
        } else if let Some(captured_size) = is_kilo_bytes(&my_size) {
            trace!("Kilo bytes detected: {captured_size}");
            real_size = 1024;
            new_size = captured_size;
        } else if let Some(captured_size) = is_mega_bytes(&my_size) {
            trace!("Mega bytes detected: {captured_size}");
            real_size = 1_048_576;
            new_size = captured_size;
        } else if let Some(captured_size) = is_giga_bytes(&my_size) {
            trace!("Giga bytes detected: {captured_size}");
            real_size = 1_073_741_824;
            new_size = captured_size;
        } else if let Some(captured_size) = is_tera_bytes(&my_size) {
            trace!("Tera bytes detected: {captured_size}");
            real_size = 1_099_511_627_776;
            new_size = captured_size;
        } else if let Some(captured_size) = is_peta_bytes(&my_size) {
            trace!("Peta bytes detected: {captured_size}");
            real_size = 1_125_899_906_842_624;
            new_size = captured_size;
        } else {
            // size may not have any modifier and be expressed
            // directly in bytes
            real_size = 1;
            new_size = my_size.to_string();
        }

        let new_size = new_size.trim();
        if my_size.contains('.') {
            match new_size.parse::<f64>() {
                Ok(number) => {
                    if number.signum().is_finite()
                        && number < 18_446_744_073_709_551_615.0
                        && number > -18_446_744_073_709_551_615.0
                    {
                        // number is not Nan nor ∞
                        // We know that .abs() will return a positive value
                        // if number is greater than `usize::MAX` then number
                        // is truncated to usize::MAX
                        real_size * ((number.abs() * 10.0) as usize) / 10
                    } else {
                        0
                    }
                }
                Err(e) => {
                    error!("error parsing {new_size} into usize: {e}");
                    0
                }
            }
        } else {
            match new_size.parse::<usize>() {
                Ok(number) => real_size * number,
                Err(e) => {
                    error!("error parsing {new_size} into usize: {e}");
                    0
                }
            }
        }
    }

    /// Returns the size of the Entry as an &str.
    /// It may contain a number or ' - ' if the entry is a directory.
    /// The number may be followed by K, M, G, T or P.
    /// use `apparent_size()` method to get the size of the file
    /// as a usize number.
    #[must_use]
    pub fn size(&self) -> &str {
        &self.size
    }

    // Returns the name of the file or directory
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn date(&self) -> Option<NaiveDateTime> {
        self.date
    }

    #[must_use]
    pub fn cmp_by_name(&self, other: &Self, order: &Sorting) -> Ordering {
        match order {
            Sorting::Ascending => self.name.cmp(&other.name),
            Sorting::Descending => other.name.cmp(&self.name),
        }
    }

    #[must_use]
    pub fn cmp_by_date(&self, other: &Self, order: &Sorting) -> Ordering {
        match order {
            Sorting::Ascending => self.date.cmp(&other.date),
            Sorting::Descending => other.date.cmp(&self.date),
        }
    }

    #[must_use]
    pub fn cmp_by_size(&self, other: &Self, order: &Sorting) -> Ordering {
        match order {
            Sorting::Ascending => self.apparent_size().cmp(&other.apparent_size()),
            Sorting::Descending => other.apparent_size().cmp(&self.apparent_size()),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.date {
            Some(date) => write!(f, "{:>8}  {}  {}", self.size, date.format("%Y-%m-%d %H:%M"), self.name),
            None => write!(f, "{:>8}  {:>16}  {}", self.size, "", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::Entry, crate::httpdirectory::Sorting, std::cmp::Ordering};
    #[test]
    fn test_apparent_size_float() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "5.0K");
        assert_eq!(entry.apparent_size(), 5120);

        let entry = Entry::new("name", "link", "2025-05-20 20:19", "5.3k");
        assert_eq!(entry.apparent_size(), 5427);
    }

    #[test]
    fn test_apparent_size_infinite() {
        let entry = Entry::new(
            "name",
            "link",
            "2025-05-20 20:19",
            "999999999999999999999999999999999999999999999999999999999.0P",
        );

        assert_eq!(entry.apparent_size(), 0);
    }

    #[test]
    fn test_entry_output() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "5.0K");
        let output = format!("{entry}");
        assert_eq!(output, "    5.0K  2025-05-20 20:19  name");
    }

    #[test]
    fn test_apparent_size_usize() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "524");

        assert_eq!(entry.apparent_size(), 524);
    }

    #[test]
    fn test_apparent_size_modifier_t() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1G");

        assert_eq!(entry.apparent_size(), 1_073_741_824);
    }

    #[test]
    fn test_apparent_size_modifier_p() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1P");

        assert_eq!(entry.apparent_size(), 1_125_899_906_842_624);
    }

    #[test]
    fn test_apparent_size_zero() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "0");

        assert_eq!(entry.apparent_size(), 0);
    }

    #[test]
    fn test_apparent_size_directory() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", " - ");

        assert_eq!(entry.apparent_size(), 0);
    }

    #[test]
    fn test_apparent_size_wrong_input() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "Not_A_Size");

        assert_eq!(entry.apparent_size(), 0);
    }

    #[test]
    fn test_apparent_size_wrong_input_with_modifier() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "Not_A_SizeT");

        assert_eq!(entry.apparent_size(), 0);
    }

    #[test]
    fn test_wrong_date_format_inverted_with_size() {
        let entry = Entry::new("name", "link", "12.0K", "05-2025-20 20:19");

        assert_eq!(entry.date, None);
    }

    #[test]
    fn test_wrong_date_format() {
        let entry = Entry::new("name", "link", "05-2025-20 20:19", "12.0K");

        assert_eq!(entry.date, None);
    }

    #[test]
    fn test_entry_output_wrong_date_format() {
        let entry = Entry::new("name", "link", "05-2025-20 20:19", "12.0K");
        let output = format!("{entry}");
        assert_eq!(output, "   12.0K                    name");
    }

    #[test]
    fn test_cmp_by_name() {
        let entry1 = Entry::new("name", "link", "2025-05-20 20:19", "112");
        let entry2 = Entry::new("othername", "link", "2025-05-20 20:19", "112");

        assert_eq!(entry1.cmp_by_name(&entry2, &Sorting::Ascending), Ordering::Less);
        assert_eq!(entry2.cmp_by_name(&entry1, &Sorting::Ascending), Ordering::Greater);
        assert_eq!(entry1.cmp_by_name(&entry2, &Sorting::Descending), Ordering::Greater);
        assert_eq!(entry2.cmp_by_name(&entry1, &Sorting::Descending), Ordering::Less);
    }

    #[test]
    fn test_cmp_by_date() {
        let entry1 = Entry::new("name", "link", "2025-05-21 03:45", "112");
        let entry2 = Entry::new("othername", "link", "2025-05-20 20:19", "112");

        assert_eq!(entry1.cmp_by_date(&entry2, &Sorting::Ascending), Ordering::Greater);
        assert_eq!(entry2.cmp_by_date(&entry1, &Sorting::Ascending), Ordering::Less);
        assert_eq!(entry1.cmp_by_date(&entry2, &Sorting::Descending), Ordering::Less);
        assert_eq!(entry2.cmp_by_date(&entry1, &Sorting::Descending), Ordering::Greater);
    }

    #[test]
    fn test_cmp_by_size() {
        let entry1 = Entry::new("name", "link", "2025-05-21 03:45", "4.0k");
        let entry2 = Entry::new("othername", "link", "2025-05-20 20:19", "112");

        assert_eq!(entry1.cmp_by_size(&entry2, &Sorting::Ascending), Ordering::Greater);
        assert_eq!(entry2.cmp_by_size(&entry1, &Sorting::Ascending), Ordering::Less);
        assert_eq!(entry1.cmp_by_size(&entry2, &Sorting::Descending), Ordering::Less);
        assert_eq!(entry2.cmp_by_size(&entry1, &Sorting::Descending), Ordering::Greater);
    }
}
