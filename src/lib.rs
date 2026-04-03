use pyo3::prelude::*;

mod colors;

#[pymodule]
fn _rustplot(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add_function(wrap_pyfunction!(colors::parse_color, m)?)?;
    Ok(())
}
