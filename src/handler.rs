use std::collections::HashSet;

use crate::PResult;

pub trait Handler<'a, 'b, S, T, const N: usize> {
    fn start_tag(&self) -> &'a [u8; N];
    fn end_tag(&self) -> &'a [u8; N];
    fn allowed_tags(&self) -> &'a HashSet<&'a [u8; N]>;
    fn handle(&mut self, tag: &'b [u8], content: S) -> PResult<()>;
    fn finish(self) -> T;
}
