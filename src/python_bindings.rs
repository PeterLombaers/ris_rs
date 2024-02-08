use crate::RisParser;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use std::collections::HashMap;

#[pyfunction]
fn parse(contents: &PyString) -> PyResult<Vec<HashMap<&str, &str>>> {
    let parser = RisParser::default();
    Ok(parser
        .parse(contents.to_str()?)
        .map_err(|e| PyValueError::new_err(e))?)
}

#[pymodule]
fn ris(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
