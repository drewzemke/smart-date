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

#[must_use]
pub fn parse(text: &str, today: NaiveDate) -> Option<ParseResult<NaiveDate>> {
    parse_date(text).map(|result| result.map(|data| convert_date(&data, today)))
}

enum FlexibleDate {
    Today,
    Tomorrow,
}

fn parse_date(text: &str) -> Option<ParseResult<FlexibleDate>> {
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

fn convert_date(date: &FlexibleDate, today: NaiveDate) -> NaiveDate {
    match date {
        FlexibleDate::Today => today,
        FlexibleDate::Tomorrow => today
            .checked_add_days(Days::new(1))
            .expect("error while adding days to date"),
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
        let now = parse_date("2023-10-08");
        let result = parse("today", now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 8);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..5));

        let result = parse("tod", now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 8);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..3));
    }

    #[test]
    fn parse_tomorrow() {
        let now = parse_date("2023-10-08");
        let result = parse("tomorrow", now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 9);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..8));

        let now = parse_date("2023-09-30");
        let result = parse("tom", now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 1);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..3));
    }

    #[test]
    fn parse_junk() {
        let now = parse_date("2023-10-08");
        let result = parse("I'm a little teapot", now);
        assert!(result.is_none());

        let result = parse("todd tomm tday tomrow todayyy", now);
        assert!(result.is_none());
    }
}
