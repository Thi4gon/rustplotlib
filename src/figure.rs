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

        let markevery = kwargs.get_item("markevery")?
            .map(|v| v.extract::<usize>().unwrap_or(1));

        // Accept and ignore markerfacecolor (for hollow markers — not yet implemented)
        let _ = kwargs.get_item("markerfacecolor")?;

        ax.plot(
            x, y,
            color,
            linewidth,
            linestyle.as_deref(),
            marker.as_deref(),
            marker_size,
            markevery,
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

    #[pyo3(signature = (ax_id, title, fontsize=None))]
    fn axes_set_title(&mut self, ax_id: usize, title: String, fontsize: Option<f32>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.title = Some(title);
        if let Some(fs) = fontsize {
            ax.title_size = fs;
        }
        Ok(())
    }

    #[pyo3(signature = (ax_id, label, fontsize=None))]
    fn axes_set_xlabel(&mut self, ax_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.xlabel = Some(label);
        if let Some(fs) = fontsize {
            ax.label_size = fs;
        }
        Ok(())
    }

    #[pyo3(signature = (ax_id, label, fontsize=None))]
    fn axes_set_ylabel(&mut self, ax_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.ylabel = Some(label);
        if let Some(fs) = fontsize {
            ax.label_size = fs;
        }
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

    fn axes_legend(&mut self, ax_id: usize, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.show_legend = true;

        if let Some(v) = kwargs.get_item("loc")? {
            ax.legend_loc = v.extract::<String>()?;
        }
        // Accept and ignore 'prop' (font properties)
        let _ = kwargs.get_item("prop")?;

        Ok(())
    }

    fn axes_grid(&mut self, ax_id: usize, visible: bool, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.grid_visible = visible;

        if let Some(c) = kwargs.get_item("color")? {
            ax.grid_color = colors::parse_color_value(&c)?;
        }
        if let Some(v) = kwargs.get_item("linewidth")? {
            ax.grid_linewidth = v.extract::<f32>()?;
        }
        if let Some(v) = kwargs.get_item("alpha")? {
            ax.grid_alpha = v.extract::<f32>()?;
        }
        // Accept linestyle but don't process it yet (grid linestyle is basic)
        let _ = kwargs.get_item("linestyle")?;

        Ok(())
    }

    fn axes_fill_between(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        y1: Vec<f64>,
        y2: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else { None };

        ax.fill_between(x, y1, y2, color, alpha, label);

        Ok(())
    }

    fn axes_step(
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

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let where_style = if let Some(v) = kwargs.get_item("where")? {
            Some(v.extract::<String>()?)
        } else { None };

        ax.step(
            x, y,
            color,
            linewidth,
            linestyle.as_deref(),
            label,
            alpha,
            where_style.as_deref(),
        );

        Ok(())
    }

    fn axes_pie(
        &mut self,
        ax_id: usize,
        sizes: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let labels = if let Some(v) = kwargs.get_item("labels")? {
            v.extract::<Vec<String>>()?
        } else {
            Vec::new()
        };

        let colors = if let Some(v) = kwargs.get_item("colors")? {
            let color_list: Vec<Bound<'_, pyo3::PyAny>> = v.extract()?;
            let mut result = Vec::new();
            for c in &color_list {
                result.push(colors::parse_color_value(c)?);
            }
            result
        } else {
            Vec::new()
        };

        let start_angle = if let Some(v) = kwargs.get_item("startangle")? {
            v.extract::<f32>()?
        } else {
            90.0
        };

        ax.pie(sizes, labels, colors, start_angle);

        Ok(())
    }

    fn axes_axhline(
        &mut self,
        ax_id: usize,
        y: f64,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let linestyle = if let Some(v) = kwargs.get_item("linestyle")? {
            v.extract::<String>()?
        } else {
            "--".to_string()
        };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        ax.axhline(y, color, &linestyle, linewidth, alpha);

        Ok(())
    }

    fn axes_axvline(
        &mut self,
        ax_id: usize,
        x: f64,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let linestyle = if let Some(v) = kwargs.get_item("linestyle")? {
            v.extract::<String>()?
        } else {
            "--".to_string()
        };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        ax.axvline(x, color, &linestyle, linewidth, alpha);

        Ok(())
    }

    fn axes_text(
        &mut self,
        ax_id: usize,
        x: f64,
        y: f64,
        text: String,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let fontsize = if let Some(v) = kwargs.get_item("fontsize")? {
            v.extract::<f32>()?
        } else {
            12.0
        };

        let color = if let Some(c) = kwargs.get_item("color")? {
            colors::parse_color_value(&c)?
        } else {
            crate::colors::Color::new(0, 0, 0, 255)
        };

        ax.texts.push(crate::axes::TextAnnotation {
            x,
            y,
            text,
            fontsize,
            color,
        });

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

        if path.ends_with(".pdf") {
            let pdf_data = Self::render_pdf(&pixmap);
            std::fs::write(&path, pdf_data)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write PDF: {}", e)))?;
        } else if path.ends_with(".svg") {
            let png_bytes = pixmap.encode_png()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("PNG encode error: {}", e)))?;
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
            let png_bytes = pixmap.encode_png()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("PNG encode error: {}", e)))?;
            std::fs::write(&path, &png_bytes)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write PNG: {}", e)))?;
        }
        Ok(())
    }

    fn show(&self) -> PyResult<()> {
        let pixmap = self.render_pixmap();
        crate::window::show_pixmap(&pixmap)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
    }
}

impl RustFigure {
    fn render_pdf(pixmap: &Pixmap) -> Vec<u8> {
        let w = pixmap.width();
        let h = pixmap.height();

        // Extract RGB data from pixmap (unpremultiply alpha, drop alpha channel)
        let mut rgb_data = Vec::with_capacity((w * h * 3) as usize);
        for px in pixmap.pixels() {
            let a = px.alpha() as u32;
            if a > 0 {
                rgb_data.push(((px.red() as u32 * 255 / a).min(255)) as u8);
                rgb_data.push(((px.green() as u32 * 255 / a).min(255)) as u8);
                rgb_data.push(((px.blue() as u32 * 255 / a).min(255)) as u8);
            } else {
                rgb_data.extend_from_slice(&[255, 255, 255]);
            }
        }

        let stream_len = rgb_data.len();

        let mut pdf = Vec::new();

        // Header
        pdf.extend_from_slice(b"%PDF-1.4\n");

        // Object 1: Catalog
        let obj1_offset = pdf.len();
        pdf.extend_from_slice(b"1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n");

        // Object 2: Pages
        let obj2_offset = pdf.len();
        pdf.extend_from_slice(
            format!("2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n").as_bytes(),
        );

        // Object 3: Page
        let obj3_offset = pdf.len();
        pdf.extend_from_slice(
            format!(
                "3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 5 0 R /Resources << /XObject << /Img 4 0 R >> >> >>\nendobj\n",
                w, h
            )
            .as_bytes(),
        );

        // Object 4: Image XObject (raw RGB)
        let obj4_offset = pdf.len();
        pdf.extend_from_slice(
            format!(
                "4 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Length {} >>\nstream\n",
                w, h, stream_len
            )
            .as_bytes(),
        );
        pdf.extend_from_slice(&rgb_data);
        pdf.extend_from_slice(b"\nendstream\nendobj\n");

        // Object 5: Content stream (draw the image)
        let obj5_offset = pdf.len();
        let content = format!("{} 0 0 {} 0 0 cm /Img Do", w, h);
        pdf.extend_from_slice(
            format!(
                "5 0 obj\n<< /Length {} >>\nstream\n{}\nendstream\nendobj\n",
                content.len(),
                content
            )
            .as_bytes(),
        );

        // Cross-reference table
        let xref_offset = pdf.len();
        pdf.extend_from_slice(b"xref\n");
        pdf.extend_from_slice(format!("0 6\n").as_bytes());
        pdf.extend_from_slice(format!("0000000000 65535 f \n").as_bytes());
        pdf.extend_from_slice(format!("{:010} 00000 n \n", obj1_offset).as_bytes());
        pdf.extend_from_slice(format!("{:010} 00000 n \n", obj2_offset).as_bytes());
        pdf.extend_from_slice(format!("{:010} 00000 n \n", obj3_offset).as_bytes());
        pdf.extend_from_slice(format!("{:010} 00000 n \n", obj4_offset).as_bytes());
        pdf.extend_from_slice(format!("{:010} 00000 n \n", obj5_offset).as_bytes());

        // Trailer
        pdf.extend_from_slice(
            format!(
                "trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
                xref_offset
            )
            .as_bytes(),
        );

        pdf
    }

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
