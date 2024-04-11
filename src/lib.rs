mod content_iter;
mod error;
mod handler;
mod hashmap_handler;
mod list_handler;
mod parser;
mod python_bindings;
mod ref_iter;
mod utils;

pub type PResult<T> = Result<T, Error>;

pub use error::Error;
pub use handler::Handler;
pub use parser::RisParser;
pub use ref_iter::ReferenceIterator;