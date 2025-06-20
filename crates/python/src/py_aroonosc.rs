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
use techalib::indicators::aroonosc::{aroonosc_into, AroonoscSample, AroonoscState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "AroonoscState")]
#[derive(Debug, Clone)]
pub struct PyAroonoscState {
    #[pyo3(get)]
    pub prev_aroonosc: Float,
    #[pyo3(get)]
    pub prev_high_window: Vec<Float>,
    #[pyo3(get)]
    pub prev_low_window: Vec<Float>,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyAroonoscState {
    #[new]
    pub fn new(
        prev_aroonosc: Float,
        prev_high_window: Vec<Float>,
        prev_low_window: Vec<Float>,
        period: usize,
    ) -> Self {
        PyAroonoscState {
            prev_aroonosc,
            prev_high_window,
            prev_low_window,
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
impl From<AroonoscState> for PyAroonoscState {
    fn from(state: AroonoscState) -> Self {
        PyAroonoscState {
            prev_aroonosc: state.prev_aroonosc,
            prev_high_window: state.prev_high_window.into(),
            prev_low_window: state.prev_low_window.into(),
            period: state.period,
        }
    }
}

impl From<PyAroonoscState> for AroonoscState {
    fn from(py_state: PyAroonoscState) -> Self {
        AroonoscState {
            prev_aroonosc: py_state.prev_aroonosc,
            prev_high_window: py_state.prev_high_window.into(),
            prev_low_window: py_state.prev_low_window.into(),
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (high, low, period = 14, release_gil = false))]
pub(crate) fn aroonosc(
    py: Python,
    high: PyReadonlyArray1<Float>,
    low: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyAroonoscState)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let len = high.len();

    if release_gil {
        let mut aroonosc = vec![0.0; len];

        let state = py
            .allow_threads(|| aroonosc_into(high_slice, low_slice, period, aroonosc.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((aroonosc.into_pyarray(py).into(), state.into()))
    } else {
        let aroonosc = PyArray1::<Float>::zeros(py, [len], false);
        let aroonosc_slice = unsafe { aroonosc.as_slice_mut()? };

        let state = aroonosc_into(high_slice, low_slice, period, aroonosc_slice)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((aroonosc.into(), state.into()))
    }
}

#[pyfunction(signature = (new_high, new_low, aroonosc_state))]
pub(crate) fn aroonosc_next(
    new_high: Float,
    new_low: Float,
    aroonosc_state: PyAroonoscState,
) -> PyResult<PyAroonoscState> {
    let mut aroonosc_state: AroonoscState = aroonosc_state.into();
    let sample = AroonoscSample {
        high: new_high,
        low: new_low,
    };

    aroonosc_state
        .update(sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(aroonosc_state.into())
}
