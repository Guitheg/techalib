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
use techalib::indicators::plus_dm::{plus_dm_into, PlusDmSample, PlusDmState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "PlusDmState")]
#[derive(Debug, Clone)]
pub struct PyPlusDmState {
    #[pyo3(get)]
    pub prev_plus_dm: Float,
    #[pyo3(get)]
    pub prev_high: Float,
    #[pyo3(get)]
    pub prev_low: Float,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyPlusDmState {
    #[new]
    pub fn new(prev_plus_dm: Float, prev_high: Float, prev_low: Float, period: usize) -> Self {
        PyPlusDmState {
            prev_plus_dm,
            prev_high,
            prev_low,
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
impl From<PlusDmState> for PyPlusDmState {
    fn from(state: PlusDmState) -> Self {
        PyPlusDmState {
            prev_plus_dm: state.prev_plus_dm,
            prev_high: state.prev_high,
            prev_low: state.prev_low,
            period: state.period,
        }
    }
}

impl From<PyPlusDmState> for PlusDmState {
    fn from(py_state: PyPlusDmState) -> Self {
        PlusDmState {
            prev_plus_dm: py_state.prev_plus_dm,
            prev_high: py_state.prev_high,
            prev_low: py_state.prev_low,
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (high, low, period = 14, release_gil = false))]
pub(crate) fn plus_dm(
    py: Python,
    high: PyReadonlyArray1<Float>,
    low: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyPlusDmState)> {
    let len = high.len();
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| plus_dm_into(high_slice, low_slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = plus_dm_into(high_slice, low_slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (high, low, plus_dm_state))]
pub(crate) fn plus_dm_next(
    high: Float,
    low: Float,
    plus_dm_state: PyPlusDmState,
) -> PyResult<PyPlusDmState> {
    let mut plus_dm_state: PlusDmState = plus_dm_state.into();
    let sample = PlusDmSample { high, low };
    plus_dm_state
        .update(&sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(plus_dm_state.into())
}
