use std::cell::RefCell;

use cpython::{py_class, py_module_initializer, PyClone, PyObject, PyResult};

type Inner = std::collections::HashSet<u32>;

py_class! {pub class SetIterator |py| {
    data iter: RefCell<std::collections::hash_set::IntoIter<u32>>;

    def __iter__(&self) -> PyResult<Self> {
        Ok(self.clone_ref(py))
    }

    def __next__(&self) -> PyResult<Option<u32>> {
        Ok(self.iter(py).borrow_mut().next())
    }
}}

py_class! {pub class HashSet |py| {
    data hash_set: RefCell<Inner>;

    def __new__(_cls) -> PyResult<HashSet> {
        HashSet::create_instance(py, RefCell::new(Inner::new()))
    }

    def __len__(&self) -> PyResult<usize> {
        Ok(self.hash_set(py).borrow().len())
    }

    def __contains__(&self, v: u32) -> PyResult<bool> {
        Ok(self.hash_set(py).borrow().contains(&v))
    }

    def __iter__(&self) -> PyResult<SetIterator> {
        SetIterator::create_instance(
            py,
            RefCell::new(self.hash_set(py).borrow().clone().into_iter())
        )
    }

    def add(&self, v: u32) -> PyResult<PyObject> {
        self.hash_set(py).borrow_mut().insert(v);

        Ok(py.None())
    }
}}

py_module_initializer! {
    wrapper, |py, module| {
        module.add_class::<HashSet>(py)?;

        Ok(())
    }
}
