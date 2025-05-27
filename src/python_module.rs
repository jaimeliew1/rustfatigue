use numpy::PyReadonlyArrayDyn;
use numpy::PyArrayMethods;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::eq_load;

#[pyfunction]
pub fn eq_load_python(signal: PyReadonlyArrayDyn<'_, f64>, m: f64, neq: u64, half: bool) -> f64 {
    eq_load(&signal.to_vec().unwrap(), m, neq, half)
}

#[pymodule]
fn rustfatigue(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eq_load_python, m)?)?;

    Ok(())
}