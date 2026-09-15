use pyo3::prelude::*;

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Shutdown,
    MemoryOut, // Keyboard, mouse, window and etc.
}
