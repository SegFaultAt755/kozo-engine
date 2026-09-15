use super::*;
use crate::engine::event::Event;

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn run(slf: &Bound<'_, Self>) -> PyResult<()> {
        // Call core functions
        slf.getattr("on_start")?.call0()?.extract::<bool>()?;
        slf.getattr("on_event")?.call1((Event::Shutdown,))?; // Use hardcoded value for a while
        slf.getattr("on_update")?.call0()?;
        slf.getattr("on_render")?.call0()?;
        slf.getattr("on_shutdown")?.call1((Event::Shutdown,))?;

        Ok(())
    }
}
