use crate::content_iter::ContentIterator;
use crate::Error;
use crate::ReferenceIterator;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

type PResult<T> = Result<T, Error>;

fn parse_utf8(a: &[u8]) -> PResult<&str> {
    std::str::from_utf8(a).map_err(|_| Error::ParserError(format!("invalid utf-8 in tag {:?}", a)))
}

#[derive(Debug, Clone)]
pub struct RisParser<'a, const N: usize> {
    start_tag: &'a [u8; N],
    end_tag: &'a [u8; N],
    allowed_tags: HashSet<&'a [u8; N]>,
}

impl<const N: usize> RisParser<'_, N> {
    pub fn parse<'a>(&self, input: &'a [u8]) -> PResult<Vec<HashMap<&'a str, &'a str>>> {
        ReferenceIterator::new(self.start_tag, self.end_tag, &input)
            .into_iter()
            .par_bridge()
            .map(|ref_string| self.parse_reference(ref_string?))
            .collect()
    }

    fn parse_reference<'a>(&self, input: &'a [u8]) -> PResult<HashMap<&'a str, &'a str>> {
        let mut reference: HashMap<&str, &str> = HashMap::with_capacity(20);

        for res in ContentIterator::new(self.allowed_tags.clone(), input) {
            let (tag, content) = res?;
            if tag != self.end_tag {
                reference.insert(parse_utf8(tag)?, parse_utf8(content)?);
            }
        }
        reference.shrink_to_fit();
        Ok(reference)
    }
}

impl Default for RisParser<'_, 6> {
    fn default() -> Self {
        Self {
            start_tag: b"TY  - ",
            end_tag: b"ER  - ",
            allowed_tags: HashSet::from([
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference() {
        let start_tag = b"TY  - ";
        let end_tag = b"ER  - ";
        let allowed_tags = HashSet::from([b"AA  - ", b"AB  - ", b"TY  - ", b"ER  - "]);
        let parser = RisParser {
            start_tag,
            end_tag,
            allowed_tags,
        };

        let input = b"TY  - ref_type
AA  - val1
AB  - val2
ER  - ";
        let output = HashMap::from([("TY  - ", "ref_type"), ("AA  - ", "val1"), ("AB  - ", "val2")]);
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
        assert_eq!(*references[0].get("T1  - ").unwrap(), "Title of reference");
        assert_eq!(*references[0].get("SP  - ").unwrap(), "e0815");
        assert_eq!(*references[1].get("CY  - ").unwrap(), "Germany");
        assert_eq!(*references[1].get("M1  - ").unwrap(), "1228150341");
    }
}
