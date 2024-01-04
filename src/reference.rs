use crate::Field;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Reference<'a> {
    ref_type: &'a str,
    fields: Vec<Field<'a>>,
}

impl<'a> Reference<'a> {
    pub fn new(ref_type: &'a str, fields: Vec<Field<'a>>) -> Self {
        Self { ref_type, fields }
    }

    pub fn fields(&self) -> &[Field<'_>] {
        self.fields.as_ref()
    }

    pub fn ref_type(&self) -> &str {
        self.ref_type
    }
}
