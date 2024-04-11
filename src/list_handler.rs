use std::collections::HashMap;
use std::collections::HashSet;

use crate::hashmap_handler::HashMapHandler;
use crate::Error;
use crate::PResult;

fn parse_utf8(a: &[u8]) -> PResult<&str> {
    std::str::from_utf8(a).map_err(|_| Error::ParserError(format!("invalid utf-8 in tag {:?}", a)))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOrItem<T> {
    List(Vec<T>),
    Item(T),
}

#[derive(Debug, Clone)]
pub struct ListHandler<'a, 'b, const N: usize> {
    handler: HashMapHandler<'a, 'b, N>,
    list_tags: &'a HashSet<&'a [u8; N]>,
    lists: HashMap<&'b str, Vec<&'b str>>,
}

impl<'a, 'b, const N: usize> ListHandler<'a, 'b, N> {
    pub fn new(handler: HashMapHandler<'a, 'b, N>, list_tags: &'a HashSet<&'a [u8; N]>) -> Self {
        Self {
            handler,
            list_tags,
            lists: HashMap::with_capacity(list_tags.len()),
        }
    }

    pub fn start_tag(&self) -> &'a [u8; N] {
        self.handler.start_tag()
    }

    pub fn end_tag(&self) -> &'a [u8; N] {
        self.handler.end_tag()
    }

    pub fn allowed_tags(&self) -> &'a HashSet<&'a [u8; N]> {
        self.handler.allowed_tags()
    }

    pub fn handle(&mut self, tag: &'b [u8], content: &'b [u8]) -> PResult<()> {
        if tag.len() != N {
            return Err(Error::UnknownTag(format!("tag should have length {}", N)));
        }
        let tag: &[u8; N] = tag.try_into().unwrap();
        if self.list_tags.contains(tag) {
            let utf_tag = parse_utf8(tag)?;
            match self.lists.get_mut(utf_tag) {
                Some(vec) => {
                    vec.push(parse_utf8(content)?);
                }
                None => {
                    self.lists.insert(&utf_tag, vec![parse_utf8(content)?]);
                }
            }
            Ok(())
        } else {
            self.handler.handle(tag, content)
        }
    }

    pub fn finish(self) -> HashMap<&'b str, ListOrItem<&'b str>> {
        self.handler
            .finish()
            .iter()
            .map(|(&k, &v)| (k, ListOrItem::Item(v)))
            .chain(
                self.lists
                    .into_iter()
                    .map(|(k, v)| (k, ListOrItem::List(v))),
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::handler;

    use super::*;

    #[test]
    fn test_list_handler() {
        let allowed_tags = HashSet::from([b"FOO", b"BAR", b"STA", b"END"]);
        let base_handler = HashMapHandler::new(b"STA", b"END", &allowed_tags);
        let list_tags = HashSet::from([b"FOO"]);
        let mut handler = ListHandler::new(base_handler, &list_tags);

        handler.handle(b"STA", b"0").unwrap();
        handler.handle(b"FOO", b"1").unwrap();
        handler.handle(b"BAR", b"2").unwrap();
        handler.handle(b"FOO", b"3").unwrap();
        handler.handle(b"BAR", b"4").unwrap();
        handler.handle(b"END", b"This is ignored").unwrap();

        assert_eq!(
            handler.finish(),
            HashMap::from([
                ("STA", ListOrItem::Item("0")),
                ("FOO", ListOrItem::List(vec!["1", "3"])),
                ("BAR", ListOrItem::Item("4")),
            ])
        );
    }
}
