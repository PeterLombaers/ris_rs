use std::collections::{HashMap, HashSet};

type Error = &'static str;
type PResult<T> = Result<T, Error>;

// If I use a visitor, I can check the allowed tags in the visitor.
pub struct RisParser<'a> {
    start_tag: &'a str,
    end_tag: &'a str,
    allowed_tags: HashSet<(char, char)>,
    post_tag: &'a str,
}

impl RisParser<'_> {
    pub fn parse<'a>(&self, input: &'a str) -> PResult<Vec<HashMap<&'a str, &'a str>>> {
        let mut cursor = 0;
        let mut references = Vec::new();
        // Skip BOM if it's there.
        if input.chars().next() == Some('\u{feff}') {
            cursor += 3
        }
        self.parse_to_next_tag(input, &mut cursor)?;
        while cursor < input.len() {
            references.push(self.parse_reference(input, &mut cursor)?);
            match self.parse_to_next_tag(input, &mut cursor) {
                Ok(_) => continue,
                Err("EOF") => break,
                Err(e) => return Err(e),
            };
        }
        Ok(references)
    }

    fn parse_tag<'a>(&self, input: &'a str, cursor: &mut usize) -> PResult<&'a str> {
        let mut chars_iter = input[*cursor..].chars();
        let first_char = chars_iter.next().ok_or("EOF")?;
        let second_char = chars_iter.next().ok_or("EOF")?;
        if !self.allowed_tags.contains(&(first_char, second_char)) {
            return Err("tag should be in the list of allowed tags");
        }
        let output = &input[*cursor..*cursor + 2];
        if !(&input[(*cursor + 2)..(*cursor + 2 + self.post_tag.len())] == self.post_tag) {
            return Err("tag should be followed by post-tag string");
        }
        *cursor += 2 + self.post_tag.len();
        Ok(output)
    }

    fn parse_line<'a>(&self, input: &'a str, cursor: &mut usize) -> PResult<&'a str> {
        let mut char_iter = input[*cursor..].chars().enumerate();
        let (idx, _) = char_iter.find(|(_, c)| *c == '\n').ok_or("EOF")?;
        let output = &input[*cursor..(*cursor + idx)];
        *cursor += idx + 1;
        Ok(output)
    }

    // If I use a visitor, I can dispatch the content to the visitor as soon as parse_tag
    // succeeds and I don't need to run parse_tag twice.
    // Right now I return a slice of the input string. This means that multiline content
    // keeps the newlines in between the lines. If I return owned copies and create a new
    // string and remove the newlines while parsing.
    fn parse_to_next_tag<'a>(&self, input: &'a str, cursor: &mut usize) -> PResult<&'a str> {
        if *cursor >= input.len() {
            return Err("EOF");
        }
        let cursor_start = cursor.clone();
        loop {
            // Pass cursor clone, so actual cursor does not advance when checking tag.
            match self.parse_tag(input, &mut cursor.clone()) {
                Err("EOF") => return Err("EOF"),
                Err(_) => self.parse_line(input, cursor)?,
                Ok(_) => break,
            };
            if *cursor >= input.len() {
                return Err("EOF");
            }
        }
        // Remove the last newline from the output if anything was parsed.
        if *cursor > cursor_start {
            Ok(&input[cursor_start..*cursor - 1])
        } else {
            Ok("")
        }
    }

    fn parse_reference<'a>(
        &self,
        input: &'a str,
        cursor: &mut usize,
    ) -> PResult<HashMap<&'a str, &'a str>> {
        let mut reference: HashMap<&str, &str> = HashMap::new();
        let start_tag = self.parse_tag(input, cursor)?;
        if start_tag != self.start_tag {
            return Err("reference should start with the start tag");
        }
        let reference_type = self.parse_to_next_tag(input, cursor)?;
        reference.insert(start_tag, reference_type);
        // Extra check that cursor does not exceed input length, so that we definitely
        // break out of the while loop.
        while *cursor < input.len() {
            let tag = self.parse_tag(input, cursor)?;
            if tag == self.end_tag {
                break;
            }
            let content = self.parse_to_next_tag(input, cursor)?;
            reference.insert(tag, content);
        }
        // Since we didn't parse content after the end_tag we need to advance past the newline
        *cursor += 1;
        Ok(reference)
    }
}

impl Default for RisParser<'_> {
    fn default() -> Self {
        Self {
            start_tag: "TY",
            end_tag: "ER",
            post_tag: "  - ",
            allowed_tags: HashSet::from([
                ('T', 'Y'),
                ('A', '1'),
                ('A', '2'),
                ('A', '3'),
                ('A', '4'),
                ('A', 'B'),
                ('A', 'D'),
                ('A', 'N'),
                ('A', 'U'),
                ('C', '1'),
                ('C', '2'),
                ('C', '3'),
                ('C', '4'),
                ('C', '5'),
                ('C', '6'),
                ('C', '7'),
                ('C', '8'),
                ('C', 'A'),
                ('C', 'N'),
                ('C', 'Y'),
                ('D', 'A'),
                ('D', 'B'),
                ('D', 'O'),
                ('D', 'P'),
                ('E', 'T'),
                ('E', 'P'),
                ('I', 'D'),
                ('I', 'S'),
                ('J', '2'),
                ('J', 'A'),
                ('J', 'F'),
                ('J', 'O'),
                ('K', 'W'),
                ('L', '1'),
                ('L', '2'),
                ('L', '4'),
                ('L', 'A'),
                ('L', 'B'),
                ('M', '1'),
                ('M', '3'),
                ('N', '1'),
                ('N', '2'),
                ('N', 'V'),
                ('O', 'P'),
                ('P', 'B'),
                ('P', 'Y'),
                ('R', 'I'),
                ('R', 'N'),
                ('R', 'P'),
                ('S', 'E'),
                ('S', 'N'),
                ('S', 'P'),
                ('S', 'T'),
                ('T', '1'),
                ('T', '2'),
                ('T', '3'),
                ('T', 'A'),
                ('T', 'I'),
                ('T', 'T'),
                ('U', 'R'),
                ('V', 'L'),
                ('Y', '1'),
                ('Y', '2'),
                ('E', 'R'),
                ('U', 'K'),
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let start_tag = "TY";
        let end_tag = "ER";
        let post_tag = "  - ";
        let allowed_tags = HashSet::from([('A', 'B')]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };

        let input = "aa\nb";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(&input, &mut cursor).unwrap(), "aa");
        assert_eq!(cursor, 3);

        let input = "";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(&input, &mut cursor), Err("EOF"));

        let input = "foobar";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(&input, &mut cursor), Err("EOF"));

        let input = "\n\n";
        let mut cursor = 0;
        assert_eq!(parser.parse_line(&input, &mut cursor), Ok(""));
        assert_eq!(cursor, 1);
        assert_eq!(parser.parse_line(&input, &mut cursor), Ok(""));
        assert_eq!(cursor, 2);
        assert_eq!(parser.parse_line(&input, &mut cursor), Err("EOF"));
    }

    #[test]
    fn test_parse_tag() {
        let start_tag = "TY";
        let end_tag = "ER";
        let post_tag = "  - ";
        let allowed_tags = HashSet::from([('A', 'B')]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };

        let input = "AB  - b";
        let mut cursor = 0;
        assert_eq!(parser.parse_tag(&input, &mut cursor).unwrap(), "AB");
        assert_eq!(cursor, 6);

        let input = "AA  - b";
        let mut cursor = 0;
        assert!(parser.parse_tag(&input, &mut cursor).is_err());

        let input = "BB  - b";
        let mut cursor = 0;
        assert!(parser.parse_tag(&input, &mut cursor).is_err());

        let input = "AB - b";
        let mut cursor = 0;
        assert!(parser.parse_tag(&input, &mut cursor).is_err());
    }

    #[test]
    fn test_parse_to_next_tag() {
        let start_tag = "TY";
        let end_tag = "ER";
        let post_tag = "  - ";
        let allowed_tags = HashSet::from([('A', 'B')]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };
        let input = "AB  - ";
        let mut cursor = 0;
        assert_eq!(parser.parse_to_next_tag(input, &mut cursor).unwrap(), "");
        assert_eq!(cursor, 0);

        let input = "\nAB  - ";
        let mut cursor = 0;
        assert_eq!(parser.parse_to_next_tag(input, &mut cursor).unwrap(), "");
        assert_eq!(cursor, 1);

        let input = "aa\nAB  - b";
        let mut cursor = 0;
        assert_eq!(parser.parse_to_next_tag(input, &mut cursor).unwrap(), "aa");

        let input = "aa\naa\naa\nAB  - b";
        let mut cursor = 0;
        assert_eq!(
            parser.parse_to_next_tag(input, &mut cursor).unwrap(),
            "aa\naa\naa"
        );

        let input = "aa\naa\n";
        let mut cursor = 0;
        assert!(parser.parse_to_next_tag(input, &mut cursor).is_err());
    }

    #[test]
    fn test_parse_reference() {
        let start_tag = "TY";
        let end_tag = "ER";
        let post_tag = "  - ";
        let allowed_tags = HashSet::from([('A', 'A'), ('A', 'B'), ('T', 'Y'), ('E', 'R')]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
            post_tag,
        };

        let input = "TY  - ref_type
AA  - val1
AB  - val2
ER  - 
aaa";
        let output = HashMap::from([("TY", "ref_type"), ("AA", "val1"), ("AB", "val2")]);
        let mut cursor = 0;
        assert_eq!(parser.parse_reference(input, &mut cursor).unwrap(), output);
        assert_eq!(&input[cursor..], "aaa");
    }

    #[test]
    fn test_eof() {
        let parser = RisParser::default();
        let input = "TY  - JOUR
A1  - author
ER  - ";
        assert!(parser.parse(&input).is_ok());

        let input = "TY  - JOUR
A1  - author
ER  - 
";
        assert!(parser.parse(&input).is_ok());

        let input = "TY  - JOUR
A1  - author
ER  - 
foo
bar   ";
        assert!(parser.parse(&input).is_ok());
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
ER  - 
";
        let parser = RisParser::default();

        let references = parser.parse(&ref_string).unwrap();
        assert_eq!(references.len(), 2);
        assert_eq!(*references[0].get("T1").unwrap(), "Title of reference");
        assert_eq!(*references[0].get("SP").unwrap(), "e0815");
        assert_eq!(*references[1].get("CY").unwrap(), "Germany");
        assert_eq!(*references[1].get("M1").unwrap(), "1228150341");
    }
}
