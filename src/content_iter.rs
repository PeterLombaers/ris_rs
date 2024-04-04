use std::collections::HashSet;

use crate::Error;
use crate::PResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TakeTagResult<'a> {
    Present(&'a [u8]),
    NotPresent,
    EOF,
}

/// Move the cursor to after the next newline character.
#[derive(Debug, Clone)]
pub struct ContentIterator<'a, 'b, const N: usize> {
    allowed_tags: &'a HashSet<&'a [u8; N]>,
    text: &'b [u8],
    cursor: usize,
}

impl<'a, 'b, const N: usize> ContentIterator<'a, 'b, N> {
    pub fn new(allowed_tags: &'a HashSet<&'a [u8; N]>, text: &'b [u8]) -> Self {
        ContentIterator {
            allowed_tags,
            text,
            cursor: 0,
        }
    }

    fn take_line(&mut self) -> Option<()> {
        while self.cursor < self.text.len() {
            if self.text[self.cursor] == b'\n' {
                self.cursor += 1;
                return Some(());
            }
            self.cursor += 1;
        }
        None
    }

    /// Get a tag at the current position.
    ///
    /// The cursor is left in place.
    fn take_tag(&mut self) -> TakeTagResult<'b> {
        if self.text.len() < self.cursor + N {
            return TakeTagResult::EOF;
        }
        if self.allowed_tags.contains::<&[u8; N]>(
            &(self.text[self.cursor..(self.cursor + N)]
                .try_into()
                .expect("this slice is by definition of size N")),
        ) {
            TakeTagResult::Present(&self.text[self.cursor..(self.cursor + N)])
        } else {
            TakeTagResult::NotPresent
        }
    }
}

impl<'a, 'b, const N: usize> Iterator for ContentIterator<'a, 'b, N> {
    type Item = PResult<(&'b [u8], &'b [u8])>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.take_tag() {
            TakeTagResult::Present(tag) => {
                self.cursor += N;
                let content_start = self.cursor;
                loop {
                    if self.take_line().is_none() {
                        return Some(Ok((tag, &self.text[content_start..])));
                    } else {
                        match self.take_tag() {
                            TakeTagResult::Present(_) => {
                                // We subtract 1 from the cursor to remove the final newline character.
                                return Some(Ok((
                                    tag,
                                    &self.text[content_start..(self.cursor - 1)],
                                )));
                            }
                            TakeTagResult::NotPresent => continue,
                            TakeTagResult::EOF => {
                                return Some(Ok((tag, &self.text[content_start..])))
                            }
                        }
                    }
                }
            }
            TakeTagResult::EOF => None,
            TakeTagResult::NotPresent => Some(Err(Error::ParserError(
                "line should start with a tag".into(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_line() {
        let allowed_tags = HashSet::from([b""]);
        let mut content_iter = ContentIterator::new(&allowed_tags, b"foo\n\nbar");
        assert!(content_iter.take_line().is_some());
        assert_eq!(content_iter.cursor, 4);
        assert!(content_iter.take_line().is_some());
        assert_eq!(content_iter.cursor, 5);
        assert!(content_iter.take_line().is_none());
        assert!(content_iter.take_line().is_none());
    }

    #[test]
    fn test_take_tag() {
        assert_eq!(
            ContentIterator::new(&HashSet::from([b"TY  - "]), b"TY  - foo bar").take_tag(),
            TakeTagResult::Present(b"TY  - ")
        );
        assert_eq!(
            ContentIterator::new(&HashSet::from([b"TY  - "]), b"QQ  - foo bar").take_tag(),
            TakeTagResult::NotPresent
        );
        assert_eq!(
            ContentIterator::new(&HashSet::from([b"TY  - "]), b"TY").take_tag(),
            TakeTagResult::EOF
        );
    }

    #[test]
    fn test_next() {
        let ref_bytes = b"TY  - JOUR
ID  - 12345
A2  - Glattauer, Daniel
UR  - http://example_url.com
ER  - ";
        let allowed_tags = HashSet::from([b"TY  - ", b"A2  - ", b"ID  - ", b"UR  - ", b"ER  - "]);
        let mut content_iter = ContentIterator::new(
            &allowed_tags,
            ref_bytes,
        );
        assert_eq!(
            content_iter.next(),
            Some(Ok(("TY  - ".as_bytes(), "JOUR".as_bytes())))
        );

        assert_eq!(
            content_iter.next(),
            Some(Ok(("ID  - ".as_bytes(), "12345".as_bytes())))
        );
        assert_eq!(
            content_iter.next(),
            Some(Ok(("A2  - ".as_bytes(), "Glattauer, Daniel".as_bytes())))
        );
        assert_eq!(
            content_iter.next(),
            Some(Ok((
                "UR  - ".as_bytes(),
                "http://example_url.com".as_bytes()
            )))
        );
        assert_eq!(
            content_iter.next(),
            Some(Ok(("ER  - ".as_bytes(), "".as_bytes())))
        );
        assert_eq!(content_iter.next(), None);
    }
}
