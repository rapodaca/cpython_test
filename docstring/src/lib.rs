use cpython::py_module_initializer;

py_module_initializer! {
    docstring, |py, module| {
        module.add(py, "__doc__", "This module is implemented in Rust.")?;

        Ok(())
    }
}
