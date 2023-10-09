use std::ops::Range;

use chrono::{DateTime, Days, Local, NaiveTime};

pub struct ParseResult<T> {
    pub data: T,
    pub range: Range<usize>,
}

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

pub fn parse(text: &str, now: &DateTime<Local>) -> Option<ParseResult<DateTime<Local>>> {
    parse_date(text).map(|result| result.map(|data| convert_date(data, now)))
}

// definitely needs a better name
enum Date {
    Today,
    Tomorrow,
}

fn parse_date(text: &str) -> Option<ParseResult<Date>> {
    if text == "today" || text == "tod" {
        Some(ParseResult {
            data: Date::Today,
            range: (0..text.len()),
        })
    } else if text == "tomorrow" || text == "tom" {
        Some(ParseResult {
            data: Date::Tomorrow,
            range: (0..text.len()),
        })
    } else {
        None
    }
}

fn convert_date(date: Date, now: &DateTime<Local>) -> DateTime<Local> {
    match date {
        Date::Today => now
            .date_naive()
            .and_time(NaiveTime::default())
            .and_local_timezone(Local)
            .unwrap(),
        Date::Tomorrow => now
            .date_naive()
            .checked_add_days(Days::new(1))
            .unwrap()
            .and_time(NaiveTime::default())
            .and_local_timezone(Local)
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Local, NaiveDateTime};

    fn parse_date_time(string: &str) -> DateTime<Local> {
        NaiveDateTime::parse_from_str(string, "%Y-%m-%d %H:%M")
            .unwrap()
            .and_local_timezone(Local)
            .unwrap()
    }

    #[test]
    fn parse_today() {
        let now = parse_date_time("2023-10-08 20:29");
        let result = parse("today", &now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 8);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..5));

        let result = parse("tod", &now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 8);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..3));
    }

    #[test]
    fn parse_tomorrow() {
        let now = parse_date_time("2023-10-08 20:29");
        let result = parse("tomorrow", &now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 9);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..8));

        let now = parse_date_time("2023-09-30 20:29");
        let result = parse("tom", &now).unwrap();
        assert_eq!(result.data.month(), 10);
        assert_eq!(result.data.day(), 1);
        assert_eq!(result.data.year(), 2023);
        assert_eq!(result.range, (0..3));
    }

    #[test]
    fn parse_junk() {
        let now = parse_date_time("2023-10-08 20:29");
        let result = parse("I'm a little teapot", &now);
        assert!(result.is_none());

        let result = parse("todd tomm tday tomrow todayyy", &now);
        assert!(result.is_none());
    }
}
