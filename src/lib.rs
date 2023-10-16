#![warn(clippy::all, clippy::pedantic, clippy::unwrap_used)]
use chrono::{Days, NaiveDate};
use std::ops::Range;

pub struct ParseResult<T> {
    pub data: T,
    pub range: Range<usize>,
}

// needs a name that doesn't include "Result"
impl<T> ParseResult<T> {
    pub fn map<U, F>(self, f: F) -> ParseResult<U>
    where
        F: FnOnce(T) -> U,
    {
        ParseResult {
            data: f(self.data),
            range: self.range,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FlexibleDate {
    Today,
    Tomorrow,
}

impl FlexibleDate {
    #[must_use]
    pub fn parse_from_str(text: &str) -> Option<ParseResult<FlexibleDate>> {
        if text == "today" || text == "tod" {
            Some(ParseResult {
                data: FlexibleDate::Today,
                range: (0..text.len()),
            })
        } else if text == "tomorrow" || text == "tom" {
            Some(ParseResult {
                data: FlexibleDate::Tomorrow,
                range: (0..text.len()),
            })
        } else {
            None
        }
    }

    /// # Panics
    /// If something rare goes wrong while incrementing the date
    #[must_use]
    pub fn into_naive_date(self, today: NaiveDate) -> NaiveDate {
        match self {
            FlexibleDate::Today => today,
            FlexibleDate::Tomorrow => today
                .checked_add_days(Days::new(1))
                .expect("error while adding days to date"),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use chrono::{Datelike, NaiveDate};

    fn parse_date(string: &str) -> NaiveDate {
        NaiveDate::parse_from_str(string, "%Y-%m-%d").expect("parsing date in test")
    }

    #[test]
    fn parse_today() {
        let result = FlexibleDate::parse_from_str("today").unwrap();
        assert_eq!(result.data, FlexibleDate::Today);
        assert_eq!(result.range, (0..5));

        let result = FlexibleDate::parse_from_str("tod").unwrap();
        assert_eq!(result.data, FlexibleDate::Today);
        assert_eq!(result.range, (0..3));
    }

    #[test]
    fn parse_tomorrow() {
        let result = FlexibleDate::parse_from_str("tomorrow").unwrap();
        assert_eq!(result.data, FlexibleDate::Tomorrow);
        assert_eq!(result.range, (0..8));

        let result = FlexibleDate::parse_from_str("tom").unwrap();
        assert_eq!(result.data, FlexibleDate::Tomorrow);
        assert_eq!(result.range, (0..3));
    }

    #[test]
    fn parse_junk() {
        let result = FlexibleDate::parse_from_str("I'm a little teapot");
        assert!(result.is_none());

        let result = FlexibleDate::parse_from_str("todd tomm tday tomrow todayyy");
        assert!(result.is_none());
    }

    #[test]
    fn today_into_date() {
        let today = parse_date("2023-10-08");
        let date = FlexibleDate::Today.into_naive_date(today);
        assert_eq!(date.month(), 10);
        assert_eq!(date.day(), 8);
        assert_eq!(date.year(), 2023);
    }

    #[test]
    fn tomorrow_into_date() {
        let today = parse_date("2023-10-08");
        let date = FlexibleDate::Tomorrow.into_naive_date(today);
        assert_eq!(date.month(), 10);
        assert_eq!(date.day(), 9);
        assert_eq!(date.year(), 2023);
    }
}
