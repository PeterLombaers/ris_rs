use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{anychar, line_ending};
use nom::combinator::{map, peek, recognize, value};
use nom::multi::{many_till, separated_list1};
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

use crate::Field;
use crate::Reference;
use crate::Tag;

fn parse_reference(input: &str) -> IResult<&str, Reference> {
    let (remainder, (ref_type, (fields, _))) = pair(
        parse_reference_type,
        many_till(parse_field, parse_end_of_reference),
    )(input)?;
    Ok((remainder, Reference::new(ref_type, fields)))
}

fn parse_tag_key(input: &str) -> IResult<&str, Tag> {
    alt((
        value(Tag::TY, tag("TY")),
        value(Tag::A1, tag("A1")),
        value(Tag::A2, tag("A2")),
        value(Tag::A3, tag("A3")),
        value(Tag::A4, tag("A4")),
        value(Tag::AB, tag("AB")),
        value(Tag::AD, tag("AD")),
        value(Tag::AN, tag("AN")),
        value(Tag::AU, tag("AU")),
        value(Tag::C1, tag("C1")),
        value(Tag::C2, tag("C2")),
        value(Tag::C3, tag("C3")),
        value(Tag::C4, tag("C4")),
        value(Tag::C5, tag("C5")),
        value(Tag::C6, tag("C6")),
        value(Tag::C7, tag("C7")),
        value(Tag::C8, tag("C8")),
        value(Tag::CA, tag("CA")),
        value(Tag::CN, tag("CN")),
        value(Tag::CY, tag("CY")),
        alt((
            value(Tag::DA, tag("DA")),
            value(Tag::DB, tag("DB")),
            value(Tag::DO, tag("DO")),
            value(Tag::DP, tag("DP")),
            value(Tag::ET, tag("ET")),
            value(Tag::EP, tag("EP")),
            value(Tag::ID, tag("ID")),
            value(Tag::IS, tag("IS")),
            value(Tag::J2, tag("J2")),
            value(Tag::JA, tag("JA")),
            value(Tag::JF, tag("JF")),
            value(Tag::JO, tag("JO")),
            value(Tag::KW, tag("KW")),
            value(Tag::L1, tag("L1")),
            value(Tag::L2, tag("L2")),
            value(Tag::L4, tag("L4")),
            value(Tag::LA, tag("LA")),
            value(Tag::LB, tag("LB")),
            value(Tag::M1, tag("M1")),
            value(Tag::M3, tag("M3")),
            alt((
                value(Tag::N1, tag("N1")),
                value(Tag::N2, tag("N2")),
                value(Tag::NV, tag("NV")),
                value(Tag::OP, tag("OP")),
                value(Tag::PB, tag("PB")),
                value(Tag::PY, tag("PY")),
                value(Tag::RI, tag("RI")),
                value(Tag::RN, tag("RN")),
                value(Tag::RP, tag("RP")),
                value(Tag::SE, tag("SE")),
                value(Tag::SN, tag("SN")),
                value(Tag::SP, tag("SP")),
                value(Tag::ST, tag("ST")),
                value(Tag::T1, tag("T1")),
                value(Tag::T2, tag("T2")),
                value(Tag::T3, tag("T3")),
                value(Tag::TA, tag("TA")),
                value(Tag::TI, tag("TI")),
                value(Tag::TT, tag("TT")),
                alt((
                    value(Tag::UR, tag("UR")),
                    value(Tag::VL, tag("VL")),
                    value(Tag::Y1, tag("Y1")),
                    value(Tag::Y2, tag("Y2")),
                    value(Tag::ER, tag("ER")),
                    value(Tag::UK, tag("UK")),
                )),
            )),
        )),
    ))(input)
}

fn parse_tag(input: &str) -> IResult<&str, Tag> {
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
    map(pair(parse_tag, parse_to_next_tag), |(tag, content)| {
        Field::new(tag, content)
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
        assert_eq!(parse_tag("A1  - Good"), Ok(("Good", Tag::A1)));
        assert_eq!(parse_tag("DA  - Also Good"), Ok(("Also Good", Tag::DA)));
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
            Reference::new(
                "JOUR",
                vec![
                    Field::new(Tag::AU, "Shannon,Claude E."),
                    Field::new(Tag::PY, "1948/07//"),
                    Field::new(Tag::TI, "A Mathematical Theory of Communication"),
                    Field::new(Tag::JF, "Bell System Technical Journal"),
                    Field::new(Tag::SP, "379"),
                    Field::new(Tag::EP, "423"),
                    Field::new(Tag::VL, "27"),
                ]
            )
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
        assert_eq!(references[0].ref_type(), "JOUR".to_string());
        assert!(references[0]
            .fields()
            .contains(&Field::new(Tag::ID, "12345")));
        assert!(references[0]
            .fields()
            .contains(&Field::new(Tag::CY, "United States")));
        assert!(references[0]
            .fields()
            .contains(&Field::new(Tag::Y1, "2014//")));

        assert_eq!(references[1].ref_type(), "JOUR".to_string());
        assert!(references[1]
            .fields()
            .contains(&Field::new(Tag::T1, "The title of the reference")));
        assert!(references[1]
            .fields()
            .contains(&Field::new(Tag::SN, "1732-4208")));
        assert!(references[1]
            .fields()
            .contains(&Field::new(Tag::UR, "http://example_url.com")));
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
        assert_eq!(reference.fields().len(), 9);
        assert!(reference.fields().contains(&Field::new(
            Tag::N2,
            "first line,  
        then second line and at the end 
        the last line"
        )));
        assert!(reference.fields().contains(&Field::new(
            Tag::N1,
            "first line
        * second line
        * last line"
        )));
    }
}
