use pyo3::prelude::*;

mod artists;
mod axes;
mod axes3d;
mod colors;
mod figure;
mod projection3d;
pub mod svg_renderer;
mod text;
mod ticker;
mod transforms;
mod window;

#[pymodule]
fn _rustplotlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add_class::<transforms::Transform>()?;
    m.add_class::<figure::RustFigure>()?;
    m.add_function(wrap_pyfunction!(colors::parse_color, m)?)?;
    m.add_function(wrap_pyfunction!(ticker::auto_ticks, m)?)?;
    m.add_function(wrap_pyfunction!(ticker::format_tick, m)?)?;
    Ok(())
}
