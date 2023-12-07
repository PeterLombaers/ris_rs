use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{line_ending, one_of};
use nom::combinator::{map, recognize, value};
use nom::multi::{many_till, separated_list1};
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

fn parse_to_next_reference(input: &str) -> IResult<&str, &str> {
    take_until("TY  - ")(input)
}

fn parse_ris(input: &str) -> IResult<&str, Vec<Reference>> {
    preceded(
        parse_to_next_reference,
        separated_list1(parse_to_next_reference, parse_reference),
    )(input)
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
    fn test_reference() {
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

    #[test]
    fn test_multiple_references() {
        let ref_string = "1.
TY  - JOUR
ID  - 12345
T1  - Title of reference
A1  - Marx, Karl
A1  - Lindgren, Astrid
A2  - Glattauer, Daniel
Y1  - 2014//
N2  - BACKGROUND: Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.  RESULTS: Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. CONCLUSIONS: Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium.
KW  - Pippi
KW  - Nordwind
KW  - Piraten
JF  - Lorem
JA  - lorem
VL  - 9
IS  - 3
SP  - e0815
CY  - United States
PB  - Fun Factory
SN  - 1932-6208
M1  - 1008150341
L2  - http://example.com
UR  - http://example_url.com
ER  - 

2.
TY  - JOUR
ID  - 12345
T1  - The title of the reference
A1  - Marxus, Karlus
A1  - Lindgren, Astrid
A2  - Glattauer, Daniel
Y1  - 2006//
N2  - BACKGROUND: Lorem dammed ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.  RESULTS: Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. CONCLUSIONS: Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium.
KW  - Pippi Langstrumpf
KW  - Nordwind
KW  - Piraten
JF  - Lorem
JA  - lorem
VL  - 6
IS  - 3
SP  - e0815341
CY  - Germany
PB  - Dark Factory
SN  - 1732-4208
M1  - 1228150341
L2  - http://example2.com
UR  - http://example_url.com
ER  - ";
        let (_, references) = parse_ris(ref_string).unwrap();
        assert_eq!(references.len(), 2);
        assert_eq!(references[0].ref_type, "JOUR".to_string());
        assert!(references[0].fields.contains(&Field {
            tag: "ID".to_string(),
            content: Content::String("12345".to_string())
        }));
        assert!(references[0].fields.contains(&Field {
            tag: "CY".to_string(),
            content: Content::String("United States".to_string())
        }));
        assert!(references[0].fields.contains(&Field {
            tag: "Y1".to_string(),
            content: Content::String("2014//".to_string())
        }));

        assert_eq!(references[1].ref_type, "JOUR".to_string());
        assert!(references[1].fields.contains(&Field {
            tag: "T1".to_string(),
            content: Content::String("The title of the reference".to_string())
        }));
        assert!(references[1].fields.contains(&Field {
            tag: "SN".to_string(),
            content: Content::String("1732-4208".to_string())
        }));
        assert!(references[1].fields.contains(&Field {
            tag: "UR".to_string(),
            content: Content::String("http://example_url.com".to_string())
        }));
    }
}
