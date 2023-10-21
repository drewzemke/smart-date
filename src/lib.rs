#![warn(clippy::all, clippy::pedantic, clippy::unwrap_used)]
use chrono::{Days, NaiveDate};
use parser::{parse_flex_date, parse_flex_date_exact};
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

/// Represents a relative (or, eventually, absolute) date.
///
/// # Examples
/// Here are of input strings that will eventually be supported.
/// See [the Todoist docs](https://todoist.com/help/articles/introduction-to-due-dates-and-due-times-q7VobO).
/// - [x] "today", "tod"
/// - [x] "tomorrow", "tom", "tmrw"
/// - [ ] "wednesday", "wed" (any weekday)
/// - [ ] "next week"
/// - [ ] "this weekend"
/// - [ ] "next weekend"
/// - [ ] "in 3 days", "in three days"
/// - [ ] "in 2 weeks", "in two weeks"
/// - [ ] "2 weeks from now"
/// - [ ] "in four months"
/// - [ ] "in one year"
/// - [ ] "next month"
/// - [ ] "january 27", "jan 27", "01/27"
/// - [ ] "jan 27 2024", "01/27/2024"
/// - [ ] "27th"
/// - [ ] "mid january"
/// - [ ] "mid jan"
/// - [ ] "later this week"
/// - [ ] "two weeks from tomorrow"
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlexibleDate {
    Today,
    Tomorrow,
}

impl FlexibleDate {
    /// Parses a `FlexibleDate` from within a string. Fails (returns `None`) if the full string does
    /// not match a date.
    ///
    ///
    /// ```rust
    /// # use smart_date::FlexibleDate;
    /// # fn main() {
    /// let result1 = FlexibleDate::parse_from_str("today").unwrap();
    /// assert_eq!(result1, FlexibleDate::Today);
    ///
    /// let result2 = FlexibleDate::parse_from_str("tom").unwrap();
    /// assert_eq!(result2, FlexibleDate::Tomorrow);
    ///
    /// let result3 = FlexibleDate::parse_from_str("go to the store today");
    /// assert_eq!(result3, None);
    ///  # }
    /// ```
    #[must_use]
    pub fn parse_from_str(text: &str) -> Option<FlexibleDate> {
        parse_flex_date_exact(text).ok().map(|(_, date)| date)
    }

    /// Finds and parses a `FlexibleDate` from within a string. The returned `Parsed<>` type contains
    /// the date that was parsed as well as the location of the matching substring in the input.
    ///
    ///
    /// ```rust
    /// # use smart_date::FlexibleDate;
    /// # fn main() {
    /// let result1 = FlexibleDate::find_and_parse_in_str("go to the store today").unwrap();
    /// assert_eq!(result1.data, FlexibleDate::Today);
    /// assert_eq!(result1.range, (16..21));
    ///
    /// let result2 = FlexibleDate::find_and_parse_in_str("do a barrel tom okay?").unwrap();
    /// assert_eq!(result2.data, FlexibleDate::Tomorrow);
    /// assert_eq!(result2.range, (12..15));
    ///  # }
    /// ```
    #[must_use]
    pub fn find_and_parse_in_str(text: &str) -> Option<Parsed<FlexibleDate>> {
        parse_flex_date(text)
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
