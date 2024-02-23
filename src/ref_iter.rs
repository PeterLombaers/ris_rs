use std::iter::Enumerate;

use crate::Error;

type PResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TakeTagResult {
    Present(usize),
    NotPresent,
    NewLine,
    EOF,
}

#[derive(Debug, Clone)]
pub struct ReferenceIterator<'a> {
    start_tag: &'a str,
    end_tag: &'a str,
    text: &'a str,
    cursor: Enumerate<std::str::Chars<'a>>,
}

impl<'a> ReferenceIterator<'a> {
    pub fn new(start_tag: &'a str, end_tag: &'a str, text: &'a str) -> Self {
        ReferenceIterator {
            start_tag,
            end_tag,
            text,
            cursor: text.chars().enumerate(),
        }
    }

    pub fn default(text: &'a str) -> Self {
        ReferenceIterator::new("TY  - ", "ER  - ", text)
    }

    /// Move the cursor to the next newline character and return its index.
    fn take_line(&mut self) -> Option<usize> {
        loop {
            let (idx, c) = self.cursor.next()?;
            if c == '\n' {
                return Some(idx);
            }
        }
    }

    /// Check if the tag occurs at the current position.
    fn take_tag(&mut self, tag: &str) -> TakeTagResult {
        let mut idx: usize = 0;
        for c in tag.chars() {
            match self.cursor.next() {
                None => return TakeTagResult::EOF,
                Some((current_idx, current_char)) => {
                    if current_char == '\n' {
                        return TakeTagResult::NewLine;
                    }
                    if current_char != c {
                        return TakeTagResult::NotPresent;
                    }
                    // current_char is tag char.
                    idx = current_idx
                }
            }
        }
        TakeTagResult::Present(idx + 1 - tag.len())
    }
}

impl<'a> Iterator for ReferenceIterator<'a> {
    type Item = PResult<&'a str>;

    fn next(&mut self) -> Option<Self::Item> {
        // Parsing to first start tag.
        let mut start_idx: usize = 0;
        loop {
            match self.take_tag(self.start_tag) {
                TakeTagResult::EOF => return None,
                TakeTagResult::NotPresent => {
                    self.take_line();
                }
                TakeTagResult::NewLine => {}
                TakeTagResult::Present(idx) => {
                    start_idx = idx;
                    break;
                }
            }
        }
        // Parsing to end tag
        loop {
            match self.take_tag(self.end_tag) {
                TakeTagResult::EOF => return Some(Err(Error::EOF)),
                TakeTagResult::NotPresent => {
                    self.take_line();
                }
                TakeTagResult::NewLine => {}
                TakeTagResult::Present(_) => match self.take_line() {
                    None => return Some(Ok(&self.text[start_idx..])),
                    Some(end_idx) => return Some(Ok(&self.text[start_idx..end_idx])),
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_line() {
        let mut ref_iter = ReferenceIterator::default("a\nbcd\negfh");
        assert_eq!(ref_iter.take_line(), Some(1));
        assert_eq!(ref_iter.take_line(), Some(5));
        assert_eq!(ref_iter.take_line(), None);
        assert_eq!(ref_iter.take_line(), None);
    }

    #[test]
    fn test_take_tag() {
        let mut ref_iter = ReferenceIterator::default("foobar\nbarbar\nfo\nbar\n");
        assert_eq!(ref_iter.take_tag("foo"), TakeTagResult::Present(0));
        assert_eq!(ref_iter.cursor.next(), Some((3, 'b')));
        ref_iter.take_line();
        assert_eq!(ref_iter.take_tag("foo"), TakeTagResult::NotPresent);
        assert_eq!(ref_iter.cursor.next(), Some((8, 'a')));
        ref_iter.take_line();
        assert_eq!(ref_iter.take_tag("foo"), TakeTagResult::NewLine);
        assert_eq!(ref_iter.cursor.next(), Some((17, 'b')));
        ref_iter.take_line();
        assert_eq!(ref_iter.take_tag("foo"), TakeTagResult::EOF);
    }

    #[test]
    fn test_next() {
        let ref_string = "1.
TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - 

2.
TY  - JOUR
ID  - 12345
T1  - The title of the reference
CY  - Germany
L2  - http://example2.com
UR  - http://example_url.com
ER  - 
";
        let mut ref_iter = ReferenceIterator::default(ref_string);
        assert_eq!(ref_iter.next(), Some(Ok("TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - ")));

        assert_eq!(ref_iter.next(), Some(Ok("TY  - JOUR
ID  - 12345
T1  - The title of the reference
CY  - Germany
L2  - http://example2.com
UR  - http://example_url.com
ER  - ")));

        assert_eq!(ref_iter.next(), None);
        assert_eq!(ref_iter.next(), None);
    }
}
