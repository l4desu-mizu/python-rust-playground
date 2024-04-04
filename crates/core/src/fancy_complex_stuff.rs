use pyo3::prelude::*;

#[pyfunction]
pub fn execute_python() -> PyResult<String> {
    Python::with_gil(|py| {
        let module = PyModule::import(py, "my_python_lib.lib")?;
        let stuff = module.getattr("MyFancyStuff")?.call1(("World", "Hello"))?;
        let res = stuff.call_method0("do_stuff")?.to_string();
        stuff.call_method0("print_stuff")?;
        Ok(res)
    })
}
