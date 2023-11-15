use crate::array::PyArray1Wrapper;
use crate::array::PyArray1Wrapper::*;

use num::ToPrimitive;
use numpy::{Element, PyArray1};
use pyo3::prelude::{pyfunction, IntoPy, PyObject, PyResult, Python};
use rayon::prelude::*;

fn check_same_size(arr1_size: usize, arr2_size: usize) {
    // dimensionality check
    if arr1_size != arr2_size {
        let err_msg = format!(
            "arrays must  have the same length ({} != {})",
            arr1_size, arr2_size
        );

        panic!("{}", err_msg)
    }
}

// fn matmul<T1, T2>(arr1: &PyArray1<T1>, arr2: &PyArray1<T2>) -> Vec<f64> {
//     todo!()
// }

fn add_pyfunc_wrapper<'py, T1: Element + ToPrimitive + Sync, T2: Element + ToPrimitive + Sync>(
    py: Python<'py>,
    arr1: &PyArray1<T1>,
    arr2: &PyArray1<T2>,
) -> &'py PyArray1<f64> {
    check_same_size(arr1.len(), arr2.len());

    let arr1_slice = unsafe { arr1.as_slice().expect("Failed to extract numpy array") };

    let arr2_slice = unsafe { arr2.as_slice().expect("Failed to extract numpy array") };

    let res: Vec<f64> = (0..arr1.len())
        .into_par_iter()
        .map(|i| arr1_slice[i].to_f64().unwrap() + arr2_slice[i].to_f64().unwrap())
        .collect();

    return PyArray1::from_vec(py, res);
}

#[pyfunction]
#[pyo3(name = "add")]
pub fn add_pyfunc<'py>(
    py: Python<'py>,
    arr1: PyArray1Wrapper,
    arr2: PyArray1Wrapper,
) -> PyResult<PyObject> {
    let res = match (arr1, arr2) {
        // (UInt8PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (UInt8PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt8PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), Int8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt16PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), Int8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt32PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), Int8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UInt64PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), Int8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (UIntSizePyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int8PyArray1(arr1), UInt8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int8PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int8PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int8PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int8PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int8PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int8PyArray1(arr1), Int16PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int8PyArray1(arr1), Int32PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int8PyArray1(arr1), Int64PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int8PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int8PyArray1(arr1), Flt32PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int8PyArray1(arr1), Flt64PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int16PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int16PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int16PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int32PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int32PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Int64PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Int64PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), Int8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (IntSizePyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Flt32PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt32PyArray1(arr1), Flt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), UInt8PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), UInt16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), UInt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), UInt64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), UIntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), Int8PyArray1(arr2)) => add_pyfunc_wrapper(py, arr1, arr2).into_py(py),
        // (Flt64PyArray1(arr1), Int16PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), Int32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), Int64PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), IntSizePyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        // (Flt64PyArray1(arr1), Flt32PyArray1(arr2)) => {
        //     add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        // }
        (Flt64PyArray1(arr1), Flt64PyArray1(arr2)) => {
            add_pyfunc_wrapper(py, arr1, arr2).into_py(py)
        }
        _ => panic!(),
    };
    Ok(res)
}
