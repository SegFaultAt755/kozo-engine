use pyo3::prelude::*;

mod impl_core;
mod impl_getters;

#[pyclass(subclass)]
#[derive(Debug)]
pub struct App {
    title: String,
    size: (usize, usize),
    position: (isize, isize),
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "Window Title | Kozo Engine".to_string(),
            size: (1280, 720),
            position: (10, 10),
        }
    }
}
