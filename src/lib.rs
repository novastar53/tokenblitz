use pyo3::prelude::*;

pub fn tokenize_text(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}

#[pyfunction]
fn tokenize(input: String) -> PyResult<Vec<String>> {
    Ok(tokenize_text(&input))
}

#[pymodule]
fn tokenblitz(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    Ok(())
}
