use numpy::PyArrayMethods;
use numpy::PyReadonlyArrayDyn;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::{eq_load, get_halfcycles, get_peaktrough};

#[pyfunction]
pub fn eq_load_python(signal: PyReadonlyArrayDyn<'_, f64>, m: f64, neq: u64, half: bool) -> f64 {
    eq_load(&signal.to_vec().unwrap(), m, neq, half)
}

#[pyfunction]
pub fn halfcycles(signal: PyReadonlyArrayDyn<'_, f64>, half: bool) -> Vec<f64> {
    let peaktrough = get_peaktrough(&signal.to_vec().unwrap());
    get_halfcycles(&peaktrough, half)
}

#[pymodule]
fn rustfatigue(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eq_load_python, m)?)?;
    m.add_function(wrap_pyfunction!(halfcycles, m)?)?;

    Ok(())
}
