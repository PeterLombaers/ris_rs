#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    EOF,
    UnknownTag,
    ParserError(String),
}