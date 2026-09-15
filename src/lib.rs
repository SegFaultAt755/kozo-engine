use pyo3::prelude::*;

mod engine;

#[pyfunction]
fn test_magic() -> PyResult<u32> {
    Ok(0xC0FFEE)
}

#[pymodule]
fn kozo(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_magic, m)?)?;

    // ----- Core -----
    let modl = PyModule::new(py, "core")?;

    // Attach functions, classes and so on
    modl.add_class::<engine::app::App>()?;
    modl.add_class::<engine::event::Event>()?;

    m.add_submodule(&modl)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kozo.core", &modl)?;

    Ok(())
}
