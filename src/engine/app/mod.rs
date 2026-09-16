use pyo3::prelude::*;
use winit::window::Window;

mod core;
mod getters;

#[pyclass(subclass)]
#[derive(Debug)]
pub struct App {
    title: String,
    size: (usize, usize),
    position: (isize, isize),
    window: Option<Window>,
    python: Option<Py<PyAny>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "Window Title | Kozo Engine".to_string(),
            size: (1280, 720),
            position: (10, 10),
            window: None,
            python: None,
        }
    }
}
