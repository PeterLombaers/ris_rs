use std::collections::{HashMap, HashSet};

use crate::utils::parse_utf8;
use crate::Error;
use crate::Handler;
use crate::PResult;

#[derive(Debug, Clone)]
pub struct HashMapHandler<'a, 'b, T, const N: usize> {
    start_tag: &'a [u8; N],
    end_tag: &'a [u8; N],
    allowed_tags: &'a HashSet<&'a [u8; N]>,
    state: HashMap<&'b str, T>,
}

impl<'a, 'b, T, const N: usize> HashMapHandler<'a, 'b, T, N> {
    pub fn new(
        start_tag: &'a [u8; N],
        end_tag: &'a [u8; N],
        allowed_tags: &'a HashSet<&'a [u8; N]>,
    ) -> Self {
        Self {
            start_tag,
            end_tag,
            allowed_tags,
            state: HashMap::with_capacity(20),
        }
    }
}

impl<'a, 'b, T, const N: usize> Handler<'a, 'b, T, HashMap<&'b str, T>, N>
    for HashMapHandler<'a, 'b, T, N>
{
    fn handle(&mut self, tag: &'b [u8], content: T) -> PResult<()> {
        if tag.len() != N {
            return Err(Error::UnknownTag(format!("tag should have length {}", N)));
        }
        let tag: &[u8; N] = tag.try_into().unwrap();
        if !self.allowed_tags.contains(tag) {
            return Err(Error::UnknownTag("tag should be in allowed tags".into()));
        }
        if tag != self.end_tag {
            self.state.insert(parse_utf8(tag)?, content);
        }
        Ok(())
    }

    fn finish(mut self) -> HashMap<&'b str, T> {
        self.state.shrink_to_fit();
        self.state
    }

    fn start_tag(&self) -> &'a [u8; N] {
        self.start_tag
    }

    fn end_tag(&self) -> &'a [u8; N] {
        self.end_tag
    }

    fn allowed_tags(&self) -> &'a HashSet<&'a [u8; N]> {
        self.allowed_tags
    }
}
