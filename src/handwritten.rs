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
    pub fn parse_ris<'a>(&self, input: &mut &'a str) -> PResult<Vec<HashMap<&'a str, &'a str>>> {
        todo!()
    }

    fn parse_tag<'a>(&self, input: &'a str, cursor: &mut usize) -> PResult<&'a str> {
        let mut chars_iter = input[*cursor..].chars();
        let first_char = chars_iter
            .next()
            .ok_or("input should contain at least 2 more characters")?;
        let second_char = chars_iter
            .next()
            .ok_or("input should contain at least 2 more characters")?;
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
        let (idx, _) = char_iter
            .find(|(_, c)| *c == '\n')
            .ok_or("line should end with newline character")?;
        let output = &input[*cursor..(*cursor + idx)];
        *cursor += idx + 1;
        Ok(output)
    }

    // If I use a visitor, I can dispatch the content to the visitor as soon as parse_tag
    // succeeds and I don't need to run parse_tag twice.
    // Right now I return a slice of the input string. This means that multiline content
    // keeps the newlines in between the lines. If I return owned copies and create a new
    // string and remove the newlines while parsing.
    fn parse_content<'a>(&self, input: &'a str, cursor: &mut usize) -> PResult<&'a str> {
        dbg!(&input, &cursor);
        let cursor_start = cursor.clone();
        while self.parse_tag(input, cursor).is_err() {
            self.parse_line(input, cursor)?;
        }
        // Move back cursor since it succesfully parsed a tag.
        *cursor -= 2 + self.post_tag.len();
        // Remove the last newline from the output.
        Ok(&input[cursor_start..*cursor - 1])
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
        let reference_type = self.parse_content(input, cursor)?;
        reference.insert(start_tag, reference_type);
        // Extra check that cursor does not exceed input length, so that we definitely
        // break out of the while loop.
        while *cursor < input.len() {
            let tag = self.parse_tag(input, cursor)?;
            if tag == self.end_tag {
                break;
            }
            let content = self.parse_content(input, cursor)?;
            reference.insert(tag, content);
        }
        // Since we didn't parse content after the end_tag we need to advance past the newline
        *cursor += 1;
        Ok(reference)
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
        assert_eq!(cursor, 3)
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
    fn test_parse_content() {
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

        let input = "aa\nAB  - b";
        let mut cursor = 0;
        assert_eq!(parser.parse_content(input, &mut cursor).unwrap(), "aa");

        let input = "aa\naa\naa\nAB  - b";
        let mut cursor = 0;
        assert_eq!(
            parser.parse_content(input, &mut cursor).unwrap(),
            "aa\naa\naa"
        );

        let input = "aa\naa\n";
        let mut cursor = 0;
        assert!(parser.parse_content(input, &mut cursor).is_err());
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
}
