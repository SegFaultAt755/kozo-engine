use super::*;
use crate::engine::event::Event;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

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
                    panic!(
                        "function \'on_start\' returned an error status; the engine cannot proceed"
                    );
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
        let call_event = |ev| {
            if let Some(obj) = &self.python {
                Python::attach(|py| {
                    if let Err(err) = obj.bind(py).call_method1("on_event", ev) {
                        err.print(py);
                        panic!(
                            "function \'on_event\' returned an error status; the engine cannot proceed"
                        );
                    }
                });
            }
        };

        let call_render = || {
            if let Some(obj) = &self.python {
                Python::attach(|py| {
                    if let Err(err) = obj.bind(py).call_method0("on_render") {
                        err.print(py);
                        panic!(
                            "function \'on_render\' returned an error status; the engine cannot proceed"
                        );
                    }
                });
            }
        };

        let call_shutdown = || {
            if let Some(obj) = &self.python {
                Python::attach(|py| {
                    if let Err(err) = obj.bind(py).call_method0("on_shutdown") {
                        err.print(py);
                        panic!(
                            "function \'on_shutdown\' returned an error status; the engine will be forced to shut down"
                        );
                    }
                });
            }
        };

        match event {
            WindowEvent::CloseRequested => {
                call_shutdown();
                event_loop.exit()
            }
            WindowEvent::RedrawRequested => {
                call_render();
            }
            _ => {
                call_event((Event::Stub,));
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(obj) = &self.python {
            Python::attach(|py| {
                if let Err(err) = obj.bind(py).call_method0("on_update") {
                    err.print(py);
                    panic!(
                        "function \'on_update\' returned an error status; the engine cannot proceed"
                    );
                }
            });
        }

        self.window.as_ref().unwrap().request_redraw();
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

        Ok(())
    }
}
