use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{anychar, line_ending, one_of};
use nom::combinator::{map, peek, recognize, value};
use nom::multi::{many_till, separated_list1};
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    tag: &'a str,
    content: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Reference<'a> {
    ref_type: &'a str,
    fields: Vec<Field<'a>>,
}

fn parse_reference(input: &str) -> IResult<&str, Reference> {
    let (remainder, (ref_type, (fields, _))) = pair(
        parse_reference_type,
        many_till(parse_field, parse_end_of_reference),
    )(input)?;
    Ok((
        remainder,
        Reference {
            ref_type,
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

fn parse_to_next_tag(input: &str) -> IResult<&str, &str> {
    terminated(
        recognize(many_till(anychar, peek(pair(line_ending, parse_tag)))),
        line_ending,
    )(input)
}

fn parse_rest_of_line(input: &str) -> IResult<&str, &str> {
    terminated(is_not("\r\n"), line_ending)(input)
}

fn parse_field(input: &str) -> IResult<&str, Field> {
    map(pair(parse_tag, parse_to_next_tag), |(tag, content)| Field {
        tag,
        content,
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

pub fn parse_ris(input: &str) -> IResult<&str, Vec<Reference>> {
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
                ref_type: "JOUR",
                fields: vec![
                    Field {
                        tag: "AU",
                        content: "Shannon,Claude E."
                    },
                    Field {
                        tag: "PY",
                        content: "1948/07//"
                    },
                    Field {
                        tag: "TI",
                        content: "A Mathematical Theory of Communication"
                    },
                    Field {
                        tag: "JF",
                        content: "Bell System Technical Journal"
                    },
                    Field {
                        tag: "SP",
                        content: "379"
                    },
                    Field {
                        tag: "EP",
                        content: "423"
                    },
                    Field {
                        tag: "VL",
                        content: "27"
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
            tag: "ID",
            content: "12345"
        }));
        assert!(references[0].fields.contains(&Field {
            tag: "CY",
            content: "United States"
        }));
        assert!(references[0].fields.contains(&Field {
            tag: "Y1",
            content: "2014//"
        }));

        assert_eq!(references[1].ref_type, "JOUR".to_string());
        assert!(references[1].fields.contains(&Field {
            tag: "T1",
            content: "The title of the reference"
        }));
        assert!(references[1].fields.contains(&Field {
            tag: "SN",
            content: "1732-4208"
        }));
        assert!(references[1].fields.contains(&Field {
            tag: "UR",
            content: "http://example_url.com"
        }));
    }

    #[test]
    fn test_multiline_field() {
        let ref_string = "TY  - JOUR
AU  - Shannon,Claude E.
PY  - 1948/07//
TI  - A Mathematical Theory of Communication
JF  - Bell System Technical Journal
SP  - 379
EP  - 423
N2  - first line,  
        then second line and at the end 
        the last line
N1  - first line
        * second line
        * last line
VL  - 27
ER  - ";
        let (_, reference) = parse_reference(ref_string).unwrap();
        assert_eq!(reference.fields.len(), 9);
        assert!(reference.fields.contains(&Field{tag: "N2", content: "first line,  
        then second line and at the end 
        the last line"}));
        assert!(reference.fields.contains(&Field{tag: "N1", content: "first line
        * second line
        * last line"}));
    }
}
