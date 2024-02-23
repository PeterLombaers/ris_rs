use crate::RisParser;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::collections::HashMap;

#[pyfunction]
fn parse(contents: &PyBytes) -> PyResult<Vec<HashMap<&str, &str>>> {
    let parser = RisParser::default();
    Ok(parser.parse(contents.as_bytes())?)
}

#[pymodule]
fn ris(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
