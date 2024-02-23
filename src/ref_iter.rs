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
pub struct ReferenceIterator<'a, 'b> {
    start_tag: &'a [u8],
    end_tag: &'a [u8],
    text: &'b [u8],
    cursor: Enumerate<std::slice::Iter<'b, u8>>,
}

impl<'a, 'b> ReferenceIterator<'a, 'b> {
    pub fn new(start_tag: &'a [u8], end_tag: &'a [u8], text: &'b [u8]) -> Self {
        let text_without_bom: &[u8];
        if &text[..3] == "\u{feff}".as_bytes() {
            text_without_bom = &text[3..]
        } else {
            text_without_bom = text
        }
        ReferenceIterator {
            start_tag,
            end_tag,
            text: text_without_bom,
            cursor: text_without_bom.iter().enumerate(),
        }
    }

    pub fn default(text: &'b [u8]) -> Self {
        ReferenceIterator::new("TY  - ".as_bytes(), "ER  - ".as_bytes(), text)
    }

    /// Move the cursor to the next newline character and return its index.
    fn take_line(&mut self) -> Option<usize> {
        loop {
            let (idx, c) = self.cursor.next()?;
            if *c == b'\n' {
                return Some(idx);
            }
        }
    }

    /// Check if the tag occurs at the current position.
    fn take_tag(&mut self, tag: &[u8]) -> TakeTagResult {
        let mut idx: usize = 0;
        for c in tag.iter() {
            match self.cursor.next() {
                None => return TakeTagResult::EOF,
                Some((current_idx, current_char)) => {
                    if *current_char == b'\n' {
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

impl<'a, 'b> Iterator for ReferenceIterator<'a, 'b> {
    type Item = PResult<&'b [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        // Parsing to first start tag.
        let start_idx;
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
        let mut ref_iter = ReferenceIterator::default("a\nbcd\negfh".as_bytes());
        assert_eq!(ref_iter.take_line(), Some(1));
        assert_eq!(ref_iter.take_line(), Some(5));
        assert_eq!(ref_iter.take_line(), None);
        assert_eq!(ref_iter.take_line(), None);
    }

    #[test]
    fn test_take_tag() {
        let mut ref_iter = ReferenceIterator::default("foobar\nbarbar\nfo\nbar\n".as_bytes());
        assert_eq!(
            ref_iter.take_tag("foo".as_bytes()),
            TakeTagResult::Present(0)
        );
        assert_eq!(ref_iter.cursor.next(), Some((3, &b'b')));
        ref_iter.take_line();
        assert_eq!(
            ref_iter.take_tag("foo".as_bytes()),
            TakeTagResult::NotPresent
        );
        assert_eq!(ref_iter.cursor.next(), Some((8, &b'a')));
        ref_iter.take_line();
        assert_eq!(ref_iter.take_tag("foo".as_bytes()), TakeTagResult::NewLine);
        assert_eq!(ref_iter.cursor.next(), Some((17, &b'b')));
        ref_iter.take_line();
        assert_eq!(ref_iter.take_tag("foo".as_bytes()), TakeTagResult::EOF);
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
        let mut ref_iter = ReferenceIterator::default(ref_string.as_bytes());
        assert_eq!(
            ref_iter.next(),
            Some(Ok("TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - "
                .as_bytes()))
        );

        assert_eq!(
            ref_iter.next(),
            Some(Ok("TY  - JOUR
ID  - 12345
T1  - The title of the reference
CY  - Germany
L2  - http://example2.com
UR  - http://example_url.com
ER  - "
                .as_bytes()))
        );

        assert_eq!(ref_iter.next(), None);
        assert_eq!(ref_iter.next(), None);
    }

    #[test]
    fn test_after_end_tag() {
        let ref_string = "1.
TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - 
foobar
";
        let mut ref_iter = ReferenceIterator::default(ref_string.as_bytes());
        assert_eq!(
            ref_iter.next(),
            Some(Ok("TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - "
                .as_bytes()))
        );
        assert_eq!(ref_iter.next(), None);
        assert_eq!(ref_iter.next(), None);
    }

    #[test]
    fn test_before_start_tag() {
        let ref_string = "
\n\nTY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - 
foobar
";
        let mut ref_iter = ReferenceIterator::default(ref_string.as_bytes());
        assert_eq!(
            ref_iter.next(),
            Some(Ok("TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - "
                .as_bytes()))
        );
        assert_eq!(ref_iter.next(), None);
        assert_eq!(ref_iter.next(), None);
    }

    #[test]
    fn test_double_byte_char() {
        let ref_string = "
TY  - ©
ER  - 

TY  - JOUR
ER  - 
";
        let mut ref_iter = ReferenceIterator::default(&ref_string.as_bytes());
        let first_ref = ref_iter.next().unwrap().unwrap();
        assert_eq!(first_ref.iter().next(), Some(&b'T'));
        let second_ref = ref_iter.next().unwrap().unwrap();
        assert_eq!(second_ref.iter().next(), Some(&b'T'));
        assert!(ref_iter.next().is_none());
    }

    #[test]
    fn test_bom() {
        let ref_string = "\u{feff}TY  - 
ER  - "
            .as_bytes();
        let mut ref_iter = ReferenceIterator::default(ref_string);
        assert_eq!(ref_iter.next(), Some(Ok(&ref_string[3..])));
        assert!(ref_iter.next().is_none());
    }

    #[test]
    fn test_empty_ref() {
        let ref_string = b"TY  - \nER  - ";
        let mut ref_iter = ReferenceIterator::default(ref_string);
        assert_eq!(ref_iter.next(), Some(Ok(&ref_string[..])));
        assert!(ref_iter.next().is_none());
    }
}
