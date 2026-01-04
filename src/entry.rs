use chrono::NaiveDateTime;
use regex::Regex;
use std::sync::LazyLock;
use std::{cmp::Ordering, fmt};
use tracing::{error, trace};
use unwrap_unreachable::UnwrapUnreachable;

/// Defines an Entry for a file or a directory
#[derive(Debug, Clone)]
pub struct Entry {
    /// Name of file or directory
    name: String,

    /// Link to that file or directory (Generally identical to name)
    link: String,

    /// Date of that file or directory
    date: Option<NaiveDateTime>,

    /// Apparent size as reported by the HTTP page used for printing
    apparent_size: String,

    /// Computed size used for sorting
    size: usize,
}

// Direct capture of the size as a number and the unit (modifier)
// using capture groups. There are 5 capture groups in that regex.
// It does not capture the unit multiplier type if any (ie: i) for
// now. See `capture_size_and_unit()` method.
static SIZE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(\d*\.?\d*)\s*([kmgtp])i?b|(\d*\.?\d*)\s*([kmgtp]|b)|(\d*\.?\d*)").unreachable());

// Tries to parse a string that should contain a date
// with an array of known formats
fn try_parse_date(date: &str) -> Option<NaiveDateTime> {
    if date.len() > 3 && date.contains(':') {
        // Format to try to parse dates
        let parse_format = [
            "%Y-%m-%d %H:%M",       // 2023-12-03 17:33
            "%d-%b-%Y %H:%M",       // 05-Apr-2024 11:59
            "%Y-%b-%d %H:%M",       // 2021-May-25 20:15
            "%Y-%m-%d %H:%M:%S",    // 2023-12-03 17:33:19
            "%d-%b-%Y %H:%M:%S",    // 05-Apr-2024 11:59:30
            "%Y-%b-%d %H:%M:%S",    // 2021-May-25 20:15:46
            "%Y/%m/%d %H:%M:%S",    // 2025/10/21 21:53:58
            "%m/%d/%Y %r %:z",      // 05/31/2025 01:54:45 PM +00:00
            "%Y-%m-%dT%H:%MZ",      // 2025-10-20T14:17Z
            "%d-%m-%Y | %H:%M",     // 20-10-2025 | 13:52
            "%Y-%m-%d %H:%M %Z",    // 2025-10-20 16:17 CEST
            "%Y-%m-%d %H:%M:%S %Z", // 2025-09-06 18:15:23 CST
            "%B %d, %Y %H:%M",      // October 21, 2025 20:53
            "%d %b %Y %H:%M:%S %z", // 06 Sep 2025 10:15:23 +0000
            "%d-%m-%Y %H:%M",       // 21-10-2025 14:19
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
#[cfg_attr(feature = "hotpath", hotpath::measure)]
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

// @todo: be more accurate with the modifier that should
// be 1000 for Kb and 1024 for KiB ?
// This will never fail because of the regex. It may return None
// when `size` string does not contain any number and the captured
// number with unit 1 in case it could not capture any unit modifier
// ie: wrong units will not be detected and answer may be wrong.
// @todo: do we need to detect such error ?
#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn capture_size_and_unit(size: &str) -> Option<(String, usize)> {
    trace!("To be captured: {size}");

    let captured_modifier: usize;
    let captured_size: String;

    if let Some(value) = SIZE_RE.captures(size) {
        let match_cap: usize;
        trace!("Captured some value: {value:?}");
        if value.get(1).is_some() {
            match_cap = 1;
        } else if value.get(3).is_some() {
            match_cap = 3;
        } else if value.get(5).is_some() {
            match_cap = 5;
        } else {
            return None;
        }

        trace!("match group: {match_cap}");
        if let Some(modifier) = value.get(match_cap + 1) {
            captured_modifier = match modifier.as_str().chars().next() {
                Some('b' | 'B') => 1,
                Some('k' | 'K') => 1024,
                Some('m' | 'M') => 1_048_576,
                Some('g' | 'G') => 1_073_741_824,
                Some('t' | 'T') => 1_099_511_627_776,
                Some('p' | 'P') => 1_125_899_906_842_624,
                _ => 0,
            };
        } else {
            captured_modifier = 1;
        }
        trace!("modifier: {captured_modifier}");
        if let Some(captured) = value.get(match_cap) {
            captured_size = captured.as_str().to_string();
            trace!("size: {captured_size}");
            if captured_size.is_empty() {
                None
            } else {
                Some((captured_size, captured_modifier))
            }
        } else {
            None
        }
    } else {
        trace!("Could not capture anything");
        None
    }
}

// new_size must contain a number with a dot ie: 423.3
// real_size contains the value of a modifier: k or Kib stands for 1024 bytes
// see capture_size_and_unit() function above.
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
fn parse_float_to_usize(new_size: &str, real_size: usize) -> usize {
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
                return real_size * ((number.abs() * 10.0) as usize) / 10;
            }
            0
        }
        Err(e) => {
            error!("error parsing '{new_size}' into usize: {e}");
            0
        }
    }
}

fn parse_to_usize(new_size: &str, real_size: usize) -> usize {
    match new_size.parse::<usize>() {
        Ok(number) => real_size * number,
        Err(e) => {
            error!("error parsing '{new_size}' into usize: {e}");
            0
        }
    }
}

/// Returns the apparent size as a usize number.
/// It is not an accurate size as 42K results in
/// 42 * 1024 = 43008 (the real size in bytes may
/// be a bit greater or a bit lower to this)
/// In case the size is greater than `usize::MAX`
/// it may be truncated to that value
#[must_use]
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn apparent_size(size: &str) -> usize {
    // Shortly determine if size is from a directory
    if size.contains('-') {
        return 0;
    }

    let real_size: usize;
    let new_size: String;

    match capture_size_and_unit(&size.to_lowercase()) {
        Some((captured_size, captured_modifier)) => {
            trace!("Detected size: {captured_size}, modifier: {captured_modifier}");
            real_size = captured_modifier;
            new_size = captured_size.trim().to_string();
        }
        None => {
            return 0;
        }
    }

    if new_size.contains('.') {
        return parse_float_to_usize(&new_size, real_size);
    } else if !new_size.is_empty() {
        return parse_to_usize(&new_size, real_size);
    }

    0
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
            apparent_size: guessed_size.to_string(),
            size: apparent_size(guessed_size),
        }
    }

    /// Returns the size of the Entry as an &str as read on the
    /// original website. It may contain a number or ' - ' if
    /// the entry is a directory.
    /// The number may be followed by units ie: K, M, G, T or P.
    #[must_use]
    pub fn apparent_size(&self) -> &str {
        &self.apparent_size
    }

    /// Gets the size of the file as a usize number. This size has
    /// been guessed and calculated when parsing each line of data
    /// scraping the website.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the name of the file or directory
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the link of the file or directory
    #[must_use]
    pub fn link(&self) -> &str {
        &self.link
    }

    /// Returns the date of the file or directory
    #[must_use]
    pub fn date(&self) -> Option<NaiveDateTime> {
        self.date
    }

    /// Compares two `Entry` by name and returns an `Ordering`
    #[must_use]
    pub fn cmp_by_name(&self, other: &Self, ascending: bool) -> Ordering {
        if ascending {
            self.name.cmp(&other.name)
        } else {
            other.name.cmp(&self.name)
        }
    }

    /// Compares two `Entry` by date and returns an `Ordering`
    #[must_use]
    pub fn cmp_by_date(&self, other: &Self, ascending: bool) -> Ordering {
        if ascending {
            self.date.cmp(&other.date)
        } else {
            other.date.cmp(&self.date)
        }
    }

    /// Compares two `Entry` by size and returns an `Ordering`
    #[must_use]
    pub fn cmp_by_size(&self, other: &Self, ascending: bool) -> Ordering {
        if ascending {
            self.size.cmp(&other.size)
        } else {
            other.size.cmp(&self.size)
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.date {
            Some(date) => write!(f, "{:>9}  {}  {}", self.apparent_size, date.format("%Y-%m-%d %H:%M"), self.name),
            None => write!(f, "{:>9}  {:>16}  {}", self.apparent_size, "", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::Entry, std::cmp::Ordering, unwrap_unreachable::UnwrapUnreachable};
    #[test]
    fn test_apparent_size_float() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "5.0K");
        assert_eq!(entry.size, 5120);

        let entry = Entry::new("name", "link", "2025-05-20 20:19", "5.3k");
        assert_eq!(entry.size, 5427);
    }

    #[test]
    fn test_apparent_size_infinite() {
        let entry = Entry::new(
            "name",
            "link",
            "2025-05-20 20:19",
            "999999999999999999999999999999999999999999999999999999999.0P",
        );

        assert_eq!(entry.size, 0);
    }

    #[test]
    fn test_entry_output() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "5.0K");
        let output = format!("{entry}");
        assert_eq!(output, "     5.0K  2025-05-20 20:19  name");
    }

    #[test]
    fn test_apparent_size_usize() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "524");

        assert_eq!(entry.size, 524);
    }

    #[test]
    fn test_apparent_size_modifier_t() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1t");

        assert_eq!(entry.size, 1_099_511_627_776);
    }

    #[test]
    fn test_apparent_size_modifier_p() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1P");

        assert_eq!(entry.size, 1_125_899_906_842_624);
    }

    #[test]
    fn test_apparent_size_modifier_tb() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1tB");

        assert_eq!(entry.size, 1_099_511_627_776);
    }

    #[test]
    fn test_apparent_size_modifier_pb() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1Pb");

        assert_eq!(entry.size, 1_125_899_906_842_624);
    }

    #[test]
    fn test_apparent_size_modifier_tib() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1tiB");

        assert_eq!(entry.size, 1_099_511_627_776);
    }

    #[test]
    fn test_apparent_size_modifier_pib() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1Pib");

        assert_eq!(entry.size, 1_125_899_906_842_624);
    }

    #[test]
    fn test_apparent_size_modifier_tib_with_space() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1 TiB");

        assert_eq!(entry.size, 1_099_511_627_776);
    }

    #[test]
    fn test_apparent_size_modifier_pib_with_space() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "1 PiB");

        assert_eq!(entry.size, 1_125_899_906_842_624);
    }

    #[test]
    fn test_apparent_size_wrong_modifier() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "4 ÀiB");

        assert_eq!(entry.size, 4);
    }

    #[test]
    fn test_apparent_size_zero() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "0");

        assert_eq!(entry.size, 0);
    }

    #[test]
    fn test_apparent_size_directory() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", " - ");

        assert_eq!(entry.size, 0);
    }

    #[test]
    fn test_apparent_size_wrong_input() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "Not_A_Size");

        assert_eq!(entry.size, 0);
    }

    #[test]
    fn test_capture_size_and_unit() {
        use crate::entry::capture_size_and_unit;
        if let Some((size, unit)) = capture_size_and_unit("12 Kib") {
            assert_eq!(size, "12".to_string());
            assert_eq!(unit, 1024);
        }
    }

    #[test]
    fn test_capture_empty_size_and_unit() {
        use crate::entry::capture_size_and_unit;
        if let Some((size, unit)) = capture_size_and_unit("") {
            panic!("This test should return None. We got {size} and {unit} !");
        }
    }

    #[test]
    fn test_capture_wrong_size_and_unit() {
        use crate::entry::capture_size_and_unit;
        if let Some((size, unit)) = capture_size_and_unit("Not_A_Size") {
            panic!("This test should return None. We got {size} and {unit} !");
        }
    }

    #[test]
    fn test_apparent_size_wrong_input_with_modifier() {
        let entry = Entry::new("name", "link", "2025-05-20 20:19", "Not_A_SizeT");

        assert_eq!(entry.size, 0);
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
        assert_eq!(output, "    12.0K                    name");
    }

    #[test]
    fn test_cmp_by_name() {
        let entry1 = Entry::new("name", "link", "2025-05-20 20:19", "112");
        let entry2 = Entry::new("othername", "link", "2025-05-20 20:19", "112");

        assert_eq!(entry1.cmp_by_name(&entry2, true), Ordering::Less);
        assert_eq!(entry2.cmp_by_name(&entry1, true), Ordering::Greater);
        assert_eq!(entry1.cmp_by_name(&entry2, false), Ordering::Greater);
        assert_eq!(entry2.cmp_by_name(&entry1, false), Ordering::Less);
    }

    #[test]
    fn test_cmp_by_date() {
        let entry1 = Entry::new("name", "link", "2025-05-21 03:45", "112");
        let entry2 = Entry::new("othername", "link", "2025-05-20 20:19", "112");

        assert_eq!(entry1.cmp_by_date(&entry2, true), Ordering::Greater);
        assert_eq!(entry2.cmp_by_date(&entry1, true), Ordering::Less);
        assert_eq!(entry1.cmp_by_date(&entry2, false), Ordering::Less);
        assert_eq!(entry2.cmp_by_date(&entry1, false), Ordering::Greater);
    }

    #[test]
    fn test_cmp_by_size() {
        let entry1 = Entry::new("name", "link", "2025-05-21 03:45", "4.0k");
        let entry2 = Entry::new("othername", "link", "2025-05-20 20:19", "112");

        assert_eq!(entry1.cmp_by_size(&entry2, true), Ordering::Greater);
        assert_eq!(entry2.cmp_by_size(&entry1, true), Ordering::Less);
        assert_eq!(entry1.cmp_by_size(&entry2, false), Ordering::Less);
        assert_eq!(entry2.cmp_by_size(&entry1, false), Ordering::Greater);
    }

    #[test]
    fn test_date_format_1() {
        let entry = Entry::new("name", "link", "2023-12-03 17:33", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2023-12-03 17:33");
    }

    #[test]
    fn test_date_format_2() {
        let entry = Entry::new("name", "link", "05-Apr-2024 11:59", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2024-04-05 11:59");
    }

    #[test]
    fn test_date_format_3() {
        let entry = Entry::new("name", "link", "2021-May-25 20:15", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2021-05-25 20:15");
    }

    #[test]
    fn test_date_format_4() {
        let entry = Entry::new("name", "link", "2023-12-03 17:33:19", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2023-12-03 17:33");
    }

    #[test]
    fn test_date_format_5() {
        let entry = Entry::new("name", "link", "05-Apr-2024 11:59:30", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2024-04-05 11:59");
    }

    #[test]
    fn test_date_format_6() {
        let entry = Entry::new("name", "link", "2021-May-25 20:15:46", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2021-05-25 20:15");
    }

    #[test]
    fn test_date_format_7() {
        let entry = Entry::new("name", "link", "2025/10/21 21:53:58", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-10-21 21:53");
    }

    #[test]
    fn test_date_format_8() {
        let entry = Entry::new("name", "link", "05/31/2025 01:54:45 PM +00:00", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-05-31 13:54");
    }

    #[test]
    fn test_date_format_9() {
        let entry = Entry::new("name", "link", "2025-10-20T14:17Z", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-10-20 14:17");
    }

    #[test]
    fn test_date_format_10() {
        let entry = Entry::new("name", "link", "20-10-2025 | 13:52", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-10-20 13:52");
    }

    #[test]
    fn test_date_format_11() {
        let entry = Entry::new("name", "link", "2025-10-20 16:17 CEST", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-10-20 16:17");
    }

    #[test]
    fn test_date_format_12() {
        let entry = Entry::new("name", "link", "2025-09-06 18:15:23 CST", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-09-06 18:15");
    }

    #[test]
    fn test_date_format_13() {
        let entry = Entry::new("name", "link", "October 21, 2025 20:53", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-10-21 20:53");
    }

    #[test]
    fn test_date_format_14() {
        let entry = Entry::new("name", "link", "06 Sep 2025 10:15:23 +0000", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-09-06 10:15");
    }

    #[test]
    fn test_date_format_15() {
        let entry = Entry::new("name", "link", "21-10-2025 14:19", "4.0 kib");
        let date_str = entry.date.unreachable().format("%Y-%m-%d %H:%M").to_string();
        assert_eq!(date_str, "2025-10-21 14:19");
    }

    #[test]
    fn test_date_not_a_format() {
        let entry = Entry::new("name", "link", "21-2025-10, 14:19", "4.0 kib");
        assert!(entry.date.is_none());
    }
}
