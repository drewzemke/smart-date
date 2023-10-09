use chrono::{DateTime, Local, NaiveTime};

pub fn parse(text: &str, now: &DateTime<Local>) -> Option<DateTime<Local>> {
    println!("{}", now);
    if text == "today" || text == "tod" {
        Some(
            now.date_naive()
                .and_time(NaiveTime::default())
                .and_local_timezone(Local)
                .unwrap(),
        )
    } else if text == "tomorrow" || text == "tom" {
        Some(
            now.date_naive()
                .checked_add_days(Days::new(1))
                .unwrap()
                .and_time(NaiveTime::default())
                .and_local_timezone(Local)
                .unwrap(),
        )
    } else {
        None
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
        assert_eq!(result.month(), 10);
        assert_eq!(result.day(), 8);
        assert_eq!(result.year(), 2023);

        let result = parse("tod", &now).unwrap();
        assert_eq!(result.month(), 10);
        assert_eq!(result.day(), 8);
        assert_eq!(result.year(), 2023);
    }

    #[test]
    fn parse_tomorrow() {
        let now = parse_date_time("2023-10-08 20:29");
        let result = parse("tomorrow", &now).unwrap();
        assert_eq!(result.month(), 10);
        assert_eq!(result.day(), 9);
        assert_eq!(result.year(), 2023);

        let now = parse_date_time("2023-09-30 20:29");
        let result = parse("tom", &now).unwrap();
        assert_eq!(result.month(), 10);
        assert_eq!(result.day(), 1);
        assert_eq!(result.year(), 2023);
    }
}
