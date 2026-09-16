use super::*;
use crate::engine::event::Event;
use pyo3::exceptions::PyNotImplementedError;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn function_not_implemented(fn_name: &str) -> PyErr {
    PyNotImplementedError::new_err(format!("function {}() must be implemented", fn_name))
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(Window::default_attributes().with_title(self.title.clone()))
                .unwrap();

            self.window = Some(window);
        }

        if let Some(obj) = &self.python {
            Python::attach(|py| {
                if let Err(err) = obj.bind(py).call_method0("on_start") {
                    err.print(py);
                }
            });
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn run(slf: &Bound<'_, Self>) -> PyResult<()> {
        let python = slf.clone().unbind();
        let event_loop = EventLoop::new().unwrap();

        let mut app = App {
            python: Some(python.into()),
            ..Default::default()
        };

        event_loop.run_app(&mut app).unwrap();

        // // Call core functions
        // slf.getattr("on_start")
        //     .map_err(|_| function_not_implemented("on_start"))?
        //     .call0()?;
        // slf.getattr("on_event")
        //     .map_err(|_| function_not_implemented("on_event"))?
        //     .call1((Event::Shutdown,))?; // Use hardcoded value for a while
        // slf.getattr("on_update")
        //     .map_err(|_| function_not_implemented("on_update"))?
        //     .call0()?;
        // slf.getattr("on_render")
        //     .map_err(|_| function_not_implemented("on_render"))?
        //     .call0()?;
        // slf.getattr("on_shutdown")
        //     .map_err(|_| function_not_implemented("on_shutdown"))?
        //     .call0()?;

        Ok(())
    }
}
