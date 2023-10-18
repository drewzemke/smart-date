#![warn(clippy::all, clippy::pedantic, clippy::unwrap_used)]
use chrono::{Days, NaiveDate};
use parser::parse_flex_date;
use std::ops::Range;

mod parser;

/// Represents some data that has been parsed out of a string.
/// Contains the data that was extracted as well as the location in
/// the input string of the substring that was related to the data.
pub struct Parsed<T> {
    pub data: T,

    // TODO: consider storing a substring instead, then provide a method to
    // compute the offset.
    // see https://stackoverflow.com/questions/67148359/check-if-a-str-is-a-sub-slice-of-another-str
    pub range: Range<usize>,
}

impl<T> Parsed<T> {
    pub fn map<U, F>(self, f: F) -> Parsed<U>
    where
        F: FnOnce(T) -> U,
    {
        Parsed {
            data: f(self.data),
            range: self.range,
        }
    }
}

/// Represents a relative (or, eventually, absolute) date.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlexibleDate {
    Today,
    Tomorrow,
}

impl FlexibleDate {
    /// Parses a `FlexibleDate` from a string. The returned `Parsed<>` type contains
    /// the date that was parsed as well as the location of the matching substring in the input.
    ///
    ///
    /// ```rust
    /// # use smart_date::FlexibleDate;
    /// # fn main() {
    /// let result1 = FlexibleDate::parse_from_str("today").unwrap();
    /// assert_eq!(result1.data, FlexibleDate::Today);
    /// assert_eq!(result1.range, (0..5));
    ///
    /// let result2 = FlexibleDate::parse_from_str("tom").unwrap();
    /// assert_eq!(result2.data, FlexibleDate::Tomorrow);
    /// assert_eq!(result2.range, (0..3));
    ///  # }
    /// ```
    #[must_use]
    pub fn parse_from_str(text: &str) -> Option<Parsed<FlexibleDate>> {
        match parse_flex_date(text) {
            Ok((_, date)) => Some(Parsed {
                data: date,
                range: (0..text.len()),
            }),
            Err(_) => None,
        }
    }

    /// Converts the `FlexibleDate` into a [`NaiveDate`].
    ///
    /// ```rust
    /// # use smart_date::FlexibleDate;
    /// # use chrono::Datelike;
    /// # fn main() {
    /// let today = chrono::NaiveDate::parse_from_str("2023-10-08", "%Y-%m-%d").unwrap();
    ///
    /// let date = FlexibleDate::Today.into_naive_date(today);
    /// assert_eq!(date.month(), 10);
    /// assert_eq!(date.day(), 8);
    /// assert_eq!(date.year(), 2023);
    ///
    /// let date = FlexibleDate::Tomorrow.into_naive_date(today);
    /// assert_eq!(date.month(), 10);
    /// assert_eq!(date.day(), 9);
    /// assert_eq!(date.year(), 2023);
    /// # }
    /// ```
    #[must_use]
    pub fn into_naive_date(self, today: NaiveDate) -> NaiveDate {
        match self {
            FlexibleDate::Today => today,
            FlexibleDate::Tomorrow => today + Days::new(1),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn parse_junk() {
        let result = FlexibleDate::parse_from_str("I'm a little teapot");
        assert!(result.is_none());

        let result = FlexibleDate::parse_from_str("todd tomm tday tomrow todayyy");
        assert!(result.is_none());
    }
}
