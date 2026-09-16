use super::*;

#[pymethods]
impl App {
    #[getter]
    fn title(&self) -> String {
        self.title.clone()
    }

    #[getter]
    fn size(&self) -> (usize, usize) {
        self.size
    }

    #[getter]
    fn position(&self) -> (isize, isize) {
        self.position
    }
}
