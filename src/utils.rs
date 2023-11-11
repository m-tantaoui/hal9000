use std::sync::Arc;

use crate::array::PyArray1Wrapper;
use crate::array::PyArray1Wrapper::*;

use crate::array::PyArray2Wrapper;
use crate::array::PyArray2Wrapper::*;

use arrow::array::Datum;
use arrow::array::{make_array, Array, ArrayData, Int32Array};
use arrow::compute::kernels;
use arrow::pyarrow::PyArrowType;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::{pyfunction, PyResult, Python};
#[pyfunction]
#[pyo3(name = "do_nothing")]
pub fn do_nothing() {
    println!("this function does nothing");
}

#[pyfunction]
#[pyo3(name = "_vec_type")]
pub fn py_vec_type<'py>(_py: Python<'py>, py_vec: PyArray1Wrapper) -> PyResult<String> {
    let t = match py_vec {
        UInt8PyArray1(_) => "uint8".to_string(),
        UInt16PyArray1(_) => "unit16".to_string(),
        UInt32PyArray1(_) => "uint32".to_string(),
        UInt64PyArray1(_) => "uint64".to_string(),
        UIntSizePyArray1(_) => "uint_size".to_string(),
        Int8PyArray1(_) => "int8".to_string(),
        Int16PyArray1(_) => "int16".to_string(),
        Int32PyArray1(_) => "int32".to_string(),
        Int64PyArray1(_) => "int64".to_string(),
        IntSizePyArray1(_) => "int_size".to_string(),
        Flt32PyArray1(_) => "float32".to_string(),
        Flt64PyArray1(_) => "float64".to_string(),
        Cplx32PyArray1(_) => "complex32".to_string(),
        Cplx64PyArray1(_) => "complex64".to_string(),
    };
    Ok(t)
}

#[pyfunction]
#[pyo3(name = "_mat_type")]
pub fn py_mat_type<'py>(_py: Python<'py>, py_vec: PyArray2Wrapper) -> PyResult<String> {
    let t = match py_vec {
        UInt8PyArray2(_) => "uint8".to_string(),
        UInt16PyArray2(_) => "unit16".to_string(),
        UInt32PyArray2(_) => "uint32".to_string(),
        UInt64PyArray2(_) => "uint64".to_string(),
        UIntSizePyArray2(_) => "uint_size".to_string(),
        Int8PyArray2(_) => "int8".to_string(),
        Int16PyArray2(_) => "int16".to_string(),
        Int32PyArray2(_) => "int32".to_string(),
        Int64PyArray2(_) => "int64".to_string(),
        IntSizePyArray2(_) => "int_size".to_string(),
        Flt32PyArray2(_) => "float32".to_string(),
        Flt64PyArray2(_) => "float64".to_string(),
        Cplx32PyArray2(_) => "complex32".to_string(),
        Cplx64PyArray2(_) => "complex64".to_string(),
    };
    Ok(t)
}

#[pyfunction]
#[pyo3(name = "add")]
pub fn addition(
    array1: PyArrowType<ArrayData>,
    array2: PyArrowType<ArrayData>,
) -> PyResult<PyArrowType<ArrayData>> {
    // Extract from PyArrowType wrapper
    let array_data1 = array1.0;
    let array_data2 = array2.0;

    // Convert ArrayData to ArrayRef
    let array_ref1: Arc<dyn Array> = make_array(array_data1);
    let array_ref2: Arc<dyn Array> = make_array(array_data2);

    // downcasting to int32
    let array1: &Int32Array = array_ref1
        .as_any()
        .downcast_ref()
        .ok_or_else(|| PyValueError::new_err("expected int32 array"))?;
    let array2: &Int32Array = array_ref2
        .as_any()
        .downcast_ref()
        .ok_or_else(|| PyValueError::new_err("expected int32 array"))?;

    let addition_result = kernels::numeric::add(array1, array2)
        .expect("There is a problem with the Addition Kernel in Arrow");

    let addition = addition_result.get().0;
    Ok(PyArrowType(addition.to_data()))
}
