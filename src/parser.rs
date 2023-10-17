use crate::FlexibleDate;
use nom::{
    branch,
    bytes::complete::tag,
    combinator::{self, all_consuming},
    IResult,
};

fn parse_today(input: &str) -> IResult<&str, FlexibleDate> {
    combinator::value(FlexibleDate::Today, branch::alt((tag("today"), tag("tod"))))(input)
}

fn parse_tomorrow(input: &str) -> IResult<&str, FlexibleDate> {
    combinator::value(
        FlexibleDate::Tomorrow,
        branch::alt((tag("tomorrow"), tag("tom"))),
    )(input)
}

/// Parses a string into a `FlexibleDate`.
///
/// NOTE: for now ,this fails if there's any text following the matched substring.
pub fn parse_flex_date(input: &str) -> IResult<&str, FlexibleDate> {
    all_consuming(branch::alt((parse_today, parse_tomorrow)))(input)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn test_parse_today() {
        let (_, result) = parse_today("today").unwrap();
        assert_eq!(result, FlexibleDate::Today);

        let (_, result) = parse_today("tod").unwrap();
        assert_eq!(result, FlexibleDate::Today);
    }

    #[test]
    fn test_parse_tomorrow() {
        let (_, result) = parse_tomorrow("tomorrow").unwrap();
        assert_eq!(result, FlexibleDate::Tomorrow);

        let (_, result) = parse_tomorrow("tom").unwrap();
        assert_eq!(result, FlexibleDate::Tomorrow);
    }

    #[test]
    fn test_parse_flex_date() {
        let (_, result) = parse_flex_date("tomorrow").unwrap();
        assert_eq!(result, FlexibleDate::Tomorrow);

        let (_, result) = parse_flex_date("tod").unwrap();
        assert_eq!(result, FlexibleDate::Today);
    }
}
