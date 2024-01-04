use crate::tag::Tag;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Field<'a> {
    tag: Tag,
    content: &'a str,
}

impl<'a> Field<'a> {
    pub fn new(tag: Tag, content: &'a str) -> Self {
        Self { tag, content }
    }

    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn content(&self) -> &str {
        self.content
    }
}
