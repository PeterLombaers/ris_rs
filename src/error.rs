use pyo3::exceptions::PyException;
use pyo3::PyErr;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    EOF,
    UnknownTag(String),
    ParserError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::EOF => "End of input reached during parsing".to_owned(),
            Self::UnknownTag(s) => format!("Unknown tag encountered: {}", &s),
            Self::ParserError(s) => s.to_string(),
        };
        write!(f, "{}", message)
    }
}

impl std::convert::From<Error> for PyErr {
    fn from(value: Error) -> Self {
        PyException::new_err(value.to_string())
    }
}