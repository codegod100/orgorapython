use ::orgora::parse_file;
use pyo3::prelude::*;


#[pyfunction]
fn parse_string(str: String) -> PyResult<String> {
    Ok(parse_file(str))
}

/// A Python module implemented in Rust.
#[pymodule]
fn orgora(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    Ok(())
}
