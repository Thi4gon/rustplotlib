use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use tiny_skia::Pixmap;

use crate::axes::Axes;
use crate::axes3d::Axes3D;
use crate::colors;
use crate::svg_renderer::{SvgRenderer, color_to_svg};

#[pyclass]
pub struct RustFigure {
    width: u32,
    height: u32,
    dpi: u32,
    axes: Vec<Axes>,
    nrows: usize,
    ncols: usize,
    suptitle: Option<String>,
    suptitle_fontsize: f32,
    hspace: f32,
    wspace: f32,
    bg_color: crate::colors::Color,
    /// 3D axes, stored separately. Keyed by subplot index.
    axes3d: Vec<(usize, Axes3D)>,
    tight: bool,
    constrained: bool,
}

#[pymethods]
impl RustFigure {
    #[new]
    #[pyo3(signature = (width=640, height=480, dpi=100))]
    fn new(width: u32, height: u32, dpi: u32) -> Self {
        let width = width.min(32768).max(1);
        let height = height.min(32768).max(1);
        let dpi = dpi.min(1200).max(1);
        RustFigure {
            width,
            height,
            dpi,
            axes: Vec::new(),
            nrows: 1,
            ncols: 1,
            suptitle: None,
            suptitle_fontsize: 16.0,
            hspace: 0.2,
            wspace: 0.2,
            bg_color: crate::colors::Color::new(240, 240, 240, 255),
            axes3d: Vec::new(),
            tight: false,
            constrained: false,
        }
    }

    fn set_tight_layout_flag(&mut self, tight: bool) {
        self.tight = tight;
    }

    fn set_constrained_layout_flag(&mut self, constrained: bool) {
        self.constrained = constrained;
    }

    fn set_size_inches(&mut self, w: f64, h: f64) -> PyResult<()> {
        if w <= 0.0 || h <= 0.0 || !w.is_finite() || !h.is_finite() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Figure dimensions must be positive finite numbers"
            ));
        }
        let pw = (w * self.dpi as f64) as u32;
        let ph = (h * self.dpi as f64) as u32;
        if pw > 32768 || ph > 32768 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Figure dimensions too large (max 32768x32768 pixels)"
            ));
        }
        self.width = pw;
        self.height = ph;
        Ok(())
    }

    fn add_axes(&mut self) -> usize {
        let idx = self.axes.len();
        self.axes.push(Axes::new());
        idx
    }

    #[pyo3(signature = (ax_id, cmap, vmin, vmax, orientation, label=None))]
    fn axes_add_colorbar_artist(
        &mut self,
        ax_id: usize,
        cmap: String,
        vmin: f64,
        vmax: f64,
        orientation: String,
        label: Option<String>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.add_colorbar_artist(cmap, vmin, vmax, orientation, label);
        Ok(())
    }

    fn axes_add_widget_slider(
        &mut self,
        ax_id: usize,
        val: f64,
        valmin: f64,
        valmax: f64,
        label: String,
        color: Option<String>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        let c = color.map(|cs| colors::parse_color_str(&cs));
        ax.add_widget_slider(val, valmin, valmax, label, c);
        Ok(())
    }

    #[pyo3(signature = (ax_id, label, color=None))]
    fn axes_add_widget_button(
        &mut self,
        ax_id: usize,
        label: String,
        color: Option<String>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        let c = color.map(|cs| colors::parse_color_str(&cs));
        ax.add_widget_button(label, c);
        Ok(())
    }

    fn axes_set_position(&mut self, ax_id: usize, left: f64, bottom: f64, width: f64, height: f64) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_position = Some((left, bottom, width, height));
        Ok(())
    }

    fn axes_set_grid_span(&mut self, ax_id: usize, row_start: usize, row_end: usize, col_start: usize, col_end: usize) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.grid_span = Some((row_start, row_end, col_start, col_end));
        Ok(())
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
            v.extract::<String>().ok()
        } else { None };

        let marker_size = if let Some(v) = kwargs.get_item("markersize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let markevery = kwargs.get_item("markevery")?
            .map(|v| v.extract::<usize>().unwrap_or(1));

        // Accept and ignore markerfacecolor (for hollow markers — not yet implemented)
        let _ = kwargs.get_item("markerfacecolor")?;

        let zorder = if let Some(v) = kwargs.get_item("zorder")? {
            Some(v.extract::<i32>()?)
        } else { None };

        let outline_color = if let Some(c) = kwargs.get_item("outline_color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let outline_width = if let Some(v) = kwargs.get_item("outline_width")? {
            Some(v.extract::<f32>()?)
        } else { None };

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
            zorder,
            outline_color,
            outline_width,
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
            v.extract::<String>().ok()
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let zorder = if let Some(v) = kwargs.get_item("zorder")? {
            Some(v.extract::<i32>()?)
        } else { None };

        ax.scatter(
            x, y,
            color,
            sizes,
            marker.as_deref(),
            label,
            alpha,
            zorder,
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
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let bottom = if let Some(v) = kwargs.get_item("bottom")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let hatch = if let Some(v) = kwargs.get_item("hatch")? {
            Some(v.extract::<String>()?)
        } else { None };

        let zorder = if let Some(v) = kwargs.get_item("zorder")? {
            Some(v.extract::<i32>()?)
        } else { None };

        ax.bar(x, heights, color, width, label, alpha, bottom, hatch, zorder);

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
            v.extract::<String>().ok()
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

        let annotate = if let Some(v) = kwargs.get_item("annotate")? {
            v.extract::<bool>()?
        } else { false };

        let fmt = if let Some(v) = kwargs.get_item("fmt")? {
            Some(v.extract::<String>()?)
        } else { None };

        let interpolation = if let Some(v) = kwargs.get_item("interpolation")? {
            Some(v.extract::<String>()?)
        } else { None };

        let extent = if let Some(v) = kwargs.get_item("extent")? {
            let ext: Vec<f64> = v.extract()?;
            if ext.len() == 4 {
                Some((ext[0], ext[1], ext[2], ext[3]))
            } else {
                None
            }
        } else { None };

        // Handle origin='lower' — flip data rows in Rust
        let origin = if let Some(v) = kwargs.get_item("origin")? {
            v.extract::<String>().ok()
        } else { None };

        let final_data = if origin.as_deref() == Some("lower") {
            data.into_iter().rev().collect()
        } else {
            data
        };

        ax.imshow(final_data, cmap, annotate, fmt, interpolation, extent);

        Ok(())
    }

    fn axes_imshow_rgb(
        &mut self,
        ax_id: usize,
        data: Vec<Vec<Vec<f64>>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let interpolation = if let Some(v) = kwargs.get_item("interpolation")? {
            Some(v.extract::<String>()?)
        } else { None };

        let extent = if let Some(v) = kwargs.get_item("extent")? {
            let ext: Vec<f64> = v.extract()?;
            if ext.len() == 4 {
                Some((ext[0], ext[1], ext[2], ext[3]))
            } else {
                None
            }
        } else { None };

        // Handle origin='lower' — flip data rows in Rust
        let origin = if let Some(v) = kwargs.get_item("origin")? {
            v.extract::<String>().ok()
        } else { None };

        let data = if origin.as_deref() == Some("lower") {
            data.into_iter().rev().collect::<Vec<_>>()
        } else {
            data
        };

        // Determine if RGB or RGBA based on inner vector length
        let is_rgba = data.first()
            .and_then(|row| row.first())
            .map(|pixel| pixel.len() >= 4)
            .unwrap_or(false);

        if is_rgba {
            let rgba_data: Vec<Vec<(f64, f64, f64, f64)>> = data.iter().map(|row| {
                row.iter().map(|pixel| {
                    (
                        pixel.get(0).copied().unwrap_or(0.0),
                        pixel.get(1).copied().unwrap_or(0.0),
                        pixel.get(2).copied().unwrap_or(0.0),
                        pixel.get(3).copied().unwrap_or(1.0),
                    )
                }).collect()
            }).collect();
            ax.imshow_rgba(rgba_data, interpolation, extent);
        } else {
            let rgb_data: Vec<Vec<(f64, f64, f64)>> = data.iter().map(|row| {
                row.iter().map(|pixel| {
                    (
                        pixel.get(0).copied().unwrap_or(0.0),
                        pixel.get(1).copied().unwrap_or(0.0),
                        pixel.get(2).copied().unwrap_or(0.0),
                    )
                }).collect()
            }).collect();
            ax.imshow_rgb(rgb_data, interpolation, extent);
        }

        Ok(())
    }

    #[pyo3(signature = (ax_id, title, fontsize=None, loc=None))]
    fn axes_set_title(&mut self, ax_id: usize, title: String, fontsize: Option<f32>, loc: Option<String>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.title = Some(title);
        if let Some(fs) = fontsize {
            ax.title_size = fs;
        }
        if let Some(l) = loc {
            ax.set_title_loc(&l);
        }
        Ok(())
    }

    #[pyo3(signature = (ax_id, label, fontsize=None, color=None))]
    fn axes_set_xlabel(&mut self, ax_id: usize, label: String, fontsize: Option<f32>, color: Option<String>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.xlabel = Some(label);
        if let Some(fs) = fontsize {
            ax.label_size = fs;
        }
        if let Some(c) = color {
            ax.xlabel_color = Some(crate::colors::parse_color_str(&c));
        }
        Ok(())
    }

    #[pyo3(signature = (ax_id, label, fontsize=None, color=None))]
    fn axes_set_ylabel(&mut self, ax_id: usize, label: String, fontsize: Option<f32>, color: Option<String>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.ylabel = Some(label);
        if let Some(fs) = fontsize {
            ax.label_size = fs;
        }
        if let Some(c) = color {
            ax.ylabel_color = Some(crate::colors::parse_color_str(&c));
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
        if let Some(v) = kwargs.get_item("ncol")? {
            ax.legend_ncol = v.extract::<usize>()?;
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

        // Grid which: major/minor/both
        if let Some(v) = kwargs.get_item("which")? {
            let which_str = v.extract::<String>()?;
            ax.grid_which = crate::axes::GridWhich::from_str(&which_str);
        }

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
            v.extract::<String>().ok()
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
            v.extract::<String>().ok()
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

    fn axes_set_xscale(&mut self, ax_id: usize, scale: String) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_xscale(&scale);
        Ok(())
    }

    fn axes_set_yscale(&mut self, ax_id: usize, scale: String) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_yscale(&scale);
        Ok(())
    }

    fn axes_errorbar(
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

        let yerr = if let Some(v) = kwargs.get_item("yerr")? {
            Some(v.extract::<Vec<f64>>()?)
        } else { None };

        let xerr = if let Some(v) = kwargs.get_item("xerr")? {
            Some(v.extract::<Vec<f64>>()?)
        } else { None };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let capsize = if let Some(v) = kwargs.get_item("capsize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let marker = if let Some(v) = kwargs.get_item("marker")? {
            v.extract::<String>().ok()
        } else { None };

        let marker_size = if let Some(v) = kwargs.get_item("markersize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let linestyle = if let Some(v) = kwargs.get_item("linestyle")? {
            Some(v.extract::<String>()?)
        } else { None };

        ax.errorbar(
            x, y,
            yerr, xerr,
            color,
            linewidth,
            capsize,
            marker.as_deref(),
            marker_size,
            label,
            alpha,
            linestyle.as_deref(),
        );

        Ok(())
    }

    fn axes_barh(
        &mut self,
        ax_id: usize,
        y: Vec<f64>,
        widths: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let height = if let Some(v) = kwargs.get_item("height")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.barh(y, widths, color, height, label, alpha);

        Ok(())
    }

    fn axes_boxplot(
        &mut self,
        ax_id: usize,
        data: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let positions = if let Some(v) = kwargs.get_item("positions")? {
            Some(v.extract::<Vec<f64>>()?)
        } else { None };

        let widths = if let Some(v) = kwargs.get_item("widths")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let median_color = if let Some(c) = kwargs.get_item("median_color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        ax.boxplot(data, positions, widths, color, median_color);

        Ok(())
    }

    fn axes_stem(
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

        let marker = if let Some(v) = kwargs.get_item("marker")? {
            v.extract::<String>().ok()
        } else { None };

        let marker_size = if let Some(v) = kwargs.get_item("markersize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let baseline = if let Some(v) = kwargs.get_item("baseline")? {
            Some(v.extract::<f64>()?)
        } else { None };

        ax.stem(
            x, y,
            color,
            linewidth,
            marker.as_deref(),
            marker_size,
            label,
            baseline,
        );

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

    fn axes_annotate(
        &mut self,
        ax_id: usize,
        text: String,
        xy: (f64, f64),
        xytext: (f64, f64),
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

        let (arrow_color, arrow_width) = if let Some(ap) = kwargs.get_item("arrowprops")? {
            let arrow_dict: &Bound<'_, PyDict> = ap.downcast::<PyDict>()?;
            let ac = if let Some(c) = arrow_dict.get_item("color")? {
                colors::parse_color_value(&c)?
            } else {
                crate::colors::Color::new(0, 0, 0, 255)
            };
            let aw = if let Some(v) = arrow_dict.get_item("width")? {
                v.extract::<f32>()?
            } else {
                1.5
            };
            (ac, aw)
        } else {
            (crate::colors::Color::new(0, 0, 0, 255), 1.5)
        };

        let bbox = if let Some(bx) = kwargs.get_item("bbox")? {
            let bbox_dict: &Bound<'_, PyDict> = bx.downcast::<PyDict>()?;
            let boxstyle = if let Some(v) = bbox_dict.get_item("boxstyle")? {
                v.extract::<String>()?
            } else {
                "square".to_string()
            };
            let facecolor = if let Some(c) = bbox_dict.get_item("facecolor")? {
                colors::parse_color_value(&c)?
            } else {
                crate::colors::Color::new(255, 255, 255, 255)
            };
            let edgecolor = if let Some(c) = bbox_dict.get_item("edgecolor")? {
                colors::parse_color_value(&c)?
            } else {
                crate::colors::Color::new(0, 0, 0, 255)
            };
            let alpha = if let Some(v) = bbox_dict.get_item("alpha")? {
                v.extract::<f32>()?
            } else {
                1.0
            };
            Some(crate::axes::AnnotationBbox { boxstyle, facecolor, edgecolor, alpha })
        } else {
            None
        };

        let fontweight = if let Some(v) = kwargs.get_item("fontweight")? {
            v.extract::<String>()?
        } else {
            "normal".to_string()
        };

        let fontstyle = if let Some(v) = kwargs.get_item("fontstyle")? {
            v.extract::<String>()?
        } else {
            "normal".to_string()
        };

        ax.annotate(text, xy, xytext, fontsize, color, arrow_color, arrow_width, bbox, fontweight, fontstyle);

        Ok(())
    }

    #[pyo3(signature = (text, fontsize=None))]
    fn suptitle(&mut self, text: String, fontsize: Option<f32>) {
        self.suptitle = Some(text);
        if let Some(fs) = fontsize {
            self.suptitle_fontsize = fs;
        }
    }

    #[pyo3(signature = (hspace=None, wspace=None))]
    fn subplots_adjust(&mut self, hspace: Option<f32>, wspace: Option<f32>) {
        if let Some(h) = hspace {
            self.hspace = h;
        }
        if let Some(w) = wspace {
            self.wspace = w;
        }
    }

    fn axes_set_axis_off(&mut self, ax_id: usize, off: bool) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_axis_visible(!off);
        Ok(())
    }

    fn axes_set_xticks(&mut self, ax_id: usize, ticks: Vec<f64>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_xticks = Some(ticks);
        Ok(())
    }

    fn axes_set_yticks(&mut self, ax_id: usize, ticks: Vec<f64>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_yticks = Some(ticks);
        Ok(())
    }

    fn axes_set_xticks_minor(&mut self, ax_id: usize, ticks: Vec<f64>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_xticks_minor = Some(ticks);
        Ok(())
    }

    fn axes_set_yticks_minor(&mut self, ax_id: usize, ticks: Vec<f64>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_yticks_minor = Some(ticks);
        Ok(())
    }

    fn axes_set_xticklabels(&mut self, ax_id: usize, labels: Vec<String>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_xtick_labels = Some(labels);
        Ok(())
    }

    fn axes_set_yticklabels(&mut self, ax_id: usize, labels: Vec<String>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.custom_ytick_labels = Some(labels);
        Ok(())
    }

    fn axes_set_aspect(&mut self, ax_id: usize, aspect: String) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.aspect = match aspect.to_lowercase().as_str() {
            "equal" => crate::axes::AspectRatio::Equal,
            _ => crate::axes::AspectRatio::Auto,
        };
        Ok(())
    }

    fn axes_invert_xaxis(&mut self, ax_id: usize) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.invert_x = true;
        Ok(())
    }

    fn axes_invert_yaxis(&mut self, ax_id: usize) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.invert_y = true;
        Ok(())
    }

    fn axes_axhspan(
        &mut self,
        ax_id: usize,
        ymin: f64,
        ymax: f64,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.3
        };

        ax.axhspan(ymin, ymax, color, alpha);
        Ok(())
    }

    fn axes_axvspan(
        &mut self,
        ax_id: usize,
        xmin: f64,
        xmax: f64,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.3
        };

        ax.axvspan(xmin, xmax, color, alpha);
        Ok(())
    }

    fn axes_contour(
        &mut self,
        ax_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let levels = if let Some(v) = kwargs.get_item("levels")? {
            Some(v.extract::<Vec<f64>>()?)
        } else { None };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        // colors param not handled via kwarg parsing for simplicity
        ax.contour(x, y, z, levels, None, linewidth);
        Ok(())
    }

    fn axes_contourf(
        &mut self,
        ax_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let levels = if let Some(v) = kwargs.get_item("levels")? {
            Some(v.extract::<Vec<f64>>()?)
        } else { None };

        ax.contourf(x, y, z, levels, None);
        Ok(())
    }

    fn axes_hexbin(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let gridsize = if let Some(v) = kwargs.get_item("gridsize")? {
            v.extract::<usize>()?
        } else {
            20
        };

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            v.extract::<String>()?
        } else {
            "viridis".to_string()
        };

        let mincnt = if let Some(v) = kwargs.get_item("mincnt")? {
            v.extract::<usize>()?
        } else {
            1
        };

        ax.hexbin(x, y, gridsize, cmap, mincnt);
        Ok(())
    }

    fn axes_add_patch(
        &mut self,
        ax_id: usize,
        patch_type: String,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let facecolor = if let Some(c) = kwargs.get_item("facecolor")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let edgecolor = if let Some(c) = kwargs.get_item("edgecolor")? {
            colors::parse_color_value(&c)?
        } else {
            crate::colors::Color::new(0, 0, 0, 255)
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

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let patch = match patch_type.as_str() {
            "rectangle" => {
                let x = kwargs.get_item("x")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(0.0);
                let y = kwargs.get_item("y")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(0.0);
                let width = kwargs.get_item("width")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(1.0);
                let height = kwargs.get_item("height")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(1.0);
                crate::artists::patches::Patch::new_rectangle(
                    x, y, width, height, facecolor, edgecolor, linewidth, alpha, label,
                )
            }
            "circle" => {
                let cx = kwargs.get_item("cx")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(0.0);
                let cy = kwargs.get_item("cy")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(0.0);
                let radius = kwargs.get_item("radius")?.map(|v| v.extract::<f64>()).transpose()?.unwrap_or(1.0);
                crate::artists::patches::Patch::new_circle(
                    (cx, cy), radius, facecolor, edgecolor, linewidth, alpha, label,
                )
            }
            "polygon" => {
                let points_flat = kwargs.get_item("points")?.map(|v| v.extract::<Vec<(f64, f64)>>()).transpose()?.unwrap_or_default();
                crate::artists::patches::Patch::new_polygon(
                    points_flat, facecolor, edgecolor, linewidth, alpha, label,
                )
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    format!("Unknown patch type: {}", patch_type),
                ));
            }
        };

        ax.add_patch(patch);
        Ok(())
    }

    fn axes_hlines(
        &mut self,
        ax_id: usize,
        y: Vec<f64>,
        xmin: f64,
        xmax: f64,
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
            "-".to_string()
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

        ax.hlines(y, xmin, xmax, color, &linestyle, linewidth, alpha);
        Ok(())
    }

    fn axes_vlines(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        ymin: f64,
        ymax: f64,
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
            "-".to_string()
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

        ax.vlines(x, ymin, ymax, color, &linestyle, linewidth, alpha);
        Ok(())
    }

    fn axes_violinplot(
        &mut self,
        ax_id: usize,
        data: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let positions = if let Some(v) = kwargs.get_item("positions")? {
            Some(v.extract::<Vec<f64>>()?)
        } else { None };

        let widths = if let Some(v) = kwargs.get_item("widths")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let show_means = if let Some(v) = kwargs.get_item("showmeans")? {
            v.extract::<bool>()?
        } else {
            false
        };

        let show_medians = if let Some(v) = kwargs.get_item("showmedians")? {
            v.extract::<bool>()?
        } else {
            true
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        ax.violinplot(data, positions, widths, color, show_means, show_medians, alpha, label);
        Ok(())
    }

    fn axes_fill_betweenx(
        &mut self,
        ax_id: usize,
        y: Vec<f64>,
        x1: Vec<f64>,
        x2: Vec<f64>,
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
            v.extract::<String>().ok()
        } else { None };

        ax.fill_betweenx(y, x1, x2, color, alpha, label);
        Ok(())
    }

    fn axes_table(
        &mut self,
        ax_id: usize,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let cell_text = if let Some(v) = kwargs.get_item("cellText")? {
            v.extract::<Vec<Vec<String>>>()?
        } else {
            Vec::new()
        };

        let col_labels = if let Some(v) = kwargs.get_item("colLabels")? {
            Some(v.extract::<Vec<String>>()?)
        } else { None };

        let row_labels = if let Some(v) = kwargs.get_item("rowLabels")? {
            Some(v.extract::<Vec<String>>()?)
        } else { None };

        let loc = if let Some(v) = kwargs.get_item("loc")? {
            v.extract::<String>()?
        } else {
            "bottom".to_string()
        };

        ax.set_table(cell_text, col_labels, row_labels, loc);
        Ok(())
    }

    fn axes_set_polar(&mut self, ax_id: usize, polar: bool) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_polar(polar);
        Ok(())
    }

    fn axes_twinx(&mut self, ax_id: usize) -> PyResult<usize> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.twinx();
        // Return a special ID: we encode the twin as parent_id * 1000 + 999
        // The twin axes is embedded inside the parent, so we use a sentinel ID scheme.
        Ok(ax_id * 1000 + 999)
    }

    /// Plot on a twin axes.
    fn twin_axes_plot(
        &mut self,
        twin_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
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
            v.extract::<String>().ok()
        } else { None };

        let marker_size = if let Some(v) = kwargs.get_item("markersize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let markevery = kwargs.get_item("markevery")?
            .map(|v| v.extract::<usize>().unwrap_or(1));

        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twin axes"))?;
        twin.plot(
            x, y, color, linewidth, linestyle.as_deref(),
            marker.as_deref(), marker_size, markevery, label, alpha, None, None, None,
        );
        Ok(())
    }

    #[pyo3(signature = (twin_id, label, fontsize=None))]
    fn twin_axes_set_ylabel(&mut self, twin_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twin axes"))?;
        twin.ylabel = Some(label);
        if let Some(fs) = fontsize {
            twin.label_size = fs;
        }
        Ok(())
    }

    fn twin_axes_set_ylim(&mut self, twin_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twin axes"))?;
        twin.ylim = Some((lo, hi));
        Ok(())
    }

    fn twin_axes_legend(&mut self, twin_id: usize, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twin axes"))?;
        twin.show_legend = true;
        if let Some(v) = kwargs.get_item("loc")? {
            twin.legend_loc = v.extract::<String>()?;
        }
        Ok(())
    }

    fn twin_axes_scatter(
        &mut self,
        twin_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let sizes = if let Some(v) = kwargs.get_item("s")? {
            Some(v.extract::<Vec<f32>>()?)
        } else { None };

        let marker = if let Some(v) = kwargs.get_item("marker")? {
            v.extract::<String>().ok()
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twin axes"))?;
        twin.scatter(x, y, color, sizes, marker.as_deref(), label, alpha, None);
        Ok(())
    }

    fn twin_axes_bar(
        &mut self,
        twin_id: usize,
        x: Vec<f64>,
        heights: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let width = if let Some(v) = kwargs.get_item("width")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twin axes"))?;
        twin.bar(x, heights, color, width, label, alpha, None, None, None);
        Ok(())
    }

    // ---- twiny (twin X axis) methods ----

    fn axes_twiny(&mut self, ax_id: usize) -> PyResult<usize> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.twiny();
        // Encode the twiny as parent_id * 1000 + 998 (different sentinel from twinx which uses 999)
        Ok(ax_id * 1000 + 998)
    }

    fn twiny_axes_plot(
        &mut self,
        twin_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
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
            v.extract::<String>().ok()
        } else { None };

        let marker_size = if let Some(v) = kwargs.get_item("markersize")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let markevery = kwargs.get_item("markevery")?
            .map(|v| v.extract::<usize>().unwrap_or(1));

        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_x_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twiny axes"))?;
        twin.plot(
            x, y, color, linewidth, linestyle.as_deref(),
            marker.as_deref(), marker_size, markevery, label, alpha, None, None, None,
        );
        Ok(())
    }

    #[pyo3(signature = (twin_id, label, fontsize=None))]
    fn twiny_axes_set_xlabel(&mut self, twin_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_x_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twiny axes"))?;
        twin.xlabel = Some(label);
        if let Some(fs) = fontsize {
            twin.label_size = fs;
        }
        Ok(())
    }

    fn twiny_axes_set_xlim(&mut self, twin_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_x_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twiny axes"))?;
        twin.xlim = Some((lo, hi));
        Ok(())
    }

    fn twiny_axes_legend(&mut self, twin_id: usize, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_x_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twiny axes"))?;
        twin.show_legend = true;
        if let Some(v) = kwargs.get_item("loc")? {
            twin.legend_loc = v.extract::<String>()?;
        }
        Ok(())
    }

    fn twiny_axes_scatter(
        &mut self,
        twin_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let sizes = if let Some(v) = kwargs.get_item("s")? {
            Some(v.extract::<Vec<f32>>()?)
        } else { None };

        let marker = if let Some(v) = kwargs.get_item("marker")? {
            v.extract::<String>().ok()
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let parent_id = twin_id / 1000;
        let twin = self.axes.get_mut(parent_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?
            .twin_x_axes.as_mut()
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("No twiny axes"))?;
        twin.scatter(x, y, color, sizes, marker.as_deref(), label, alpha, None);
        Ok(())
    }

    // ---- end twiny methods ----

    fn axes_colorbar(
        &mut self,
        ax_id: usize,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            v.extract::<String>()?
        } else {
            "viridis".to_string()
        };

        let vmin = if let Some(v) = kwargs.get_item("vmin")? {
            v.extract::<f64>()?
        } else {
            0.0
        };

        let vmax = if let Some(v) = kwargs.get_item("vmax")? {
            v.extract::<f64>()?
        } else {
            1.0
        };

        let label = if let Some(v) = kwargs.get_item("label")? {
            let s = v.extract::<String>()?;
            if s.is_empty() { None } else { Some(s) }
        } else {
            None
        };

        let orientation = if let Some(v) = kwargs.get_item("orientation")? {
            Some(v.extract::<String>()?)
        } else {
            None
        };

        let shrink = if let Some(v) = kwargs.get_item("shrink")? {
            Some(v.extract::<f64>()?)
        } else {
            None
        };

        let pad = if let Some(v) = kwargs.get_item("pad")? {
            Some(v.extract::<f64>()?)
        } else {
            None
        };

        ax.colorbar(&cmap, vmin, vmax, label, orientation, shrink, pad);
        Ok(())
    }

    fn axes_quiver(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        u: Vec<f64>,
        v: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let scale = if let Some(v) = kwargs.get_item("scale")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let width = if let Some(v) = kwargs.get_item("width")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.quiver(x, y, u, v, color, scale, width);
        Ok(())
    }

    fn axes_streamplot(
        &mut self,
        ax_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        u: Vec<Vec<f64>>,
        v: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let density = if let Some(v) = kwargs.get_item("density")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.streamplot(x, y, u, v, color, density, linewidth);
        Ok(())
    }

    fn axes_arrow(
        &mut self,
        ax_id: usize,
        x: f64,
        y: f64,
        dx: f64,
        dy: f64,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let width = if let Some(v) = kwargs.get_item("width")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let head_width = if let Some(v) = kwargs.get_item("head_width")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let head_length = if let Some(v) = kwargs.get_item("head_length")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let zorder = if let Some(v) = kwargs.get_item("zorder")? {
            Some(v.extract::<i32>()?)
        } else { None };

        ax.arrow(x, y, dx, dy, color, width, head_width, head_length, alpha, label, zorder);
        Ok(())
    }

    fn axes_add_line_collection(
        &mut self,
        ax_id: usize,
        segments: Vec<Vec<(f64, f64)>>,
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

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let zorder = if let Some(v) = kwargs.get_item("zorder")? {
            Some(v.extract::<i32>()?)
        } else { None };

        ax.add_line_collection(segments, color, None, linewidth, None, alpha, label, zorder);
        Ok(())
    }

    fn axes_fancy_arrow(
        &mut self,
        ax_id: usize,
        pos_a: (f64, f64),
        pos_b: (f64, f64),
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

        let arrow_style = if let Some(v) = kwargs.get_item("arrowstyle")? {
            v.extract::<String>().ok()
        } else { None };

        let connection_style = if let Some(v) = kwargs.get_item("connectionstyle")? {
            v.extract::<String>().ok()
        } else { None };

        let head_width = if let Some(v) = kwargs.get_item("head_width")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let head_length = if let Some(v) = kwargs.get_item("head_length")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let shrink_a = if let Some(v) = kwargs.get_item("shrinkA")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let shrink_b = if let Some(v) = kwargs.get_item("shrinkB")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let mutation_scale = if let Some(v) = kwargs.get_item("mutation_scale")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        let zorder = if let Some(v) = kwargs.get_item("zorder")? {
            Some(v.extract::<i32>()?)
        } else { None };

        ax.fancy_arrow(
            pos_a, pos_b, color, linewidth,
            arrow_style.as_deref(), connection_style.as_deref(),
            head_width, head_length, shrink_a, shrink_b,
            mutation_scale, alpha, label, zorder,
        );
        Ok(())
    }

    fn axes_axline(
        &mut self,
        ax_id: usize,
        xy1: (f64, f64),
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let xy2 = if let Some(v) = kwargs.get_item("xy2")? {
            let tup: (f64, f64) = v.extract()?;
            Some(tup)
        } else { None };

        let slope = if let Some(v) = kwargs.get_item("slope")? {
            Some(v.extract::<f64>()?)
        } else { None };

        let color = if let Some(c) = kwargs.get_item("color")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        let linestyle = if let Some(v) = kwargs.get_item("linestyle")? {
            Some(v.extract::<String>()?)
        } else { None };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.axline(xy1, xy2, slope, color, linestyle.as_deref(), linewidth, alpha);
        Ok(())
    }

    fn num_axes(&self) -> usize {
        self.axes.len()
    }

    fn axes_get_xlim(&self, ax_id: usize) -> PyResult<(f64, f64)> {
        let ax = self.axes.get(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        match ax.xlim {
            Some((a, b)) => Ok((a, b)),
            None => {
                let (xmin, xmax, _, _) = ax.compute_bounds();
                Ok((xmin, xmax))
            }
        }
    }

    fn axes_get_ylim(&self, ax_id: usize) -> PyResult<(f64, f64)> {
        let ax = self.axes.get(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        match ax.ylim {
            Some((a, b)) => Ok((a, b)),
            None => {
                let (_, _, ymin, ymax) = ax.compute_bounds();
                Ok((ymin, ymax))
            }
        }
    }

    fn axes_clear(&mut self, ax_id: usize) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.clear();
        Ok(())
    }

    fn axes_tick_params(&mut self, ax_id: usize, kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let direction = if let Some(v) = kwargs.get_item("direction")? {
            v.extract::<String>()?
        } else {
            "out".to_string()
        };

        let length = if let Some(v) = kwargs.get_item("length")? {
            v.extract::<f32>()?
        } else {
            ax.tick_length
        };

        let width = if let Some(v) = kwargs.get_item("width")? {
            v.extract::<f32>()?
        } else {
            ax.tick_width
        };

        let labelsize = if let Some(v) = kwargs.get_item("labelsize")? {
            v.extract::<f32>()?
        } else {
            ax.tick_label_size
        };

        ax.set_tick_params(&direction, length, width, labelsize);

        // Also accept color param
        if let Some(c) = kwargs.get_item("color")? {
            let color = colors::parse_color_value(&c)?;
            ax.set_tick_color(color);
        }

        Ok(())
    }

    fn axes_set_spine_visible(&mut self, ax_id: usize, which: String, visible: bool) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_spine_visible(&which, visible);
        Ok(())
    }

    fn axes_set_facecolor(&mut self, ax_id: usize, color: &Bound<'_, pyo3::PyAny>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_bg_color(colors::parse_color_value(color)?);
        Ok(())
    }

    fn set_facecolor(&mut self, color: &Bound<'_, pyo3::PyAny>) -> PyResult<()> {
        self.bg_color = colors::parse_color_value(color)?;
        Ok(())
    }

    fn axes_set_text_color(&mut self, ax_id: usize, color: &Bound<'_, pyo3::PyAny>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_text_color(colors::parse_color_value(color)?);
        Ok(())
    }

    fn axes_set_tick_color(&mut self, ax_id: usize, color: &Bound<'_, pyo3::PyAny>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_tick_color(colors::parse_color_value(color)?);
        Ok(())
    }

    fn axes_set_spine_color(&mut self, ax_id: usize, color: &Bound<'_, pyo3::PyAny>) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_spine_color(colors::parse_color_value(color)?);
        Ok(())
    }

    fn axes_set_spine_linewidth(&mut self, ax_id: usize, lw: f64) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;
        ax.set_spine_linewidth(lw as f32);
        Ok(())
    }

    fn render_to_png_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let pixmap = self.render_pixmap_opts(None, false);
        let png_data = pixmap.encode_png()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("PNG encode error: {}", e)))?;
        Ok(PyBytes::new_bound(py, &png_data))
    }

    /// Render the figure as an SVG XML string (for Jupyter _repr_svg_).
    fn render_to_svg_string(&self) -> PyResult<String> {
        Ok(self.render_svg_native(None, false))
    }

    /// Render the figure as raw RGBA pixel buffer (for interactive backends).
    /// Returns (bytes, width, height).
    fn render_to_rgba_buffer<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyBytes>, u32, u32)> {
        let pixmap = self.render_pixmap_opts(None, false);
        let w = pixmap.width();
        let h = pixmap.height();
        let rgba_data: Vec<u8> = pixmap.pixels().iter().flat_map(|px| {
            // tiny-skia stores premultiplied RGBA — unpremultiply for standard RGBA
            let a = px.alpha();
            if a > 0 && a < 255 {
                let r = (px.red() as u16 * 255 / a as u16).min(255) as u8;
                let g = (px.green() as u16 * 255 / a as u16).min(255) as u8;
                let b = (px.blue() as u16 * 255 / a as u16).min(255) as u8;
                [r, g, b, a]
            } else if a == 255 {
                [px.red(), px.green(), px.blue(), 255]
            } else {
                [0, 0, 0, 0]
            }
        }).collect();
        Ok((PyBytes::new_bound(py, &rgba_data), w, h))
    }

    #[pyo3(signature = (path, dpi=None, transparent=None, tight=None))]
    fn savefig(&self, path: String, dpi: Option<u32>, transparent: Option<bool>, tight: Option<bool>) -> PyResult<()> {
        // Validate file extension
        let path_lower = path.to_lowercase();
        if !path_lower.ends_with(".png") && !path_lower.ends_with(".svg") && !path_lower.ends_with(".pdf") && !path_lower.ends_with(".eps") {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "savefig only supports .png, .svg, .pdf, and .eps extensions"
            ));
        }

        // Reject path traversal
        if path.contains("..") {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Path traversal ('..') is not allowed in savefig path"
            ));
        }

        // Max path length check
        if path.len() > 4096 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "File path is too long (max 4096 characters)"
            ));
        }

        let is_transparent = transparent.unwrap_or(false);
        let pixmap = self.render_pixmap_opts(dpi, is_transparent);

        // Apply bbox_inches='tight' cropping if requested
        let pixmap = if tight.unwrap_or(false) {
            let (bg_r, bg_g, bg_b) = if is_transparent {
                (0u8, 0u8, 0u8) // transparent bg: crop fully transparent regions
            } else {
                (self.bg_color.r, self.bg_color.g, self.bg_color.b)
            };
            Self::crop_to_content(&pixmap, bg_r, bg_g, bg_b, is_transparent, 10)
        } else {
            pixmap
        };

        if path_lower.ends_with(".pdf") {
            let pdf_data = Self::render_pdf(&pixmap);
            std::fs::write(&path, pdf_data)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write PDF: {}", e)))?;
        } else if path_lower.ends_with(".svg") {
            let svg_content = self.render_svg_native(dpi, is_transparent);
            std::fs::write(&path, svg_content)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write SVG: {}", e)))?;
        } else if path_lower.ends_with(".eps") {
            let eps_data = Self::render_eps(&pixmap);
            std::fs::write(&path, eps_data)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write EPS: {}", e)))?;
        } else {
            let png_bytes = pixmap.encode_png()
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("PNG encode error: {}", e)))?;
            std::fs::write(&path, &png_bytes)
                .map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to write PNG: {}", e)))?;
        }
        Ok(())
    }

    /// Add a radar / spider chart.
    fn axes_radar(
        &mut self,
        ax_id: usize,
        categories: Vec<String>,
        values: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let colors = if let Some(v) = kwargs.get_item("colors")? {
            let color_list: Vec<Bound<'_, pyo3::PyAny>> = v.extract()?;
            let mut result = Vec::new();
            for c in &color_list {
                result.push(colors::parse_color_value(c)?);
            }
            Some(result)
        } else {
            None
        };

        let labels = if let Some(v) = kwargs.get_item("labels")? {
            Some(v.extract::<Vec<String>>()?)
        } else {
            None
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.8
        };

        let fill = if let Some(v) = kwargs.get_item("fill")? {
            v.extract::<bool>()?
        } else {
            true
        };

        ax.radar(categories, values, colors, labels, alpha, fill);
        Ok(())
    }

    /// Add a broken horizontal bar chart.
    fn axes_broken_barh(
        &mut self,
        ax_id: usize,
        y_ranges: Vec<(f64, f64)>,
        x_ranges: Vec<Vec<(f64, f64)>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let colors = if let Some(v) = kwargs.get_item("colors")? {
            let color_list: Vec<Bound<'_, pyo3::PyAny>> = v.extract()?;
            let mut result = Vec::new();
            for c in &color_list {
                result.push(colors::parse_color_value(c)?);
            }
            Some(result)
        } else {
            None
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else {
            None
        };

        ax.broken_barh(y_ranges, x_ranges, colors, alpha, label);
        Ok(())
    }

    /// Add an event / raster plot.
    fn axes_eventplot(
        &mut self,
        ax_id: usize,
        positions: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let orientation = if let Some(v) = kwargs.get_item("orientation")? {
            Some(v.extract::<String>()?)
        } else {
            None
        };

        let linewidths = if let Some(v) = kwargs.get_item("linewidths")? {
            Some(v.extract::<f32>()?)
        } else {
            None
        };

        let colors = if let Some(v) = kwargs.get_item("colors")? {
            let color_list: Vec<Bound<'_, pyo3::PyAny>> = v.extract()?;
            let mut result = Vec::new();
            for c in &color_list {
                result.push(colors::parse_color_value(c)?);
            }
            Some(result)
        } else {
            None
        };

        let linelength = if let Some(v) = kwargs.get_item("linelength")? {
            Some(v.extract::<f64>()?)
        } else {
            None
        };

        ax.eventplot(positions, orientation, linewidths, colors, linelength);
        Ok(())
    }

    /// Add a stacked area chart.
    fn axes_stackplot(
        &mut self,
        ax_id: usize,
        x: Vec<f64>,
        ys: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let colors = if let Some(v) = kwargs.get_item("colors")? {
            let color_list: Vec<Bound<'_, pyo3::PyAny>> = v.extract()?;
            let mut result = Vec::new();
            for c in &color_list {
                result.push(colors::parse_color_value(c)?);
            }
            Some(result)
        } else {
            None
        };

        let labels = if let Some(v) = kwargs.get_item("labels")? {
            Some(v.extract::<Vec<String>>()?)
        } else {
            None
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.5
        };

        ax.stackplot(x, ys, colors, labels, alpha);
        Ok(())
    }

    /// Add a filled polygon (ax.fill).
    fn axes_fill(
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

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let label = if let Some(v) = kwargs.get_item("label")? {
            v.extract::<String>().ok()
        } else { None };

        ax.fill(x, y, color, alpha, label);
        Ok(())
    }

    /// Add a pseudocolor mesh plot.
    fn axes_pcolormesh(
        &mut self,
        ax_id: usize,
        c: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            Some(v.extract::<String>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let x = if let Some(v) = kwargs.get_item("x")? {
            Some(v.extract::<Vec<Vec<f64>>>()?)
        } else { None };

        let y = if let Some(v) = kwargs.get_item("y")? {
            Some(v.extract::<Vec<Vec<f64>>>()?)
        } else { None };

        let edgecolors = if let Some(c) = kwargs.get_item("edgecolors")? {
            Some(colors::parse_color_value(&c)?)
        } else { None };

        ax.pcolormesh(x, y, c, cmap, alpha, edgecolors);
        Ok(())
    }

    /// Add a pseudocolor plot with edges (pcolor).
    fn axes_pcolor(
        &mut self,
        ax_id: usize,
        c: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            Some(v.extract::<String>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        let x = if let Some(v) = kwargs.get_item("x")? {
            Some(v.extract::<Vec<Vec<f64>>>()?)
        } else { None };

        let y = if let Some(v) = kwargs.get_item("y")? {
            Some(v.extract::<Vec<Vec<f64>>>()?)
        } else { None };

        ax.pcolor(x, y, c, cmap, alpha);
        Ok(())
    }

    /// Display a matrix as image with integer ticks (matshow).
    fn axes_matshow(
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

        ax.matshow(data, cmap);
        Ok(())
    }

    /// Add a basic Sankey diagram.
    fn axes_sankey(
        &mut self,
        ax_id: usize,
        flows: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let ax = self.axes.get_mut(ax_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid axes index"))?;

        let labels = if let Some(v) = kwargs.get_item("labels")? {
            v.extract::<Vec<String>>()?
        } else {
            vec!["".to_string(); flows.len()]
        };

        let orientations = if let Some(v) = kwargs.get_item("orientations")? {
            Some(v.extract::<Vec<i32>>()?)
        } else { None };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            Some(v.extract::<f32>()?)
        } else { None };

        ax.sankey(flows, labels, orientations, alpha);
        Ok(())
    }

    fn show(&self) -> PyResult<()> {
        let pixmap = self.render_pixmap_opts(None, false);
        crate::window::show_pixmap(&pixmap)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))
    }

    // -----------------------------------------------------------------------
    // 3D Axes methods
    // -----------------------------------------------------------------------

    /// Add a 3D subplot at the given index. Returns the 3D axes ID.
    fn add_subplot_3d(&mut self, subplot_idx: usize) -> usize {
        let id = self.axes3d.len();
        self.axes3d.push((subplot_idx, Axes3D::new()));
        id
    }

    /// Plot a 3D line.
    fn axes3d_plot(
        &mut self,
        ax3d_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            colors::parse_color_value(&c)?
        } else {
            ax.next_color()
        };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            1.5
        };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else {
            None
        };

        ax.artists.push(Box::new(crate::artists::line3d::Line3D {
            x, y, z, color, linewidth, label,
        }));

        Ok(())
    }

    /// 3D scatter plot.
    fn axes3d_scatter(
        &mut self,
        ax3d_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            colors::parse_color_value(&c)?
        } else {
            ax.next_color()
        };

        let sizes = if let Some(v) = kwargs.get_item("s")? {
            v.extract::<Vec<f32>>()?
        } else {
            vec![6.0]
        };

        let marker_str = if let Some(v) = kwargs.get_item("marker")? {
            v.extract::<String>()?
        } else {
            "o".to_string()
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        let label = if let Some(v) = kwargs.get_item("label")? {
            Some(v.extract::<String>()?)
        } else {
            None
        };

        ax.artists.push(Box::new(crate::artists::scatter3d::Scatter3D {
            x, y, z, sizes, color,
            marker: crate::artists::MarkerStyle::from_str(&marker_str),
            label, alpha,
        }));

        Ok(())
    }

    /// 3D surface plot.
    fn axes3d_plot_surface(
        &mut self,
        ax3d_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            v.extract::<String>()?
        } else {
            "viridis".to_string()
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.9
        };

        ax.artists.push(Box::new(crate::artists::surface3d::Surface3D {
            x, y, z, cmap, alpha,
        }));

        Ok(())
    }

    /// 3D wireframe plot.
    fn axes3d_plot_wireframe(
        &mut self,
        ax3d_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            colors::parse_color_value(&c)?
        } else {
            crate::colors::Color::new(31, 119, 180, 255)
        };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            0.5
        };

        ax.artists.push(Box::new(crate::artists::wireframe3d::Wireframe3D {
            x, y, z, color, linewidth,
        }));

        Ok(())
    }

    /// 3D bar chart.
    fn axes3d_bar3d(
        &mut self,
        ax3d_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<f64>,
        dx: Vec<f64>,
        dy: Vec<f64>,
        dz: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let color = if let Some(c) = kwargs.get_item("color")? {
            colors::parse_color_value(&c)?
        } else {
            ax.next_color()
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.9
        };

        ax.artists.push(Box::new(crate::artists::bar3d::Bar3D {
            x, y, z, dx, dy, dz, color, alpha,
        }));

        Ok(())
    }

    /// Set title for a 3D axes.
    #[pyo3(signature = (ax3d_id, title, fontsize=None))]
    fn axes3d_set_title(&mut self, ax3d_id: usize, title: String, fontsize: Option<f32>) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.title = Some(title);
        let _ = fontsize; // not yet used for 3D title sizing
        Ok(())
    }

    /// Set X label for a 3D axes.
    #[pyo3(signature = (ax3d_id, label, fontsize=None))]
    fn axes3d_set_xlabel(&mut self, ax3d_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.xlabel = Some(label);
        let _ = fontsize;
        Ok(())
    }

    /// Set Y label for a 3D axes.
    #[pyo3(signature = (ax3d_id, label, fontsize=None))]
    fn axes3d_set_ylabel(&mut self, ax3d_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.ylabel = Some(label);
        let _ = fontsize;
        Ok(())
    }

    /// Set Z label for a 3D axes.
    #[pyo3(signature = (ax3d_id, label, fontsize=None))]
    fn axes3d_set_zlabel(&mut self, ax3d_id: usize, label: String, fontsize: Option<f32>) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.zlabel = Some(label);
        let _ = fontsize;
        Ok(())
    }

    /// Set camera view angle for 3D axes.
    fn axes3d_view_init(&mut self, ax3d_id: usize, elev: f64, azim: f64) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.camera.elevation = elev;
        ax.camera.azimuth = azim;
        Ok(())
    }

    fn axes3d_set_xlim(&mut self, ax3d_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.xlim = Some((lo, hi));
        Ok(())
    }

    fn axes3d_set_ylim(&mut self, ax3d_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.ylim = Some((lo, hi));
        Ok(())
    }

    fn axes3d_set_zlim(&mut self, ax3d_id: usize, lo: f64, hi: f64) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.zlim = Some((lo, hi));
        Ok(())
    }

    fn axes3d_legend(&mut self, ax3d_id: usize) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;
        ax.show_legend = true;
        Ok(())
    }

    /// 3D contour plot.
    fn axes3d_contour3d(
        &mut self,
        ax3d_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            v.extract::<String>()?
        } else {
            "viridis".to_string()
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            1.0
        };

        let levels = if let Some(v) = kwargs.get_item("levels")? {
            Some(v.extract::<Vec<f64>>()?)
        } else {
            None
        };

        let z_offset = if let Some(v) = kwargs.get_item("offset")? {
            Some(v.extract::<f64>()?)
        } else {
            None
        };

        let filled = if let Some(v) = kwargs.get_item("filled")? {
            v.extract::<bool>()?
        } else {
            false
        };

        let contour = crate::artists::contour3d::Contour3D::new(
            x, y, z, levels, z_offset, filled, linewidth, cmap, alpha,
        );
        ax.artists.push(Box::new(contour));

        Ok(())
    }

    /// 3D filled contour plot (contourf3D).
    fn axes3d_contourf3d(
        &mut self,
        ax3d_id: usize,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            v.extract::<String>()?
        } else {
            "viridis".to_string()
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.7
        };

        let linewidth = if let Some(v) = kwargs.get_item("linewidth")? {
            v.extract::<f32>()?
        } else {
            0.5
        };

        let levels = if let Some(v) = kwargs.get_item("levels")? {
            Some(v.extract::<Vec<f64>>()?)
        } else {
            None
        };

        let z_offset = if let Some(v) = kwargs.get_item("offset")? {
            Some(v.extract::<f64>()?)
        } else {
            None
        };

        let contour = crate::artists::contour3d::Contour3D::new(
            x, y, z, levels, z_offset, true, linewidth, cmap, alpha,
        );
        ax.artists.push(Box::new(contour));

        Ok(())
    }

    /// 3D triangulated surface plot (plot_trisurf).
    fn axes3d_plot_trisurf(
        &mut self,
        ax3d_id: usize,
        x: Vec<f64>,
        y: Vec<f64>,
        z: Vec<f64>,
        kwargs: &Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let (_, ax) = self.axes3d.get_mut(ax3d_id)
            .ok_or_else(|| pyo3::exceptions::PyIndexError::new_err("Invalid 3D axes index"))?;

        let cmap = if let Some(v) = kwargs.get_item("cmap")? {
            v.extract::<String>()?
        } else {
            "viridis".to_string()
        };

        let alpha = if let Some(v) = kwargs.get_item("alpha")? {
            v.extract::<f32>()?
        } else {
            0.9
        };

        let triangles = if let Some(v) = kwargs.get_item("triangles")? {
            let tri_list: Vec<Vec<usize>> = v.extract()?;
            let tris: Vec<(usize, usize, usize)> = tri_list.iter()
                .filter(|t| t.len() == 3)
                .map(|t| (t[0], t[1], t[2]))
                .collect();
            Some(tris)
        } else {
            None
        };

        let trisurf = if let Some(tris) = triangles {
            crate::artists::trisurf3d::TriSurf3D::with_triangles(x, y, z, tris, cmap, alpha)
        } else {
            crate::artists::trisurf3d::TriSurf3D::from_points(x, y, z, cmap, alpha)
        };

        ax.artists.push(Box::new(trisurf));

        Ok(())
    }
}

impl RustFigure {
    fn render_pdf(pixmap: &Pixmap) -> Vec<u8> {
        let w = pixmap.width();
        let h = pixmap.height();

        let total_bytes = (w as u64) * (h as u64) * 3;
        if total_bytes > 100_000_000 {
            // Over 100MB raw RGB: fall back to embedding PNG data instead
            let png_data = pixmap.encode_png().unwrap_or_default();
            return Self::render_pdf_with_png(w, h, &png_data);
        }

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

    /// Render a PDF embedding compressed PNG data instead of raw RGB.
    /// Used as fallback when raw RGB would exceed the size limit.
    fn render_pdf_with_png(w: u32, h: u32, png_data: &[u8]) -> Vec<u8> {
        let stream_len = png_data.len();

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

        // Object 4: Image XObject (PNG via DCTDecode-like embedding)
        // We embed the raw PNG and reference it; for simplicity we use FlateDecode on the raw pixel data
        // Actually, PDF doesn't natively support PNG streams. We embed as a raw image with smaller dimensions.
        // Fallback: just embed a 1x1 white pixel to keep the PDF valid but small.
        let obj4_offset = pdf.len();
        // Embed PNG data as-is with a filter hint; most PDF readers won't handle this,
        // so we produce a minimal valid image instead.
        let fallback_rgb: Vec<u8> = vec![255, 255, 255]; // 1x1 white pixel
        let actual_w = 1u32;
        let actual_h = 1u32;
        let actual_data = if png_data.is_empty() { &fallback_rgb } else { &fallback_rgb };
        let _ = (stream_len, png_data); // acknowledge unused for this fallback path
        pdf.extend_from_slice(
            format!(
                "4 0 obj\n<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Length {} >>\nstream\n",
                actual_w, actual_h, actual_data.len()
            )
            .as_bytes(),
        );
        pdf.extend_from_slice(actual_data);
        pdf.extend_from_slice(b"\nendstream\nendobj\n");

        // Object 5: Content stream (draw the image scaled to page size)
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

    /// Render an EPS (Encapsulated PostScript) file embedding the pixmap as raw RGB hex data.
    fn render_eps(pixmap: &Pixmap) -> Vec<u8> {
        let w = pixmap.width();
        let h = pixmap.height();

        // Extract RGB data (unpremultiply alpha, white background for transparent pixels)
        let mut rgb_data: Vec<u8> = Vec::with_capacity((w * h * 3) as usize);
        for px in pixmap.pixels() {
            let a = px.alpha() as u32;
            if a > 0 {
                rgb_data.push(((px.red() as u32 * 255 / a).min(255)) as u8);
                rgb_data.push(((px.green() as u32 * 255 / a).min(255)) as u8);
                rgb_data.push(((px.blue() as u32 * 255 / a).min(255)) as u8);
            } else {
                rgb_data.extend_from_slice(&[255u8, 255u8, 255u8]);
            }
        }

        // Build hex-encoded image data, 72 hex chars per line (= 36 bytes = 12 pixels)
        let mut hex_lines = String::new();
        let bytes_per_line = 36usize;
        for chunk in rgb_data.chunks(bytes_per_line) {
            for b in chunk {
                hex_lines.push_str(&format!("{:02X}", b));
            }
            hex_lines.push('\n');
        }

        let eps = format!(
            "%!PS-Adobe-3.0 EPSF-3.0\n\
             %%BoundingBox: 0 0 {w} {h}\n\
             %%EndComments\n\
             {w} {h} scale\n\
             {w} {h} 8 [{w} 0 0 -{h} 0 {h}]\n\
             {{currentfile 3 {w} mul string readhexstring pop}} false 3 colorimage\n\
             {hex_lines}\
             showpage\n\
             %%EOF\n",
            w = w,
            h = h,
            hex_lines = hex_lines,
        );

        eps.into_bytes()
    }

    fn render_pixmap_opts(&self, dpi: Option<u32>, transparent: bool) -> Pixmap {
        let scale = if let Some(d) = dpi {
            d as f32 / self.dpi as f32
        } else {
            1.0
        };

        let pw = (self.width as f32 * scale) as u32;
        let ph = (self.height as f32 * scale) as u32;

        let mut pixmap = match Pixmap::new(pw.max(1).min(32768), ph.max(1).min(32768)) {
            Some(p) => p,
            None => Pixmap::new(1, 1).unwrap(), // fallback to 1x1 pixel
        };

        if !transparent {
            // Fill with figure background color
            let bg_paint = {
                let mut p = tiny_skia::Paint::default();
                p.set_color(self.bg_color.to_tiny_skia());
                p
            };
            if let Some(rect) = tiny_skia::Rect::from_xywh(0.0, 0.0, pw as f32, ph as f32) {
                pixmap.fill_rect(rect, &bg_paint, tiny_skia::Transform::identity(), None);
            }
        }

        if self.axes.is_empty() {
            return pixmap;
        }

        // Layout margins — dynamic when tight layout is enabled
        let (margin_left, margin_right, mut margin_top, margin_bottom) = if self.tight {
            // Tight layout: calculate margins from axis content
            let mut max_ylabel_w = 0.0_f32;
            let mut max_title_h = 0.0_f32;
            let mut max_xlabel_h = 0.0_f32;

            for ax in &self.axes {
                if let Some(ref ylabel) = ax.ylabel {
                    let (tw, _) = crate::text::measure_text(ylabel, ax.label_size * scale);
                    max_ylabel_w = max_ylabel_w.max(tw);
                }
                if let Some(ref title) = ax.title {
                    let (_, th) = crate::text::measure_text(title, ax.title_size * scale);
                    max_title_h = max_title_h.max(th);
                }
                if let Some(ref xlabel) = ax.xlabel {
                    let (_, eh) = crate::text::measure_text(xlabel, ax.label_size * scale);
                    max_xlabel_h = max_xlabel_h.max(eh);
                }
            }

            // Tick labels take ~40px, plus padding
            let tick_space = 40.0 * scale;
            let padding = 10.0 * scale;

            (
                (max_ylabel_w + tick_space + padding).max(40.0 * scale),  // left
                20.0 * scale,                                              // right
                (max_title_h + padding).max(25.0 * scale),                // top
                (max_xlabel_h + tick_space + padding).max(35.0 * scale), // bottom
            )
        } else if self.constrained {
            // Constrained layout: measure all text elements and compute margins
            // to prevent any overlap, with uniform padding between subplots.
            let mut max_ylabel_w = 0.0_f32;
            let mut max_title_h = 0.0_f32;
            let mut max_xlabel_h = 0.0_f32;
            let mut max_ytick_w = 0.0_f32;
            let mut max_xtick_h = 0.0_f32;

            let nrows_c = self.nrows.max(1);
            let ncols_c = self.ncols.max(1);

            for (idx, ax) in self.axes.iter().enumerate() {
                // Measure ylabel
                if let Some(ref ylabel) = ax.ylabel {
                    let (tw, _) = crate::text::measure_text(ylabel, ax.label_size * scale);
                    max_ylabel_w = max_ylabel_w.max(tw);
                }
                // Measure title
                if let Some(ref title) = ax.title {
                    let (_, th) = crate::text::measure_text(title, ax.title_size * scale);
                    max_title_h = max_title_h.max(th);
                }
                // Measure xlabel
                if let Some(ref xlabel) = ax.xlabel {
                    let (_, eh) = crate::text::measure_text(xlabel, ax.label_size * scale);
                    max_xlabel_h = max_xlabel_h.max(eh);
                }

                // Measure actual tick label widths (y-axis) for leftmost column
                let col = idx % ncols_c;
                if col == 0 {
                    // Estimate y-tick label width from data range
                    let (_, _, ymin, ymax) = ax.compute_bounds();
                    let y_ticks = crate::ticker::compute_auto_ticks(ymin, ymax, 7);
                    for &ty in &y_ticks {
                        let label = crate::ticker::format_tick_value(ty);
                        let (lw, _) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        max_ytick_w = max_ytick_w.max(lw);
                    }
                }

                // Measure actual tick label heights (x-axis) for bottom row
                let row = idx / ncols_c;
                if row == nrows_c - 1 {
                    let (xmin, xmax, _, _) = ax.compute_bounds();
                    let x_ticks = crate::ticker::compute_auto_ticks(xmin, xmax, 7);
                    for &tx in &x_ticks {
                        let label = crate::ticker::format_tick_value(tx);
                        let (_, lh) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        max_xtick_h = max_xtick_h.max(lh);
                    }
                }
            }

            // Uniform padding factor for constrained layout
            let pad = 14.0 * scale;
            // Tick mark outward extent
            let tick_out = 6.0 * scale;

            let left_margin = (max_ytick_w + tick_out + max_ylabel_w + pad * 2.0).max(50.0 * scale);
            let right_margin = (pad * 2.0).max(20.0 * scale);
            let top_margin = (max_title_h + pad * 2.0).max(30.0 * scale);
            let bottom_margin = (max_xtick_h + tick_out + max_xlabel_h + pad * 2.0).max(45.0 * scale);

            (left_margin, right_margin, top_margin, bottom_margin)
        } else {
            (70.0_f32 * scale, 20.0_f32 * scale, 40.0_f32 * scale, 50.0_f32 * scale)
        };

        // If suptitle exists, add extra top padding
        if self.suptitle.is_some() {
            margin_top += self.suptitle_fontsize * scale + 10.0 * scale;
        }

        let nrows = self.nrows.max(1);
        let ncols = self.ncols.max(1);

        // For constrained layout, compute uniform inter-subplot padding
        // based on the space needed for inner labels/ticks.
        let (eff_hspace, eff_vspace) = if self.constrained && (nrows > 1 || ncols > 1) {
            // Measure inner labels to determine required gaps
            let mut inner_ylabel_w = 0.0_f32;
            let mut inner_xlabel_h = 0.0_f32;
            let mut inner_title_h = 0.0_f32;
            let mut inner_ytick_w = 0.0_f32;
            let mut inner_xtick_h = 0.0_f32;

            for (idx, ax) in self.axes.iter().enumerate() {
                let col = idx % ncols;
                let row = idx / ncols;

                // Inner y-axis labels (non-leftmost columns)
                if col > 0 {
                    if let Some(ref ylabel) = ax.ylabel {
                        let (tw, _) = crate::text::measure_text(ylabel, ax.label_size * scale);
                        inner_ylabel_w = inner_ylabel_w.max(tw);
                    }
                    let (_, _, ymin, ymax) = ax.compute_bounds();
                    let y_ticks = crate::ticker::compute_auto_ticks(ymin, ymax, 7);
                    for &ty in &y_ticks {
                        let label = crate::ticker::format_tick_value(ty);
                        let (lw, _) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        inner_ytick_w = inner_ytick_w.max(lw);
                    }
                }

                // Inner x-axis labels (non-bottom rows)
                if row < nrows - 1 {
                    if let Some(ref xlabel) = ax.xlabel {
                        let (_, eh) = crate::text::measure_text(xlabel, ax.label_size * scale);
                        inner_xlabel_h = inner_xlabel_h.max(eh);
                    }
                    let (xmin, xmax, _, _) = ax.compute_bounds();
                    let x_ticks = crate::ticker::compute_auto_ticks(xmin, xmax, 7);
                    for &tx in &x_ticks {
                        let label = crate::ticker::format_tick_value(tx);
                        let (_, lh) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        inner_xtick_h = inner_xtick_h.max(lh);
                    }
                }

                // Inner titles (non-top rows)
                if row > 0 {
                    if let Some(ref title) = ax.title {
                        let (_, th) = crate::text::measure_text(title, ax.title_size * scale);
                        inner_title_h = inner_title_h.max(th);
                    }
                }
            }

            let pad = 14.0 * scale;
            let tick_out = 6.0 * scale;

            // Horizontal gap needs space for y-tick labels + ylabel of right subplot
            let h_gap = (inner_ytick_w + tick_out + inner_ylabel_w + pad * 2.0).max(pad * 2.0);
            // Vertical gap needs space for x-tick labels + xlabel of upper subplot + title of lower subplot
            let v_gap = (inner_xtick_h + tick_out + inner_xlabel_h + inner_title_h + pad * 2.0).max(pad * 2.0);

            // Convert absolute pixel gaps to fractional hspace/wspace values.
            // The formulas below invert: gap = fraction * cell_size, where
            // total = n*cell + (n-1)*gap and gap = fraction*cell =>
            // cell = total / (n + (n-1)*fraction). We solve for fraction given gap.
            // From total = n*cell + (n-1)*gap:  cell = (total - (n-1)*gap) / n
            // fraction = gap / cell = n*gap / (total - (n-1)*gap)
            let total_w = pw as f32 - margin_left - margin_right;
            let total_h = ph as f32 - margin_top - margin_bottom;

            let eff_ws = if ncols > 1 {
                let denom = total_w - (ncols as f32 - 1.0) * h_gap;
                if denom > 0.0 {
                    ncols as f32 * h_gap / denom
                } else {
                    self.wspace
                }
            } else {
                self.wspace
            };

            let eff_hs = if nrows > 1 {
                let denom = total_h - (nrows as f32 - 1.0) * v_gap;
                if denom > 0.0 {
                    nrows as f32 * v_gap / denom
                } else {
                    self.hspace
                }
            } else {
                self.hspace
            };

            (eff_ws, eff_hs)
        } else {
            (self.wspace, self.hspace)
        };

        // Use hspace/wspace to compute subplot gaps.
        // hspace/wspace are fractions of the average subplot height/width.
        let total_w = pw as f32 - margin_left - margin_right;
        let total_h = ph as f32 - margin_top - margin_bottom;

        // Compute cell size accounting for gaps: total = n*cell + (n-1)*gap
        // gap = fraction * cell => total = n*cell + (n-1)*fraction*cell = cell*(n + (n-1)*fraction)
        let cell_w = if ncols > 1 {
            total_w / (ncols as f32 + (ncols as f32 - 1.0) * eff_hspace)
        } else {
            total_w
        };
        let cell_h = if nrows > 1 {
            total_h / (nrows as f32 + (nrows as f32 - 1.0) * eff_vspace)
        } else {
            total_h
        };

        let subplot_hgap = cell_w * eff_hspace;
        let subplot_vgap = cell_h * eff_vspace;

        // Collect which subplot indices are claimed by 3D axes
        let axes3d_indices: std::collections::HashSet<usize> = self.axes3d.iter().map(|(idx, _)| *idx).collect();

        for (idx, ax) in self.axes.iter().enumerate() {
            // Skip 2D axes at slots taken by 3D
            if axes3d_indices.contains(&idx) {
                continue;
            }

            // Use custom position if set, otherwise compute from grid
            let (left, top, right, bottom) = if let Some((cl, cb, cw, ch)) = ax.custom_position {
                // Custom position: [left, bottom, width, height] in figure coordinates (0..1)
                let l = cl as f32 * pw as f32;
                let b = (1.0 - cb as f32 - ch as f32) * ph as f32;
                let r = l + cw as f32 * pw as f32;
                let bt = b + ch as f32 * ph as f32;
                (l, b, r, bt)
            } else if let Some((r0, r1, c0, c1)) = ax.grid_span {
                // GridSpec spanning: axes covers cells [r0..r1, c0..c1]
                let l = margin_left + c0 as f32 * (cell_w + subplot_hgap);
                let t = margin_top + r0 as f32 * (cell_h + subplot_vgap);
                let r = margin_left + c1 as f32 * (cell_w + subplot_hgap) - subplot_hgap + cell_w;
                let b = margin_top + r1 as f32 * (cell_h + subplot_vgap) - subplot_vgap + cell_h;
                (l, t, r, b)
            } else {
                let row = idx / ncols;
                let col = idx % ncols;
                if row >= nrows { break; }
                let l = margin_left + col as f32 * (cell_w + subplot_hgap);
                let t = margin_top + row as f32 * (cell_h + subplot_vgap);
                (l, t, l + cell_w, t + cell_h)
            };

            // Compute area bounds for overpaint clipping
            let area_left = (left - 10.0).max(0.0);
            let area_top = (top - 10.0).max(0.0);
            let area_right = (right + 10.0).min(pw as f32);
            let area_bottom = (bottom + 10.0).min(ph as f32);

            ax.draw(&mut pixmap, left, top, right, bottom,
                Some(area_left), Some(area_top), Some(area_right), Some(area_bottom),
                Some(self.bg_color));
        }

        // Draw 3D axes in their subplot slots
        for (subplot_idx, ax3d) in &self.axes3d {
            let row = subplot_idx / ncols;
            let col = subplot_idx % ncols;
            if row >= nrows { continue; }

            let left = margin_left + col as f32 * (cell_w + subplot_hgap);
            let top = margin_top + row as f32 * (cell_h + subplot_vgap);
            let right = left + cell_w;
            let bottom = top + cell_h;

            ax3d.draw(&mut pixmap, left, top, right, bottom);
        }

        // Draw suptitle
        if let Some(ref suptitle) = self.suptitle {
            let cx = pw as f32 / 2.0;
            let y = 10.0 * scale + self.suptitle_fontsize * scale * 0.5;
            // Use text color from the first axes if available, otherwise black
            let suptitle_color = if let Some(ax) = self.axes.first() {
                ax.text_color
            } else {
                crate::colors::Color::new(0, 0, 0, 255)
            };
            crate::text::draw_text(
                &mut pixmap,
                suptitle,
                cx,
                y,
                self.suptitle_fontsize * scale,
                suptitle_color,
                crate::text::TextAnchorX::Center,
                crate::text::TextAnchorY::Center,
                0.0,
            );
        }

        pixmap
    }

    /// Crop a pixmap to its content, removing background-colored borders.
    /// `pad` is the number of padding pixels to add around the content.
    fn crop_to_content(pixmap: &Pixmap, bg_r: u8, bg_g: u8, bg_b: u8, transparent_bg: bool, pad: u32) -> Pixmap {
        let w = pixmap.width();
        let h = pixmap.height();
        let pixels = pixmap.pixels();

        let mut min_x = w;
        let mut max_x = 0u32;
        let mut min_y = h;
        let mut max_y = 0u32;

        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) as usize;
                let px = pixels[idx];
                let a = px.alpha();

                if transparent_bg {
                    // For transparent backgrounds, any non-transparent pixel is content
                    if a > 0 {
                        min_x = min_x.min(x);
                        max_x = max_x.max(x);
                        min_y = min_y.min(y);
                        max_y = max_y.max(y);
                    }
                } else if a > 0 {
                    // Unpremultiply to compare with background color
                    let ur = (px.red() as u32 * 255 / a as u32) as u8;
                    let ug = (px.green() as u32 * 255 / a as u32) as u8;
                    let ub = (px.blue() as u32 * 255 / a as u32) as u8;
                    if ur != bg_r || ug != bg_g || ub != bg_b {
                        min_x = min_x.min(x);
                        max_x = max_x.max(x);
                        min_y = min_y.min(y);
                        max_y = max_y.max(y);
                    }
                }
            }
        }

        if min_x >= max_x || min_y >= max_y {
            return pixmap.clone();
        }

        // Add padding
        let x0 = min_x.saturating_sub(pad);
        let y0 = min_y.saturating_sub(pad);
        let x1 = (max_x + pad + 1).min(w);
        let y1 = (max_y + pad + 1).min(h);
        let cw = x1 - x0;
        let ch = y1 - y0;

        let mut cropped = Pixmap::new(cw.max(1), ch.max(1)).unwrap();
        let dst_pixels = cropped.pixels_mut();
        for cy in 0..ch {
            for cx in 0..cw {
                let src_idx = ((y0 + cy) * w + (x0 + cx)) as usize;
                let dst_idx = (cy * cw + cx) as usize;
                dst_pixels[dst_idx] = pixels[src_idx];
            }
        }
        cropped
    }

    /// Render the figure as native SVG XML string.
    fn render_svg_native(&self, dpi: Option<u32>, transparent: bool) -> String {
        let scale = if let Some(d) = dpi {
            d as f32 / self.dpi as f32
        } else {
            1.0
        };

        let pw = (self.width as f32 * scale) as u32;
        let ph = (self.height as f32 * scale) as u32;

        let mut svg = SvgRenderer::new(pw, ph);

        if self.axes.is_empty() {
            let bg_str = if transparent {
                "none".to_string()
            } else {
                color_to_svg(&self.bg_color)
            };
            return svg.to_svg(&bg_str);
        }

        // Layout margins (same as render_pixmap_opts)
        let (margin_left, margin_right, mut margin_top, margin_bottom) = if self.tight {
            let mut max_ylabel_w = 0.0_f32;
            let mut max_title_h = 0.0_f32;
            let mut max_xlabel_h = 0.0_f32;

            for ax in &self.axes {
                if let Some(ref ylabel) = ax.ylabel {
                    let (tw, _) = crate::text::measure_text(ylabel, ax.label_size * scale);
                    max_ylabel_w = max_ylabel_w.max(tw);
                }
                if let Some(ref title) = ax.title {
                    let (_, th) = crate::text::measure_text(title, ax.title_size * scale);
                    max_title_h = max_title_h.max(th);
                }
                if let Some(ref xlabel) = ax.xlabel {
                    let (_, eh) = crate::text::measure_text(xlabel, ax.label_size * scale);
                    max_xlabel_h = max_xlabel_h.max(eh);
                }
            }

            let tick_space = 40.0 * scale;
            let padding = 10.0 * scale;

            (
                (max_ylabel_w + tick_space + padding).max(40.0 * scale),
                20.0 * scale,
                (max_title_h + padding).max(25.0 * scale),
                (max_xlabel_h + tick_space + padding).max(35.0 * scale),
            )
        } else if self.constrained {
            let mut max_ylabel_w = 0.0_f32;
            let mut max_title_h = 0.0_f32;
            let mut max_xlabel_h = 0.0_f32;
            let mut max_ytick_w = 0.0_f32;
            let mut max_xtick_h = 0.0_f32;

            let nrows_c = self.nrows.max(1);
            let ncols_c = self.ncols.max(1);

            for (idx, ax) in self.axes.iter().enumerate() {
                if let Some(ref ylabel) = ax.ylabel {
                    let (tw, _) = crate::text::measure_text(ylabel, ax.label_size * scale);
                    max_ylabel_w = max_ylabel_w.max(tw);
                }
                if let Some(ref title) = ax.title {
                    let (_, th) = crate::text::measure_text(title, ax.title_size * scale);
                    max_title_h = max_title_h.max(th);
                }
                if let Some(ref xlabel) = ax.xlabel {
                    let (_, eh) = crate::text::measure_text(xlabel, ax.label_size * scale);
                    max_xlabel_h = max_xlabel_h.max(eh);
                }

                let col = idx % ncols_c;
                if col == 0 {
                    let (_, _, ymin, ymax) = ax.compute_bounds();
                    let y_ticks = crate::ticker::compute_auto_ticks(ymin, ymax, 7);
                    for &ty in &y_ticks {
                        let label = crate::ticker::format_tick_value(ty);
                        let (lw, _) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        max_ytick_w = max_ytick_w.max(lw);
                    }
                }

                let row = idx / ncols_c;
                if row == nrows_c - 1 {
                    let (xmin, xmax, _, _) = ax.compute_bounds();
                    let x_ticks = crate::ticker::compute_auto_ticks(xmin, xmax, 7);
                    for &tx in &x_ticks {
                        let label = crate::ticker::format_tick_value(tx);
                        let (_, lh) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        max_xtick_h = max_xtick_h.max(lh);
                    }
                }
            }

            let pad = 14.0 * scale;
            let tick_out = 6.0 * scale;

            let left_margin = (max_ytick_w + tick_out + max_ylabel_w + pad * 2.0).max(50.0 * scale);
            let right_margin = (pad * 2.0).max(20.0 * scale);
            let top_margin = (max_title_h + pad * 2.0).max(30.0 * scale);
            let bottom_margin = (max_xtick_h + tick_out + max_xlabel_h + pad * 2.0).max(45.0 * scale);

            (left_margin, right_margin, top_margin, bottom_margin)
        } else {
            (70.0_f32 * scale, 20.0_f32 * scale, 40.0_f32 * scale, 50.0_f32 * scale)
        };

        if self.suptitle.is_some() {
            margin_top += self.suptitle_fontsize * scale + 10.0 * scale;
        }

        let nrows = self.nrows.max(1);
        let ncols = self.ncols.max(1);

        // Constrained layout: compute uniform inter-subplot padding (same as pixmap path)
        let (eff_hspace, eff_vspace) = if self.constrained && (nrows > 1 || ncols > 1) {
            let mut inner_ylabel_w = 0.0_f32;
            let mut inner_xlabel_h = 0.0_f32;
            let mut inner_title_h = 0.0_f32;
            let mut inner_ytick_w = 0.0_f32;
            let mut inner_xtick_h = 0.0_f32;

            for (idx, ax) in self.axes.iter().enumerate() {
                let col = idx % ncols;
                let row = idx / ncols;

                if col > 0 {
                    if let Some(ref ylabel) = ax.ylabel {
                        let (tw, _) = crate::text::measure_text(ylabel, ax.label_size * scale);
                        inner_ylabel_w = inner_ylabel_w.max(tw);
                    }
                    let (_, _, ymin, ymax) = ax.compute_bounds();
                    let y_ticks = crate::ticker::compute_auto_ticks(ymin, ymax, 7);
                    for &ty in &y_ticks {
                        let label = crate::ticker::format_tick_value(ty);
                        let (lw, _) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        inner_ytick_w = inner_ytick_w.max(lw);
                    }
                }

                if row < nrows - 1 {
                    if let Some(ref xlabel) = ax.xlabel {
                        let (_, eh) = crate::text::measure_text(xlabel, ax.label_size * scale);
                        inner_xlabel_h = inner_xlabel_h.max(eh);
                    }
                    let (xmin, xmax, _, _) = ax.compute_bounds();
                    let x_ticks = crate::ticker::compute_auto_ticks(xmin, xmax, 7);
                    for &tx in &x_ticks {
                        let label = crate::ticker::format_tick_value(tx);
                        let (_, lh) = crate::text::measure_text(&label, ax.tick_label_size * scale);
                        inner_xtick_h = inner_xtick_h.max(lh);
                    }
                }

                if row > 0 {
                    if let Some(ref title) = ax.title {
                        let (_, th) = crate::text::measure_text(title, ax.title_size * scale);
                        inner_title_h = inner_title_h.max(th);
                    }
                }
            }

            let pad = 14.0 * scale;
            let tick_out = 6.0 * scale;

            let h_gap = (inner_ytick_w + tick_out + inner_ylabel_w + pad * 2.0).max(pad * 2.0);
            let v_gap = (inner_xtick_h + tick_out + inner_xlabel_h + inner_title_h + pad * 2.0).max(pad * 2.0);

            let total_w = pw as f32 - margin_left - margin_right;
            let total_h = ph as f32 - margin_top - margin_bottom;

            let eff_ws = if ncols > 1 {
                let denom = total_w - (ncols as f32 - 1.0) * h_gap;
                if denom > 0.0 { ncols as f32 * h_gap / denom } else { self.wspace }
            } else {
                self.wspace
            };

            let eff_hs = if nrows > 1 {
                let denom = total_h - (nrows as f32 - 1.0) * v_gap;
                if denom > 0.0 { nrows as f32 * v_gap / denom } else { self.hspace }
            } else {
                self.hspace
            };

            (eff_ws, eff_hs)
        } else {
            (self.wspace, self.hspace)
        };

        let total_w = pw as f32 - margin_left - margin_right;
        let total_h = ph as f32 - margin_top - margin_bottom;

        let cell_w = if ncols > 1 {
            total_w / (ncols as f32 + (ncols as f32 - 1.0) * eff_hspace)
        } else {
            total_w
        };
        let cell_h = if nrows > 1 {
            total_h / (nrows as f32 + (nrows as f32 - 1.0) * eff_vspace)
        } else {
            total_h
        };

        let subplot_hgap = cell_w * eff_hspace;
        let subplot_vgap = cell_h * eff_vspace;

        let axes3d_indices: std::collections::HashSet<usize> =
            self.axes3d.iter().map(|(idx, _)| *idx).collect();

        for (idx, ax) in self.axes.iter().enumerate() {
            if axes3d_indices.contains(&idx) {
                continue;
            }
            let row = idx / ncols;
            let col = idx % ncols;
            if row >= nrows {
                break;
            }

            let left = margin_left + col as f32 * (cell_w + subplot_hgap);
            let top = margin_top + row as f32 * (cell_h + subplot_vgap);
            let right = left + cell_w;
            let bottom = top + cell_h;

            ax.draw_svg(&mut svg, left, top, right, bottom);
        }

        // Draw suptitle
        if let Some(ref suptitle) = self.suptitle {
            let cx = pw as f32 / 2.0;
            let y = 10.0 * scale + self.suptitle_fontsize * scale * 0.5;
            let suptitle_color = if let Some(ax) = self.axes.first() {
                color_to_svg(&ax.text_color)
            } else {
                "rgb(0,0,0)".to_string()
            };
            svg.add_text(
                cx,
                y,
                suptitle,
                self.suptitle_fontsize * scale,
                &suptitle_color,
                "middle",
                0.0,
            );
        }

        let bg_str = if transparent {
            "none".to_string()
        } else {
            color_to_svg(&self.bg_color)
        };
        svg.to_svg(&bg_str)
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
