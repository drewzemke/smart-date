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
        let result1 = parse("today", &now).unwrap();
        assert_eq!(result1.month(), 10);
        assert_eq!(result1.day(), 8);
        assert_eq!(result1.year(), 2023);

        let result2 = parse("tod", &now).unwrap();
        assert_eq!(result2.month(), 10);
        assert_eq!(result2.day(), 8);
        assert_eq!(result2.year(), 2023);
    }
}
