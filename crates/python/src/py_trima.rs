use numpy::{IntoPyArray, PyArray1, PyArrayMethods, PyReadonlyArray1, PyUntypedArrayMethods};
use pyo3::pymethods;
use pyo3::{exceptions::PyValueError, pyclass, pyfunction, Py, PyResult, Python};
use techalysis::indicators::trima::{trima_into, TrimaState};
use techalysis::types::Float;

#[pyclass(name = "TrimaState")]
#[derive(Debug, Clone)]
pub struct PyTrimaState {
    #[pyo3(get)]
    pub trima: Float,
    #[pyo3(get)]
    pub period: usize,
    #[pyo3(get)]
    pub period_sub: Float,
    #[pyo3(get)]
    pub period_sum: Float,
    #[pyo3(get)]
    pub window: Vec<Float>,
}
#[pymethods]
impl PyTrimaState {
    #[new]
    pub fn new(
        trima: Float,
        period: usize,
        period_sub: Float,
        period_sum: Float,
        window: Vec<Float>,
    ) -> Self {
        PyTrimaState {
            trima,
            period,
            period_sub,
            period_sum,
            window,
        }
    }
    #[getter]
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
    #[getter]
    pub fn __repr__(&self) -> String {
        format!("TrimaState(trima={}, period={})", self.trima, self.period)
    }
}
impl From<TrimaState> for PyTrimaState {
    fn from(state: TrimaState) -> Self {
        PyTrimaState {
            trima: state.trima,
            period: state.period,
            period_sub: state.period_sub,
            period_sum: state.period_sum,
            window: state.window.into(),
        }
    }
}

impl From<PyTrimaState> for TrimaState {
    fn from(py_state: PyTrimaState) -> Self {
        TrimaState {
            trima: py_state.trima,
            period: py_state.period,
            period_sub: py_state.period_sub,
            period_sum: py_state.period_sum,
            window: py_state.window.into(),
        }
    }
}

#[pyfunction(signature = (data, period = 14, release_gil = false))]
pub(crate) fn trima(
    py: Python,
    data: PyReadonlyArray1<Float>,
    period: usize,
    release_gil: bool,
) -> PyResult<(Py<PyArray1<Float>>, PyTrimaState)> {
    let len = data.len();
    let slice = data.as_slice()?;

    if release_gil {
        let mut output = vec![0.0; len];
        let state = py
            .allow_threads(|| trima_into(slice, period, output.as_mut_slice()))
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((output.into_pyarray(py).into(), state.into()))
    } else {
        let py_array_out = PyArray1::<Float>::zeros(py, [slice.len()], false);
        let py_array_ptr = unsafe { py_array_out.as_slice_mut()? };

        let state = trima_into(slice, period, py_array_ptr)
            .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

        Ok((py_array_out.into(), state.into()))
    }
}

#[pyfunction(signature = (new_value, trima_state))]
pub(crate) fn trima_next(new_value: Float, trima_state: PyTrimaState) -> PyResult<PyTrimaState> {
    let trima_state: TrimaState = trima_state.into();
    let trima_state = trima_state
        .next(new_value)
        .map_err(|e| PyValueError::new_err(format!("{:?}", e)))?;

    Ok(trima_state.into())
}
