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
use techalib::indicators::aroon::{aroon_into, AroonSample, AroonState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "AroonState")]
#[derive(Debug, Clone)]
pub struct PyAroonState {
    #[pyo3(get)]
    pub prev_aroon_down: Float,
    #[pyo3(get)]
    pub prev_aroon_up: Float,
    #[pyo3(get)]
    pub prev_high_window: Vec<Float>,
    #[pyo3(get)]
    pub prev_low_window: Vec<Float>,
    #[pyo3(get)]
    pub period: usize,
}
#[pymethods]
impl PyAroonState {
    #[new]
    pub fn new(
        prev_aroon_down: Float,
        prev_aroon_up: Float,
        prev_high_window: Vec<Float>,
        prev_low_window: Vec<Float>,
        period: usize,
    ) -> Self {
        PyAroonState {
            prev_aroon_down,
            prev_aroon_up,
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
impl From<AroonState> for PyAroonState {
    fn from(state: AroonState) -> Self {
        PyAroonState {
            prev_aroon_down: state.prev_aroon_down,
            prev_aroon_up: state.prev_aroon_up,
            prev_high_window: state.prev_high_window.into(),
            prev_low_window: state.prev_low_window.into(),
            period: state.period,
        }
    }
}

impl From<PyAroonState> for AroonState {
    fn from(py_state: PyAroonState) -> Self {
        AroonState {
            prev_aroon_down: py_state.prev_aroon_down,
            prev_aroon_up: py_state.prev_aroon_up,
            prev_high_window: py_state.prev_high_window.into(),
            prev_low_window: py_state.prev_low_window.into(),
            period: py_state.period,
        }
    }
}

#[pyfunction(signature = (high, low, period = 14, release_gil = false))]
pub(crate) fn aroon(
    py: Python,
    high: PyReadonlyArray1<Float>,
    low: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, Py<PyArray1<Float>>, PyAroonState)> {
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let len = high.len();

    if release_gil {
        let mut aroon_down = vec![0.0; len];
        let mut aroon_up = vec![0.0; len];

        let state = py
            .allow_threads(|| {
                aroon_into(
                    high_slice,
                    low_slice,
                    period,
                    aroon_down.as_mut_slice(),
                    aroon_up.as_mut_slice(),
                )
            })
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((
            aroon_down.into_pyarray(py).into(),
            aroon_up.into_pyarray(py).into(),
            state.into(),
        ))
    } else {
        let aroon_down = PyArray1::<Float>::zeros(py, [len], false);
        let aroon_down_slice = unsafe { aroon_down.as_slice_mut()? };
        let aroon_up = PyArray1::<Float>::zeros(py, [len], false);
        let aroon_up_slice = unsafe { aroon_up.as_slice_mut()? };

        let state = aroon_into(
            high_slice,
            low_slice,
            period,
            aroon_down_slice,
            aroon_up_slice,
        )
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((aroon_down.into(), aroon_up.into(), state.into()))
    }
}

#[pyfunction(signature = (new_high, new_low, aroon_state))]
pub(crate) fn aroon_next(
    new_high: Float,
    new_low: Float,
    aroon_state: PyAroonState,
) -> PyResult<PyAroonState> {
    let mut aroon_state: AroonState = aroon_state.into();
    let sample = AroonSample {
        high: new_high,
        low: new_low,
    };

    aroon_state
        .update(sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(aroon_state.into())
}
