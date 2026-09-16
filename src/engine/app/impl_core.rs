use super::*;
use crate::engine::event::Event;
use pyo3::exceptions::PyNotImplementedError;

fn function_not_implemented(fn_name: &str) -> PyErr {
    PyNotImplementedError::new_err(format!("function {}() must be implemented", fn_name))
}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn run(slf: &Bound<'_, Self>) -> PyResult<()> {
        // Call core functions
        slf.getattr("on_start")
            .map_err(|_| function_not_implemented("on_start"))?
            .call0()?;
        slf.getattr("on_event")
            .map_err(|_| function_not_implemented("on_event"))?
            .call1((Event::Shutdown,))?; // Use hardcoded value for a while
        slf.getattr("on_update")
            .map_err(|_| function_not_implemented("on_update"))?
            .call0()?;
        slf.getattr("on_render")
            .map_err(|_| function_not_implemented("on_render"))?
            .call0()?;
        slf.getattr("on_shutdown")
            .map_err(|_| function_not_implemented("on_shutdown"))?
            .call1((Event::Shutdown,))?;

        Ok(())
    }
}
