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
use techalib::indicators::minus_di::{minus_di_into, MinusDiSample, MinusDiState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "MinusDiState")]
#[derive(Debug, Clone)]
pub struct PyMinusDiState {
    #[pyo3(get)]
    pub prev_minus_di: Float,
    #[pyo3(get)]
    pub prev_minus_dm: Float,
    #[pyo3(get)]
    pub prev_true_range: Float,
    #[pyo3(get)]
    pub prev_high: Float,
    #[pyo3(get)]
    pub prev_low: Float,
    #[pyo3(get)]
    pub prev_close: Float,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyMinusDiState {
    #[new]
    pub fn new(
        prev_minus_di: Float,
        prev_minus_dm: Float,
        prev_true_range: Float,
        prev_high: Float,
        prev_low: Float,
        prev_close: Float,
        period: usize,
    ) -> Self {
        PyMinusDiState {
            prev_minus_di,
            prev_minus_dm,
            prev_true_range,
            prev_high,
            prev_low,
            prev_close,
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
impl From<MinusDiState> for PyMinusDiState {
    fn from(state: MinusDiState) -> Self {
        PyMinusDiState {
            prev_minus_di: state.prev_minus_di,
            prev_minus_dm: state.prev_minus_dm,
            prev_true_range: state.prev_true_range,
            prev_high: state.prev_high,
            prev_low: state.prev_low,
            prev_close: state.prev_close,
            period: state.period,
        }
    }
}

impl From<PyMinusDiState> for MinusDiState {
    fn from(py_state: PyMinusDiState) -> Self {
        MinusDiState {
            prev_minus_di: py_state.prev_minus_di,
            prev_minus_dm: py_state.prev_minus_dm,
            prev_true_range: py_state.prev_true_range,
            prev_high: py_state.prev_high,
            prev_low: py_state.prev_low,
            prev_close: py_state.prev_close,
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (high, low, close, period = 14, release_gil = false))]
pub(crate) fn minus_di(
    py: Python,
    high: PyReadonlyArray1<Float>,
    low: PyReadonlyArray1<Float>,
    close: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyMinusDiState)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let len = high.len();

    if release_gil {
        let mut minus_di = vec![0.0; len];

        let state = py
            .allow_threads(|| {
                minus_di_into(
                    high_slice,
                    low_slice,
                    close_slice,
                    period,
                    minus_di.as_mut_slice(),
                )
            })
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((minus_di.into_pyarray(py).into(), state.into()))
    } else {
        let minus_di = PyArray1::<Float>::zeros(py, [len], false);
        let minus_di_slice = unsafe { minus_di.as_slice_mut()? };

        let state = minus_di_into(high_slice, low_slice, close_slice, period, minus_di_slice)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((minus_di.into(), state.into()))
    }
}

#[pyfunction(signature = (new_high, new_low, new_close, minus_di_state))]
pub(crate) fn minus_di_next(
    new_high: Float,
    new_low: Float,
    new_close: Float,
    minus_di_state: PyMinusDiState,
) -> PyResult<PyMinusDiState> {
    let mut minus_di_state: MinusDiState = minus_di_state.into();
    let sample = MinusDiSample {
        high: new_high,
        low: new_low,
        close: new_close,
    };

    minus_di_state
        .update(sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(minus_di_state.into())
}
