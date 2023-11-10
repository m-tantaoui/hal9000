use num_complex::{Complex32, Complex64};
use numpy::{PyArray1, PyArray2};
use pyo3::prelude::FromPyObject;

/// Mapping numpy 1D arrays
/// with the correspondant rust-numpy PyArray wrapper
#[derive(FromPyObject)]
pub enum PyArray1Wrapper<'py> {
    UInt8PyArray1(&'py PyArray1<u8>),
    UInt16PyArray1(&'py PyArray1<u16>),
    UInt32PyArray1(&'py PyArray1<u32>),
    UInt64PyArray1(&'py PyArray1<u64>),
    UIntSizePyArray1(&'py PyArray1<usize>),
    Int8PyArray1(&'py PyArray1<i8>),
    Int16PyArray1(&'py PyArray1<i16>),
    Int32PyArray1(&'py PyArray1<i32>),
    Int64PyArray1(&'py PyArray1<i64>),
    IntSizePyArray1(&'py PyArray1<isize>),
    Flt32PyArray1(&'py PyArray1<f32>),
    Flt64PyArray1(&'py PyArray1<f64>),
    Cplx32PyArray1(&'py PyArray1<Complex32>),
    Cplx64PyArray1(&'py PyArray1<Complex64>),
}

/// Mapping numpy 2D array dtypes with some rust primary types
#[derive(FromPyObject)]
pub enum PyArray2Wrapper<'py> {
    UInt8PyArray2(&'py PyArray2<u8>),
    UInt16PyArray2(&'py PyArray2<u16>),
    UInt32PyArray2(&'py PyArray2<u32>),
    UInt64PyArray2(&'py PyArray2<u64>),
    UIntSizePyArray2(&'py PyArray2<usize>),
    Int8PyArray2(&'py PyArray2<i8>),
    Int16PyArray2(&'py PyArray2<i16>),
    Int32PyArray2(&'py PyArray2<i32>),
    Int64PyArray2(&'py PyArray2<i64>),
    IntSizePyArray2(&'py PyArray2<isize>),
    Flt32PyArray2(&'py PyArray2<f32>),
    Flt64PyArray2(&'py PyArray2<f64>),
    Cplx32PyArray2(&'py PyArray2<Complex32>),
    Cplx64PyArray2(&'py PyArray2<Complex64>),
}
