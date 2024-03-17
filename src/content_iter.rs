use std::collections::HashSet;

use crate::Error;

type PResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TakeTagResult<'a> {
    Present(&'a [u8]),
    NotPresent,
    EOF,
}

/// Move the cursor to after the next newline character.
#[derive(Debug, Clone)]
pub struct ContentIterator<'a, 'b, const N: usize> {
    allowed_tags: HashSet<&'a [u8; N]>,
    text: &'b [u8],
    cursor: usize,
}

impl<'a, 'b, const N: usize> ContentIterator<'a, 'b, N> {
    pub fn new(allowed_tags: HashSet<&'a [u8; N]>, text: &'b [u8]) -> Self {
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

impl<'a, 'b> ContentIterator<'a, 'b, 6> {
    pub fn default(text: &'b [u8]) -> Self {
        let allowed_tags = HashSet::from([
            b"TY  - ", b"A1  - ", b"A2  - ", b"A3  - ", b"A4  - ", b"AB  - ", b"AD  - ", b"AN  - ",
            b"AU  - ", b"C1  - ", b"C2  - ", b"C3  - ", b"C4  - ", b"C5  - ", b"C6  - ", b"C7  - ",
            b"C8  - ", b"CA  - ", b"CN  - ", b"CY  - ", b"DA  - ", b"DB  - ", b"DO  - ", b"DP  - ",
            b"ET  - ", b"EP  - ", b"ID  - ", b"IS  - ", b"J2  - ", b"JA  - ", b"JF  - ", b"JO  - ",
            b"KW  - ", b"L1  - ", b"L2  - ", b"L4  - ", b"LA  - ", b"LB  - ", b"M1  - ", b"M3  - ",
            b"N1  - ", b"N2  - ", b"NV  - ", b"OP  - ", b"PB  - ", b"PY  - ", b"RI  - ", b"RN  - ",
            b"RP  - ", b"SE  - ", b"SN  - ", b"SP  - ", b"ST  - ", b"T1  - ", b"T2  - ", b"T3  - ",
            b"TA  - ", b"TI  - ", b"TT  - ", b"UR  - ", b"VL  - ", b"Y1  - ", b"Y2  - ", b"UK  - ",
            b"ER  - ",
        ]);
        ContentIterator::new(allowed_tags, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_line() {
        let mut content_iter = ContentIterator::default(b"foo\n\nbar");
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
            ContentIterator::default(b"TY  - foo bar").take_tag(),
            TakeTagResult::Present(b"TY  - ")
        );
        assert_eq!(
            ContentIterator::default(b"QQ  - foo bar").take_tag(),
            TakeTagResult::NotPresent
        );
        assert_eq!(
            ContentIterator::default(b"TY").take_tag(),
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
        let mut content_iter = ContentIterator::default(ref_bytes);
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
