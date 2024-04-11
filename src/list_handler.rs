use std::collections::HashMap;
use std::collections::HashSet;

use crate::hashmap_handler::HashMapHandler;
use crate::Error;
use crate::PResult;
use crate::utils::parse_utf8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOrItem<T> {
    List(Vec<T>),
    Item(T),
}

#[derive(Debug, Clone)]
pub struct ListHandler<'a, 'b, T, const N: usize> {
    handler: HashMapHandler<'a, 'b, T, N>,
    list_tags: &'a HashSet<&'a [u8; N]>,
    lists: HashMap<&'b str, Vec<T>>,
}

impl<'a, 'b, T, const N: usize> ListHandler<'a, 'b, T, N> {
    pub fn new(handler: HashMapHandler<'a, 'b, T, N>, list_tags: &'a HashSet<&'a [u8; N]>) -> Self {
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

    pub fn handle(&mut self, tag: &'b [u8], content: T) -> PResult<()> {
        if tag.len() != N {
            return Err(Error::UnknownTag(format!("tag should have length {}", N)));
        }
        let tag: &[u8; N] = tag.try_into().unwrap();
        if self.list_tags.contains(tag) {
            let utf_tag = parse_utf8(tag)?;
            match self.lists.get_mut(utf_tag) {
                Some(vec) => {
                    vec.push(content);
                }
                None => {
                    self.lists.insert(&utf_tag, vec![content]);
                }
            }
            Ok(())
        } else {
            self.handler.handle(tag, content)
        }
    }

    pub fn finish(self) -> HashMap<&'b str, ListOrItem<T>> {
        self.handler
            .finish()
            .into_iter()
            .map(|(k, v)| (k, ListOrItem::Item(v)))
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
    use super::*;

    #[test]
    fn test_list_handler() {
        let allowed_tags = HashSet::from([b"FOO", b"BAR", b"STA", b"END"]);
        let base_handler = HashMapHandler::new(b"STA", b"END", &allowed_tags);
        let list_tags = HashSet::from([b"FOO"]);
        let mut handler = ListHandler::new(base_handler, &list_tags);

        handler.handle(b"STA", "0").unwrap();
        handler.handle(b"FOO", "1").unwrap();
        handler.handle(b"BAR", "2").unwrap();
        handler.handle(b"FOO", "3").unwrap();
        handler.handle(b"BAR", "4").unwrap();
        handler.handle(b"END", "This is ignored").unwrap();

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
