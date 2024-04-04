mod fancy_complex_stuff;

pub use fancy_complex_stuff::execute_python;
use pyo3::prelude::*;

#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute_python, m)?)?;
    Ok(())
}
