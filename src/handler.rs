use std::collections::{HashMap, HashSet};

use crate::Error;

type PResult<T> = Result<T, Error>;

fn parse_utf8(a: &[u8]) -> PResult<&str> {
    std::str::from_utf8(a).map_err(|_| Error::ParserError(format!("invalid utf-8 in tag {:?}", a)))
}

pub struct Handler<'a, 'b, const N: usize> {
    start_tag: &'a [u8; N],
    end_tag: &'a [u8; N],
    allowed_tags: HashSet<&'a [u8; N]>,
    state: HashMap<&'b str, &'b str>,
}

impl<'a, 'b, const N: usize> Handler<'a, 'b, N> {
    pub fn new(
        start_tag: &'a [u8; N],
        end_tag: &'a [u8; N],
        allowed_tags: HashSet<&'a [u8; N]>,
    ) -> Self {
        Handler {
            start_tag,
            end_tag,
            allowed_tags,
            state: HashMap::with_capacity(20),
        }
    }

    pub fn handle(&mut self, tag: &'b [u8], content: &'b [u8]) -> PResult<()> {
        if tag.len() != N {
            return Err(Error::UnknownTag(format!("tag should have length {}", N)))
        }
        let tag: &[u8;N] = tag.try_into().unwrap();
        if !self.allowed_tags.contains(tag) {
            return Err(Error::UnknownTag("tag should be in allowed tags".into()));
        }
        if tag != self.end_tag {
            self.state.insert(parse_utf8(tag)?, parse_utf8(content)?);
        }
        Ok(())
    }

    pub fn finish(mut self) -> HashMap<&'b str, &'b str> {
        self.state.shrink_to_fit();
        self.state
    }
}

impl Default for Handler<'_, '_, 6> {
    fn default() -> Self {
        Handler::new(
            b"TY  - ",
            b"ER  - ",
            HashSet::from([
                b"TY  - ", b"A1  - ", b"A2  - ", b"A3  - ", b"A4  - ", b"AB  - ", b"AD  - ",
                b"AN  - ", b"AU  - ", b"C1  - ", b"C2  - ", b"C3  - ", b"C4  - ", b"C5  - ",
                b"C6  - ", b"C7  - ", b"C8  - ", b"CA  - ", b"CN  - ", b"CY  - ", b"DA  - ",
                b"DB  - ", b"DO  - ", b"DP  - ", b"ET  - ", b"EP  - ", b"ID  - ", b"IS  - ",
                b"J2  - ", b"JA  - ", b"JF  - ", b"JO  - ", b"KW  - ", b"L1  - ", b"L2  - ",
                b"L4  - ", b"LA  - ", b"LB  - ", b"M1  - ", b"M3  - ", b"N1  - ", b"N2  - ",
                b"NV  - ", b"OP  - ", b"PB  - ", b"PY  - ", b"RI  - ", b"RN  - ", b"RP  - ",
                b"SE  - ", b"SN  - ", b"SP  - ", b"ST  - ", b"T1  - ", b"T2  - ", b"T3  - ",
                b"TA  - ", b"TI  - ", b"TT  - ", b"UR  - ", b"VL  - ", b"Y1  - ", b"Y2  - ",
                b"UK  - ", b"ER  - ",
            ]),
        )
    }
}
