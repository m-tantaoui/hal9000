#![feature(portable_simd)]

mod arithmetic;
mod array;
mod utils;
use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
use crate::arithmetic::add_pyfunc;
use crate::utils::{do_nothing, py_mat_type, py_vec_type};
use pyo3::wrap_pyfunction;

#[pymodule]
fn hal_rs(_py: Python, hal_module: &PyModule) -> PyResult<()> {
    println!("Deep learning using PyO3 (Rust Extension for Python)");
    hal_module.add_function(wrap_pyfunction!(do_nothing, hal_module)?)?;
    hal_module.add_function(wrap_pyfunction!(py_vec_type, hal_module)?)?;
    hal_module.add_function(wrap_pyfunction!(py_mat_type, hal_module)?)?;
    hal_module.add_function(wrap_pyfunction!(add_pyfunc, hal_module)?)?;
    Ok(())
}
