use numpy::PyReadonlyArrayDyn;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::eq_load;

#[pyfunction]
pub fn eq_load_python(signal: PyReadonlyArrayDyn<'_, f64>, m: f64, neq: u64) -> f64 {
    eq_load(&signal.to_vec().unwrap(), m, neq)
}
#[pymodule]
fn rustfatigue(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eq_load_python, m)?)?;

    Ok(())
}