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

/// Load a custom .ttf/.otf font file for text rendering.
#[pyfunction]
fn set_font(path: String) -> PyResult<()> {
    let data = std::fs::read(&path).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to read font file '{}': {}", path, e))
    })?;
    // Validate it is a valid font
    let _ = ab_glyph::FontRef::try_from_slice(&data).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Invalid font file '{}': {}", path, e))
    })?;
    text::set_custom_font_data(data);
    Ok(())
}

/// Clear any custom font, reverting to the embedded DejaVu Sans.
#[pyfunction]
fn clear_font() -> PyResult<()> {
    text::clear_custom_font();
    Ok(())
}

#[pymodule]
fn _rustplotlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.0")?;
    m.add_class::<transforms::Transform>()?;
    m.add_class::<figure::RustFigure>()?;
    m.add_function(wrap_pyfunction!(colors::parse_color, m)?)?;
    m.add_function(wrap_pyfunction!(ticker::auto_ticks, m)?)?;
    m.add_function(wrap_pyfunction!(ticker::format_tick, m)?)?;
    m.add_function(wrap_pyfunction!(set_font, m)?)?;
    m.add_function(wrap_pyfunction!(clear_font, m)?)?;
    Ok(())
}
