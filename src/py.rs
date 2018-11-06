use pyo3::prelude::*;

use super::data::*;
use super::parsers::*;


#[pyfunction]
fn parse(_file: String) -> PyResult<((i32, i32), Vec<f32>)> {
    Ok(((1, 1), vec!(1.0f32)))
}

#[pymodinit]
fn ser_parser(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_function!(parse))?;

    Ok(())
}
