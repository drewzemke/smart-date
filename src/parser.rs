use crate::{FlexibleDate, Parsed};
use nom::{
    branch,
    bytes::complete::{is_not, tag},
    character::complete::space1,
    combinator::value,
    error::{Error, ErrorKind},
    sequence::tuple,
    Err, IResult,
};

fn not_whitespace(input: &str) -> IResult<&str, &str> {
    is_not(" \t")(input)
}

fn parse_today(input: &str) -> IResult<&str, FlexibleDate> {
    value(FlexibleDate::Today, branch::alt((tag("today"), tag("tod"))))(input)
}

fn parse_tomorrow(input: &str) -> IResult<&str, FlexibleDate> {
    value(
        FlexibleDate::Tomorrow,
        branch::alt((tag("tomorrow"), tag("tom"))),
    )(input)
}

/// Try to parse a string into a `FlexibleDate` starting at the beginning of the string
fn parse_flex_date_exact(input: &str) -> IResult<&str, FlexibleDate> {
    branch::alt((parse_today, parse_tomorrow))(input)
}

/// Try to parse a string into a `FlexibleDate` starting at the beginning of the string.
/// Only succeeds if it can parse the date as a complete collection of tokens.
fn parse_flex_date_with_suffix(input: &str) -> IResult<&str, FlexibleDate> {
    let (remainder, date) = parse_flex_date_exact(input)?;

    // make sure that the next character in the output (if there is one) is a space
    if remainder.is_empty() || remainder.chars().next().is_some_and(char::is_whitespace) {
        Ok((remainder, date))
    } else {
        // gross
        Err(Err::Error(Error {
            input,
            code: ErrorKind::Char,
        }))
    }
}

// TODO: docs
pub(crate) fn parse_flex_date(input: &str) -> Option<Parsed<FlexibleDate>> {
    let mut input = input;
    let mut offset = 0;
    while parse_flex_date_with_suffix(input).is_err() && !input.is_empty() {
        // eat a token
        let (remainder, (token, space)) = tuple((not_whitespace, space1))(input).ok()?;
        input = remainder;
        offset += token.len() + space.len();
    }
    parse_flex_date_exact(input)
        .ok()
        .map(|(remainder, date)| Parsed {
            data: date,
            range: offset..(offset + input.len() - remainder.len()),
        })
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
    fn test_parse_flex_date_exact() {
        let (_, result) = parse_flex_date_exact("tomorrow").unwrap();
        assert_eq!(result, FlexibleDate::Tomorrow);

        let (_, result) = parse_flex_date_exact("tod").unwrap();
        assert_eq!(result, FlexibleDate::Today);
    }

    #[test]
    fn test_parse_flex_date_substring() {
        let Parsed { data, range } = parse_flex_date("tomorrow after").unwrap();
        assert_eq!(data, FlexibleDate::Tomorrow);
        assert_eq!(range, (0..8));

        let Parsed { data, range } = parse_flex_date("before tomorrow").unwrap();
        assert_eq!(data, FlexibleDate::Tomorrow);
        assert_eq!(range, (7..15));

        let input = "before tomorrow after";
        let Parsed { data, range } = parse_flex_date(input).unwrap();
        assert_eq!(data, FlexibleDate::Tomorrow);
        assert_eq!(range, (7..15));
        assert_eq!(&input[range], "tomorrow");

        let Parsed { data, range } = parse_flex_date("do a barrel roll tod").unwrap();
        assert_eq!(data, FlexibleDate::Today);
        assert_eq!(range, (17..20));
    }

    #[test]
    fn test_parse_junk() {
        let result = parse_flex_date("I'm a little teapot");
        assert!(result.is_none());

        // Make sure we only recognize dates that appear as full tokens
        let result = parse_flex_date("todd tomm ttoday dtomorrow todayyy");
        assert!(result.is_none());
    }
}
