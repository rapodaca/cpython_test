use cpython::{py_module_initializer, py_fn, PyResult, Python};

fn greet(_: Python, name: String) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

py_module_initializer! {
    function, |py, module| {
        module.add(py, "greet", py_fn!(py, greet(string: String)))?;

        Ok(())
    }
}