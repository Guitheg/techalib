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
use techalib::indicators::ad::{ad_into, AdSample, AdState};
use techalib::traits::State;
use techalib::types::Float;

#[pyclass(name = "AdState")]
#[derive(Debug, Clone)]
pub struct PyAdState {
    #[pyo3(get)]
    pub ad: Float,
}
#[pymethods]
impl PyAdState {
    #[new]
    pub fn new(ad: Float) -> Self {
        PyAdState { ad }
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
impl From<AdState> for PyAdState {
    fn from(state: AdState) -> Self {
        PyAdState { ad: state.ad }
    }
}

impl From<PyAdState> for AdState {
    fn from(py_state: PyAdState) -> Self {
        AdState { ad: py_state.ad }
    }
}

#[pyfunction(signature = (high, low, close, volume, release_gil = false))]
pub(crate) fn ad(
    py: Python,
    high: PyReadonlyArray1<Float>,
    low: PyReadonlyArray1<Float>,
    close: PyReadonlyArray1<Float>,
    volume: PyReadonlyArray1<Float>,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyAdState)> {
    let len = high.len();
    let high_slice = high.as_slice()?;
    let low_slice = low.as_slice()?;
    let close_slice = close.as_slice()?;
    let volume_slice = volume.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];

        let state = py
            .allow_threads(|| {
                ad_into(
                    high_slice,
                    low_slice,
                    close_slice,
                    volume_slice,
                    output.as_mut_slice(),
                )
            })
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [len], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = ad_into(
            high_slice,
            low_slice,
            close_slice,
            volume_slice,
            py_array_ptr,
        )
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (high, low, close, volume, ad_state))]
pub(crate) fn ad_next(
    high: Float,
    low: Float,
    close: Float,
    volume: Float,
    ad_state: PyAdState,
) -> PyResult<PyAdState> {
    let mut ad_state: AdState = ad_state.into();
    let sample = AdSample {
        high,
        low,
        close,
        volume,
    };
    ad_state
        .update(&sample)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(ad_state.into())
}
