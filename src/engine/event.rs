use pyo3::prelude::*;

#[pyclass(from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Stub,
    Shutdown,
    MemoryOut, // Keyboard, mouse, window and etc.
}
