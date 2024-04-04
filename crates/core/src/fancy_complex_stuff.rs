use pyo3::prelude::*;

#[pyfunction]
pub fn execute_python() -> PyResult<String> {
    Python::with_gil(|py| {
        let module = PyModule::import(py, "my_python_lib.lib")?;
        let res = module
            .getattr("MyFancyStuff")?
            .call1(("Hello",))?
            .call_method0("do_stuff")?
            .to_string();
        Ok(res)
    })
}
