use pyo3::prelude::*;

#[pyfunction]
fn tokenize(input: String) -> PyResult<Vec<String>> {
    let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    Ok(words)
}

#[pymodule]
fn tokenblitz(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize, m)?)?;
    Ok(())
}
