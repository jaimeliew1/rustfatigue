use numpy::PyArrayMethods;
use numpy::PyReadonlyArrayDyn;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::rainflow::{get_halfcycles, get_peaktrough, ExtremaError};
use crate::{eq_load, eq_load_max_half_cycle_closed};

impl From<ExtremaError> for PyErr {
    fn from(err: ExtremaError) -> PyErr {
        PyValueError::new_err(err.to_string())
    }
}

#[pyfunction]
pub fn eq_load_python(signal: PyReadonlyArrayDyn<'_, f64>, m: f64, neq: u64, half: bool) -> PyResult<f64> {
    Ok(eq_load(&signal.to_vec().unwrap(), m, neq, half)?)
}

#[pyfunction]
pub fn eq_load_max_half_cycle_closed_python(
    signal: PyReadonlyArrayDyn<'_, f64>,
    m: f64,
    neq: u64,
    half: bool,
) -> PyResult<f64> {
    Ok(eq_load_max_half_cycle_closed(&signal.to_vec().unwrap(), m, neq, half)?)
}

#[pyfunction]
pub fn halfcycles(signal: PyReadonlyArrayDyn<'_, f64>, half: bool) -> PyResult<Vec<(f64, f64)>> {
    let peaktrough = get_peaktrough(&signal.to_vec().unwrap())?;
    Ok(get_halfcycles(&peaktrough, half))
}

#[pymodule]
fn rustfatigue(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eq_load_python, m)?)?;
    m.add_function(wrap_pyfunction!(eq_load_max_half_cycle_closed_python, m)?)?;
    m.add_function(wrap_pyfunction!(halfcycles, m)?)?;

    Ok(())
}
