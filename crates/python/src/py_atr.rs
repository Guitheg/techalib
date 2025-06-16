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
use techalib::indicators::atr::{atr_into, AtrSample, AtrState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "AtrState")]
#[derive(Debug, Clone)]
pub struct PyAtrState {
    #[pyo3(get)]
    pub atr: Float,
    #[pyo3(get)]
    pub prev_close: Float,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyAtrState {
    #[new]
    pub fn new(atr: Float, prev_close: Float, period: usize) -> Self {
        PyAtrState {
            atr,
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
impl From<AtrState> for PyAtrState {
    fn from(state: AtrState) -> Self {
        PyAtrState {
            atr: state.atr,
            prev_close: state.prev_close,
            period: state.period,
        }
    }
}

impl From<PyAtrState> for AtrState {
    fn from(py_state: PyAtrState) -> Self {
        AtrState {
            atr: py_state.atr,
            prev_close: py_state.prev_close,
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (high, low, close, period = 14, release_gil = false))]
pub(crate) fn atr(
    py: Python,
    high: PyReadonlyArray1<Float>,
    low: PyReadonlyArray1<Float>,
    close: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyAtrState)> {
    let len = close.len();
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| {
                atr_into(
                    high_slice,
                    low_slice,
                    close_slice,
                    period,
                    output.as_mut_slice(),
                )
            })
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = atr_into(high_slice, low_slice, close_slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (high, low, close, atr_state))]
pub(crate) fn atr_next(
    high: Float,
    low: Float,
    close: Float,
    atr_state: PyAtrState,
) -> PyResult<PyAtrState> {
    let mut atr_state: AtrState = atr_state.into();
    let sample = AtrSample { high, low, close };
    atr_state
        .update(&sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(atr_state.into())
}
