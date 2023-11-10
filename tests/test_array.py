import numpy as np
import pytest

from hal_rs import _mat_type, _vec_type

types: list[type] = [
    np.complex128,
    np.complex64,
    np.float32,
    np.float64,
    np.int16,
    np.int32,
    np.int64,
    np.int8,
    np.longlong,
    np.uint16,
    np.uint32,
    np.uint64,
    np.uint8,
    np.ulonglong,
]


@pytest.mark.parametrize("t", types)
def test_types(t: type):
    ones_vec: np.ndarray = np.ones(5, dtype=t)
    assert _vec_type(ones_vec)
    ones_mat: np.ndarray = np.ones((5, 5), dtype=t)
    assert _mat_type(ones_mat)
