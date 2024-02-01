use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

use crate::RisParser;

#[pyfunction]
fn parse(contents: &str) -> PyResult<Vec<HashMap<&str, &str>>> {
    let parser = RisParser::default();
    Ok(parser.parse(&contents).map_err(|e| PyValueError::new_err(e))?)
}

#[pymodule]
fn ris(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
