use crate::Error;
use crate::ReferenceIterator;
use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

type PResult<T> = Result<T, Error>;

fn parse_utf8(a: &[u8]) -> PResult<&str> {
    std::str::from_utf8(a).map_err(|_| Error::ParserError(format!("invalid utf-8 in tag {:?}", a)))
}

#[derive(Debug, Clone)]
pub struct RisParser<'a> {
    start_tag: &'a [u8],
    end_tag: &'a [u8],
    allowed_tags: HashSet<[u8; 2]>,
    post_tag: &'a [u8],
}

impl RisParser<'_> {
    pub fn parse<'a>(&self, input: &'a [u8]) -> PResult<Vec<HashMap<&'a str, &'a str>>> {
        let mut complete_start_tag = Vec::from(self.start_tag);
        complete_start_tag.append(&mut Vec::from(self.post_tag));
        let mut complete_end_tag = Vec::from(self.end_tag);
        complete_end_tag.append(&mut Vec::from(self.post_tag));

        ReferenceIterator::new(&complete_start_tag, &complete_end_tag, &input)
            .into_iter()
            .par_bridge()
            .map(|ref_string| self.parse_reference(ref_string?))
            .collect()
    }

    fn parse_reference<'a>(&self, input: &'a [u8]) -> PResult<HashMap<&'a str, &'a str>> {
        let mut cursor = 0;
        let mut reference: HashMap<&str, &str> = HashMap::with_capacity(20);

        while cursor < input.len() {
            let tag = self.parse_tag(input, &mut cursor)?;
            // Ignore any end tag content.
            if tag == self.end_tag {
                break;
            }
            let content_start = cursor.clone();
            match self.parse_to_next_tag(input, &mut cursor) {
                Ok(s) => {
                    reference.insert(parse_utf8(tag)?, parse_utf8(s)?);
                }
                Err(Error::EOF) => {
                    reference.insert(parse_utf8(tag)?, parse_utf8(&input[content_start..])?);
                    break;
                }
                Err(e) => return Err(e),
            }
        }
        reference.shrink_to_fit();
        Ok(reference)
    }

    /// Parse a tag from the cursor position and return the tag without the post tag.
    ///
    /// Check if there is a tag followed by the post-tag string at the current position.
    /// If not, will return an error. If there is as tag, it will return it, and leave
    /// the cursor after the last character of the post-tag string.
    fn parse_tag<'a>(&self, input: &'a [u8], cursor: &mut usize) -> PResult<&'a [u8]> {
        let mut chars_iter = input[*cursor..].iter();
        let first_char = chars_iter.next().ok_or(Error::EOF)?;
        let second_char = chars_iter.next().ok_or(Error::EOF)?;
        if !self.allowed_tags.contains(&[*first_char, *second_char]) {
            return Err(Error::UnknownTag(format!("{}{}", first_char, second_char)));
        }
        let output = &input[*cursor..*cursor + 2];
        if !(&input[(*cursor + 2)..(*cursor + 2 + self.post_tag.len())] == self.post_tag) {
            return Err(Error::ParserError(
                "tag should be followed by post-tag string".to_owned(),
            ));
        }
        *cursor += 2 + self.post_tag.len();
        Ok(output)
    }

    fn parse_line<'a>(&self, input: &'a [u8], cursor: &mut usize) -> PResult<&'a [u8]> {
        let mut char_iter = input[*cursor..].iter().enumerate();
        let (idx, _) = char_iter.find(|(_, c)| *c == &b'\n').ok_or(Error::EOF)?;
        let output = &input[*cursor..(*cursor + idx)];
        *cursor += idx + 1;
        Ok(output)
    }

    // If I use a visitor, I can dispatch the content to the visitor as soon as parse_tag
    // succeeds and I don't need to run parse_tag twice.
    // Right now I return a slice of the input string. This means that multiline content
    // keeps the newlines in between the lines. If I return owned copies and create a new
    // string and remove the newlines while parsing.
    fn parse_to_next_tag<'a>(&self, input: &'a [u8], cursor: &mut usize) -> PResult<&'a [u8]> {
        if *cursor >= input.len() {
            return Err(Error::EOF);
        }
        let cursor_start = cursor.clone();
        loop {
            // Pass cursor clone, so actual cursor does not advance when checking tag.
            match self.parse_tag(input, &mut cursor.clone()) {
                Err(Error::EOF) => return Ok(&input[cursor_start..]),
                Err(_) => self.parse_line(input, cursor)?,
                Ok(_) => break,
            };
            if *cursor >= input.len() {
                return Err(Error::EOF);
            }
        }
        // Remove the last newline from the output if anything was parsed.
        if *cursor > cursor_start {
            Ok(&input[cursor_start..*cursor - 1])
        } else {
            Ok(&[])
        }
    }
}

impl Default for RisParser<'_> {
    fn default() -> Self {
        Self {
            start_tag: b"TY",
            end_tag: b"ER",
            post_tag: b"  - ",
            allowed_tags: HashSet::from([
                [b'T', b'Y'],
                [b'A', b'1'],
                [b'A', b'2'],
                [b'A', b'3'],
                [b'A', b'4'],
                [b'A', b'B'],
                [b'A', b'D'],
                [b'A', b'N'],
                [b'A', b'U'],
                [b'C', b'1'],
                [b'C', b'2'],
                [b'C', b'3'],
                [b'C', b'4'],
                [b'C', b'5'],
                [b'C', b'6'],
                [b'C', b'7'],
                [b'C', b'8'],
                [b'C', b'A'],
                [b'C', b'N'],
                [b'C', b'Y'],
                [b'D', b'A'],
                [b'D', b'B'],
                [b'D', b'O'],
                [b'D', b'P'],
                [b'E', b'T'],
                [b'E', b'P'],
                [b'I', b'D'],
                [b'I', b'S'],
                [b'J', b'2'],
                [b'J', b'A'],
                [b'J', b'F'],
                [b'J', b'O'],
                [b'K', b'W'],
                [b'L', b'1'],
                [b'L', b'2'],
                [b'L', b'4'],
                [b'L', b'A'],
                [b'L', b'B'],
                [b'M', b'1'],
                [b'M', b'3'],
                [b'N', b'1'],
                [b'N', b'2'],
                [b'N', b'V'],
                [b'O', b'P'],
                [b'P', b'B'],
                [b'P', b'Y'],
                [b'R', b'I'],
                [b'R', b'N'],
                [b'R', b'P'],
                [b'S', b'E'],
                [b'S', b'N'],
                [b'S', b'P'],
                [b'S', b'T'],
                [b'T', b'1'],
                [b'T', b'2'],
                [b'T', b'3'],
                [b'T', b'A'],
                [b'T', b'I'],
                [b'T', b'T'],
                [b'U', b'R'],
                [b'V', b'L'],
                [b'Y', b'1'],
                [b'Y', b'2'],
                [b'U', b'K'],
                [b'E', b'R'],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let start_tag = b"TY";
        let end_tag = b"ER";
        let post_tag = b"  - ";
        let allowed_tags = HashSet::from([[b'A', b'B']]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };

        let input = b"aa\nb";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(input, &mut cursor).unwrap(), b"aa");
        assert_eq!(cursor, 3);

        let input = &[];
        let mut cursor = 0;
        assert_eq!(parser.parse_line(input, &mut cursor), Err(Error::EOF));

        let input = b"foobar";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(input, &mut cursor), Err(Error::EOF));

        let input = b"\n\n";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(input, &mut cursor), Ok(&input[0..0]));
        assert_eq!(cursor, 1);
        assert_eq!(parser.parse_line(input, &mut cursor), Ok(&input[0..0]));
        assert_eq!(cursor, 2);
        assert_eq!(parser.parse_line(input, &mut cursor), Err(Error::EOF));
    }

    #[test]
    fn test_parse_tag() {
        let start_tag = b"TY";
        let end_tag = b"ER";
        let post_tag = b"  - ";
        let allowed_tags = HashSet::from([[b'A', b'B']]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };

        let input = b"AB  - b";
        let mut cursor = 0;
        assert_eq!(parser.parse_tag(input, &mut cursor).unwrap(), b"AB");
        assert_eq!(cursor, 6);

        let input = b"AA  - b";
        let mut cursor = 0;
        assert!(parser.parse_tag(input, &mut cursor).is_err());

        let input = b"BB  - b";
        let mut cursor = 0;
        assert!(parser.parse_tag(input, &mut cursor).is_err());

        let input = b"AB - b";
        let mut cursor = 0;
        assert!(parser.parse_tag(input, &mut cursor).is_err());
    }

    #[test]
    fn test_parse_to_next_tag() {
        let start_tag = b"TY";
        let end_tag = b"ER";
        let post_tag = b"  - ";
        let allowed_tags = HashSet::from([[b'A', b'B']]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };
        let input = b"AB  - ";
        let mut cursor = 0;
        assert_eq!(parser.parse_to_next_tag(input, &mut cursor).unwrap(), []);
        assert_eq!(cursor, 0);

        let input = b"\nAB  - ";
        let mut cursor = 0;
        assert_eq!(parser.parse_to_next_tag(input, &mut cursor).unwrap(), []);
        assert_eq!(cursor, 1);

        let input = b"aa\nAB  - b";
        let mut cursor = 0;
        assert_eq!(parser.parse_to_next_tag(input, &mut cursor).unwrap(), b"aa");

        let input = b"aa\naa\naa\nAB  - b";
        let mut cursor = 0;
        assert_eq!(
            parser.parse_to_next_tag(input, &mut cursor).unwrap(),
            b"aa\naa\naa"
        );

        let input = b"aa\n\nAB  - b";
        let mut cursor = 0;
        assert_eq!(
            parser.parse_to_next_tag(input, &mut cursor).unwrap(),
            b"aa\n"
        );

        let input = b"aa\naa\n";
        let mut cursor = 0;
        assert!(parser.parse_to_next_tag(input, &mut cursor).is_err());
    }

    #[test]
    fn test_parse_reference() {
        let start_tag = b"TY";
        let end_tag = b"ER";
        let post_tag = b"  - ";
        let allowed_tags = HashSet::from([[b'A', b'A'], [b'A', b'B'], [b'T', b'Y'], [b'E', b'R']]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };

        let input = b"TY  - ref_type
AA  - val1
AB  - val2
ER  - 
aaa";
        let output = HashMap::from([("TY", "ref_type"), ("AA", "val1"), ("AB", "val2")]);
        assert_eq!(parser.parse_reference(input).unwrap(), output);
    }

    #[test]
    fn test_eof() {
        let parser = RisParser::default();
        let input = b"TY  - JOUR
A1  - author
ER  - ";
        assert!(parser.parse(input).is_ok());

        let input = b"TY  - JOUR
A1  - author
ER  - 
";
        assert!(parser.parse(input).is_ok());

        let input = b"TY  - JOUR
A1  - author
ER  - 
foo
bar   ";
        assert!(parser.parse(input).is_ok());
    }

    #[test]
    fn test_multiple_references() {
        let ref_string = b"1.
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
ER  - 
";
        let parser = RisParser::default();

        let references = parser.parse(ref_string).unwrap();
        assert_eq!(references.len(), 2);
        assert_eq!(*references[0].get("T1").unwrap(), "Title of reference");
        assert_eq!(*references[0].get("SP").unwrap(), "e0815");
        assert_eq!(*references[1].get("CY").unwrap(), "Germany");
        assert_eq!(*references[1].get("M1").unwrap(), "1228150341");
    }
}
