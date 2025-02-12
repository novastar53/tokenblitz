use pyo3::prelude::*;

#[pyfunction]
fn say_hello(name: Option<&str>) -> PyResult<String> {
    match name {
        Some(n) => Ok(format!("Hello, {}!", n)),
        None => Ok(String::from("Hello, World!"))
    }
}

#[pymodule]
fn hello_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}
