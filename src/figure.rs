use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use tiny_skia::Pixmap;

use crate::axes::Axes;
use crate::colors;

#[pyclass]
pub struct RustFigure {
    width: u32,
    height: u32,
    dpi: u32,
    axes: Vec<Axes>,
    nrows: usize,
    ncols: usize,
}

#[pymethods]
impl RustFigure {
    #[new]
    #[pyo3(signature = (width=640, height=480, dpi=100))]
    fn new(width: u32, height: u32, dpi: u32) -> Self {
        RustFigure {
            width,
            height,
            dpi,
            axes: Vec::new(),
            nrows: 1,
            ncols: 1,
        }
    }

    fn set_size_inches(&mut self, w: f64, h: f64) {
        self.width = (w * self.dpi as f64) as u32;
        self.height = (h * self.dpi as f64) as u32;
    }

    fn add_axes(&mut self) -> usize {
        let idx = self.axes.len();
        self.axes.push(Axes::new());
        idx
    }

    fn setup_subplots(&mut self, nrows: usize, ncols: usize) {
        self.nrows = nrows;
        self.ncols = ncols;
        self.axes.clear();
        for _ in 0..(nrows * ncols) {
            self.axes.push(Axes::new());
        }
    }

    fn axes_plot(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let linestyle = if let Some(v) = kwargs.get_item("linestyle")? {
            Some(v.extract::<String>()?)
        } else { None };

        let marker = if let Some(v) = kwargs.get_item("marker")? {
            Some(v.extract::<String>()?)
        } else { None };

        let marker_size = if let Some(v) = kwargs.get_item("markersize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.plot(
            x, y,
            color,
            linewidth,
            linestyle.as_deref(),
            marker.as_deref(),
            marker_size,
            label,
            alpha,
        );

        Ok(())
    }

    fn axes_scatter(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let sizes = if let Some(v) = kwargs.get_item("s")? {
            Some(v.extract::<Vec<f32>>()?)
        } else { None };

        let marker = if let Some(v) = kwargs.get_item("marker")? {
            Some(v.extract::<String>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.scatter(
            x, y,
            color,
            sizes,
            marker.as_deref(),
            label,
            alpha,
        );

        Ok(())
    }

    fn axes_bar(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        heights: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let width = if let Some(v) = kwargs.get_item("width")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.bar(x, heights, color, width, label, alpha);

        Ok(())
    }

    fn axes_hist(
        &mut self,
        ax_id: usize,
        data: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let bins = if let Some(v) = kwargs.get_item("bins")? {
            v.extract::<usize>()?
        } else { 10 };

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else { None };

        ax.hist(&data, bins, color, alpha, label);

        Ok(())
    }

    fn axes_imshow(
        &mut self,
        ax_id: usize,
        data: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            Some(v.extract::<String>()?)
        } else { None };

        ax.imshow(data, cmap);

        Ok(())
    }

    fn axes_set_title(&mut self, ax_id: usize, title: String) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.title = Some(title);
        Ok(())
    }

    fn axes_set_xlabel(&mut self, ax_id: usize, label: String) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.xlabel = Some(label);
        Ok(())
    }

    fn axes_set_ylabel(&mut self, ax_id: usize, label: String) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.ylabel = Some(label);
        Ok(())
    }

    fn axes_set_xlim(&mut self, ax_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.xlim = Some((lo, hi));
        Ok(())
    }

    fn axes_set_ylim(&mut self, ax_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.ylim = Some((lo, hi));
        Ok(())
    }

    fn axes_legend(&mut self, ax_id: usize) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.show_legend = true;
        Ok(())
    }

    fn axes_grid(&mut self, ax_id: usize, visible: bool) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.grid_visible = visible;
        Ok(())
    }

    fn num_axes(&self) -> usize {
        self.axes.len()
    }

    fn render_to_png_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let pixmap = self.render_pixmap();
        let png_data = pixmap.encode_png()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("PNG encode error: {}", e)))?;
        Ok(PyBytes::new_bound(py, &png_data))
    }

    fn savefig(&self, path: String) -> PyResult<()> {
        let pixmap = self.render_pixmap();
        let png_bytes = pixmap.encode_png()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("PNG encode error: {}", e)))?;

        if path.ends_with(".svg") {
            let b64 = simple_base64_encode(&png_bytes);
            let svg = format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"
     width="{}" height="{}" viewBox="0 0 {} {}">
  <image width="{}" height="{}" href="data:image/png;base64,{}"/>
</svg>"#,
                self.width, self.height,
                self.width, self.height,
                self.width, self.height,
                b64,
            );
            std::fs::write(&path, svg)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write SVG: {}", e)))?;
        } else {
            std::fs::write(&path, &png_bytes)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write PNG: {}", e)))?;
        }
        Ok(())
    }
}

impl RustFigure {
    fn render_pixmap(&self) -> Pixmap {
        let mut pixmap = Pixmap::new(self.width, self.height)
            .expect("Failed to create pixmap");

        // Fill with light gray background (figure background)
        let bg_paint = {
            let mut p = tiny_skia::Paint::default();
            p.set_color(tiny_skia::Color::from_rgba8(240, 240, 240, 255));
            p
        };
        if let Some(rect) = tiny_skia::Rect::from_xywh(0.0, 0.0, self.width as f32, self.height as f32) {
            pixmap.fill_rect(rect, &bg_paint, tiny_skia::Transform::identity(), None);
        }

        if self.axes.is_empty() {
            return pixmap;
        }

        // Layout margins
        let margin_left = 70.0_f32;
        let margin_right = 20.0_f32;
        let margin_top = 40.0_f32;
        let margin_bottom = 50.0_f32;
        let subplot_hgap = 60.0_f32;
        let subplot_vgap = 60.0_f32;

        let nrows = self.nrows.max(1);
        let ncols = self.ncols.max(1);

        let total_w = self.width as f32 - margin_left - margin_right;
        let total_h = self.height as f32 - margin_top - margin_bottom;

        let cell_w = (total_w - (ncols as f32 - 1.0) * subplot_hgap) / ncols as f32;
        let cell_h = (total_h - (nrows as f32 - 1.0) * subplot_vgap) / nrows as f32;

        for (idx, ax) in self.axes.iter().enumerate() {
            let row = idx / ncols;
            let col = idx % ncols;
            if row >= nrows { break; }

            let left = margin_left + col as f32 * (cell_w + subplot_hgap);
            let top = margin_top + row as f32 * (cell_h + subplot_vgap);
            let right = left + cell_w;
            let bottom = top + cell_h;

            ax.draw(&mut pixmap, left, top, right, bottom);
        }

        pixmap
    }
}

/// Simple base64 encoder (no external dependency).
fn simple_base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    let chunks = data.chunks(3);

    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}
