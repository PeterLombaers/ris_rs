mod content_iter;
mod error;
mod hashmap_handler;
mod list_handler;
mod parser;
mod python_bindings;
mod ref_iter;

pub type PResult<T> = Result<T, Error>;

pub use error::Error;
pub use parser::RisParser;
pub use ref_iter::ReferenceIterator;