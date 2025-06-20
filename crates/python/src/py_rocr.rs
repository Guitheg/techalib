/*
    BSD 3-Clause License

    Copyright (c) 2025, Guillaume GOBIN (Guitheg)

    Redistribution and use in source and binary forms, with or without modification,
    are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice,
    this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation and/or
    other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors
    may be used to endorse or promote products derived from this software without
    specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
    AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
    WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
    DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
    FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
    DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
    SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
    OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
    THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/*
    List of contributors:
    - Guitheg: Initial implementation
*/

use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, pymethods, Py, PyResult, Python};
use techalib::indicators::rocr::{rocr_into, RocrState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "RocrState")]
#[derive(Debug, Clone)]
pub struct PyRocrState {
    #[pyo3(get)]
    pub prev_rocr: Float,
    #[pyo3(get)]
    pub prev_roc_window: Vec<Float>,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyRocrState {
    #[new]
    pub fn new(prev_rocr: Float, prev_roc_window: Vec<Float>, period: usize) -> Self {
        PyRocrState {
            prev_rocr,
            prev_roc_window,
            period,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
impl From<RocrState> for PyRocrState {
    fn from(state: RocrState) -> Self {
        PyRocrState {
            prev_rocr: state.prev_rocr,
            prev_roc_window: state.prev_roc_window.into(),
            period: state.period,
        }
    }
}

impl From<PyRocrState> for RocrState {
    fn from(py_state: PyRocrState) -> Self {
        RocrState {
            prev_rocr: py_state.prev_rocr,
            prev_roc_window: py_state.prev_roc_window.into(),
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (data, period = 10, release_gil = false))]
pub(crate) fn rocr(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyRocrState)> {
    let data_slice = data.as_slice()?;
    let len = data.len();

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| rocr_into(data_slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = rocr_into(data_slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, rocr_state))]
pub(crate) fn rocr_next(new_value: Float, rocr_state: PyRocrState) -> PyResult<PyRocrState> {
    let mut rocr_state: RocrState = rocr_state.into();
    let sample = new_value;
    rocr_state
        .update(sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(rocr_state.into())
}
