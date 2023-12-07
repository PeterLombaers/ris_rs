use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{map, recognize, value};
use nom::multi::many_till;
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Content {
    String(String),
    List(Vec<String>),
}

#[derive(Debug, PartialEq)]
pub struct Field {
    tag: String,
    content: Content,
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    key: String,
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct Reference {
    ref_type: String,
    fields: Vec<Field>,
}

fn parse_reference(input: &str) -> IResult<&str, Reference> {
    let (remainder, (ref_type, (fields, _))) = pair(
        parse_reference_type,
        many_till(parse_string_field, parse_end_of_reference),
    )(input)?;
    Ok((
        remainder,
        Reference {
            ref_type: ref_type.to_string(),
            fields,
        },
    ))
}

fn parse_uppercase_char(input: &str) -> IResult<&str, char> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

fn parse_single_digit(input: &str) -> IResult<&str, char> {
    one_of("0123456789")(input)
}

fn parse_tag_key(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        parse_uppercase_char,
        alt((parse_uppercase_char, parse_single_digit)),
    ))(input)
}

fn parse_tag(input: &str) -> IResult<&str, &str> {
    terminated(parse_tag_key, tag("  - "))(input)
}

fn parse_rest_of_line(input: &str) -> IResult<&str, &str> {
    terminated(is_not("\r\n"), line_ending)(input)
}

fn parse_string_field(input: &str) -> IResult<&str, Field> {
    map(pair(parse_tag, parse_rest_of_line), |(t, c)| Field {
        tag: t.to_string(),
        content: Content::String(c.to_string()),
    })(input)
}

fn parse_reference_type(input: &str) -> IResult<&str, &str> {
    preceded(tag("TY  - "), parse_rest_of_line)(input)
}

fn parse_end_of_reference(input: &str) -> IResult<&str, ()> {
    value((), tag("ER  - "))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tag() {
        assert_eq!(parse_tag("AB  - Good"), Ok(("Good", "AB")));
        assert_eq!(parse_tag("M9  - Good"), Ok(("Good", "M9")));
        assert!(parse_tag("9M  - Bad").is_err());
        assert!(parse_tag("m9  - Bad").is_err());
    }

    #[test]
    fn test_parse_reference() {
        let ref_string = "TY  - JOUR
AU  - Shannon,Claude E.
PY  - 1948/07//
TI  - A Mathematical Theory of Communication
JF  - Bell System Technical Journal
SP  - 379
EP  - 423
VL  - 27
ER  - ";
        let (remainder, reference) = parse_reference(ref_string).unwrap();
        assert_eq!(remainder, "");
        assert_eq!(
            reference,
            Reference {
                ref_type: "JOUR".to_string(),
                fields: vec![
                    Field {
                        tag: "AU".to_string(),
                        content: Content::String("Shannon,Claude E.".to_string())
                    },
                    Field {
                        tag: "PY".to_string(),
                        content: Content::String("1948/07//".to_string())
                    },
                    Field {
                        tag: "TI".to_string(),
                        content: Content::String(
                            "A Mathematical Theory of Communication".to_string()
                        )
                    },
                    Field {
                        tag: "JF".to_string(),
                        content: Content::String("Bell System Technical Journal".to_string())
                    },
                    Field {
                        tag: "SP".to_string(),
                        content: Content::String("379".to_string())
                    },
                    Field {
                        tag: "EP".to_string(),
                        content: Content::String("423".to_string())
                    },
                    Field {
                        tag: "VL".to_string(),
                        content: Content::String("27".to_string())
                    },
                ]
            }
        )
    }
}
