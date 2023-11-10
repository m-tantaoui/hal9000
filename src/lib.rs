use pyo3::prelude::{pymodule, PyModule, PyResult, Python};

#[pymodule]
fn hal_rs(_py: Python, _module: &PyModule) -> PyResult<()> {
    println!("Deep learning using PyO3 (Rust Extension for Python)");
    Ok(())
}
