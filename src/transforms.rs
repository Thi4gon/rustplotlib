use pyo3::prelude::*;

/// Transforms data coordinates to pixel coordinates.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Transform {
    pub data_xmin: f64,
    pub data_xmax: f64,
    pub data_ymin: f64,
    pub data_ymax: f64,
    pub pixel_left: f64,
    pub pixel_right: f64,
    pub pixel_top: f64,
    pub pixel_bottom: f64,
    pub log_x: bool,
    pub log_y: bool,
}

#[pymethods]
impl Transform {
    #[new]
    #[pyo3(signature = (data_xlim, data_ylim, pixel_left, pixel_right, pixel_top, pixel_bottom, log_x=false, log_y=false))]
    pub fn new(
        data_xlim: (f64, f64),
        data_ylim: (f64, f64),
        pixel_left: f64,
        pixel_right: f64,
        pixel_top: f64,
        pixel_bottom: f64,
        log_x: bool,
        log_y: bool,
    ) -> Self {
        Transform {
            data_xmin: data_xlim.0,
            data_xmax: data_xlim.1,
            data_ymin: data_ylim.0,
            data_ymax: data_ylim.1,
            pixel_left,
            pixel_right,
            pixel_top,
            pixel_bottom,
            log_x,
            log_y,
        }
    }

    /// Map data coordinates (x, y) to pixel coordinates.
    /// Y is inverted: data-y increases upward, pixel-y increases downward.
    /// If log_x or log_y is true, data coords are transformed through log10 first.
    pub fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        let x = if self.log_x { x.max(1e-15).log10() } else { x };
        let y = if self.log_y { y.max(1e-15).log10() } else { y };

        let dx = self.data_xmax - self.data_xmin;
        let dy = self.data_ymax - self.data_ymin;

        let px = if dx.abs() < 1e-15 {
            (self.pixel_left + self.pixel_right) / 2.0
        } else {
            self.pixel_left + (x - self.data_xmin) / dx * (self.pixel_right - self.pixel_left)
        };

        // Y is inverted: data_ymin maps to pixel_bottom, data_ymax maps to pixel_top
        let py = if dy.abs() < 1e-15 {
            (self.pixel_top + self.pixel_bottom) / 2.0
        } else {
            self.pixel_bottom - (y - self.data_ymin) / dy * (self.pixel_bottom - self.pixel_top)
        };

        (px, py)
    }

    /// Batch-transform arrays of data coordinates to pixel coordinates.
    pub fn transform_batch(&self, xs: Vec<f64>, ys: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        let mut pxs = Vec::with_capacity(xs.len());
        let mut pys = Vec::with_capacity(ys.len());
        for (x, y) in xs.iter().zip(ys.iter()) {
            let (px, py) = self.transform(*x, *y);
            pxs.push(px);
            pys.push(py);
        }
        (pxs, pys)
    }
}

impl Transform {
    /// Create a linear (non-log) transform — convenience for Rust-side code.
    pub fn new_linear(
        data_xlim: (f64, f64),
        data_ylim: (f64, f64),
        pixel_left: f64,
        pixel_right: f64,
        pixel_top: f64,
        pixel_bottom: f64,
    ) -> Self {
        Self::new(data_xlim, data_ylim, pixel_left, pixel_right, pixel_top, pixel_bottom, false, false)
    }

    /// Internal transform returning f32 (for use in Rust drawing code).
    pub fn transform_xy(&self, x: f64, y: f64) -> (f32, f32) {
        let (px, py) = self.transform(x, y);
        (px as f32, py as f32)
    }
}
