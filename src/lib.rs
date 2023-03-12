use orgora::parse_file;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn parse_string(str: String) -> PyResult<String> {
    Ok(parse_file(str))
}

/// A Python module implemented in Rust.
#[pymodule]
fn orgorapython(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    Ok(())
}
