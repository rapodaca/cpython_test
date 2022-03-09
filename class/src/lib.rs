use cpython::{py_class, PyResult, py_module_initializer};

py_class!{pub class Class |py| {
    def __new__(_cls) -> PyResult<Class> {
        Class::create_instance(py)
    }

    def greet(&self, name: String) -> PyResult<String> {
        Ok(format!("Hello, {}!", name))
    }
}}

py_module_initializer! {
    klass, |py, module| {
        module.add_class::<Class>(py)?;

        Ok(())
    }
}