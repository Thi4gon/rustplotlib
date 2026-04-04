use pyo3::prelude::*;

mod artists;
mod axes;
mod axes3d;
mod colors;
mod figure;
mod mathtext;
mod parse;
mod projection3d;
mod projections;
pub mod svg_renderer;
mod text;
mod ticker;
mod transforms;
mod window;

/// Parse LaTeX math symbols to Unicode.
#[pyfunction]
fn parse_math_symbols_py(text: &str) -> String {
    mathtext::parse_math_symbols(text)
}

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
    m.add_class::<transforms::Affine2D>()?;
    m.add_class::<figure::RustFigure>()?;
    m.add_function(wrap_pyfunction!(colors::parse_color, m)?)?;
    m.add_function(wrap_pyfunction!(ticker::auto_ticks, m)?)?;
    m.add_function(wrap_pyfunction!(ticker::format_tick, m)?)?;
    m.add_function(wrap_pyfunction!(set_font, m)?)?;
    m.add_function(wrap_pyfunction!(clear_font, m)?)?;
    // Parse utilities (format strings, tick formatting, hit testing)
    m.add_function(wrap_pyfunction!(parse::parse_fmt, m)?)?;
    m.add_function(wrap_pyfunction!(parse::color_char_to_name, m)?)?;
    m.add_function(wrap_pyfunction!(parse::format_tick_scalar, m)?)?;
    m.add_function(wrap_pyfunction!(parse::format_tick_percent, m)?)?;
    m.add_function(wrap_pyfunction!(parse::format_tick_engineering, m)?)?;
    m.add_function(wrap_pyfunction!(parse::format_tick_log, m)?)?;
    m.add_function(wrap_pyfunction!(parse::tick_values_multiple, m)?)?;
    m.add_function(wrap_pyfunction!(parse::tick_values_log, m)?)?;
    m.add_function(wrap_pyfunction!(parse::tick_values_linear, m)?)?;
    m.add_function(wrap_pyfunction!(parse::hit_test_points, m)?)?;
    m.add_function(wrap_pyfunction!(parse::hit_test_line, m)?)?;
    m.add_function(wrap_pyfunction!(parse::parse_plot_groups, m)?)?;
    m.add_function(wrap_pyfunction!(parse::figure_to_json, m)?)?;
    // Geographic projections
    m.add_function(wrap_pyfunction!(projections::hammer_project, m)?)?;
    m.add_function(wrap_pyfunction!(projections::aitoff_project, m)?)?;
    m.add_function(wrap_pyfunction!(projections::mollweide_project, m)?)?;
    m.add_function(wrap_pyfunction!(projections::hammer_project_batch, m)?)?;
    m.add_function(wrap_pyfunction!(projections::aitoff_project_batch, m)?)?;
    m.add_function(wrap_pyfunction!(projections::mollweide_project_batch, m)?)?;
    m.add_function(wrap_pyfunction!(projections::generate_graticule, m)?)?;
    m.add_function(wrap_pyfunction!(parse_math_symbols_py, m)?)?;
    Ok(())
}
