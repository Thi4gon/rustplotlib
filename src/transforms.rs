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

/// A 2D affine transformation matrix:
///   | a  b  tx |
///   | c  d  ty |
///   | 0  0  1  |
#[pyclass]
#[derive(Debug, Clone)]
pub struct Affine2D {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}

#[pymethods]
impl Affine2D {
    #[new]
    pub fn new() -> Self {
        // Identity matrix
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 }
    }

    /// Create a translation transform.
    #[staticmethod]
    pub fn translate(tx: f64, ty: f64) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx, ty }
    }

    /// Create a scaling transform.
    #[staticmethod]
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0 }
    }

    /// Create a rotation transform (angle in radians).
    #[staticmethod]
    pub fn rotate(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self { a: cos, b: -sin, c: sin, d: cos, tx: 0.0, ty: 0.0 }
    }

    /// Create a rotation transform (angle in degrees).
    #[staticmethod]
    pub fn rotate_deg(angle: f64) -> Self {
        Self::rotate(angle.to_radians())
    }

    /// Transform a point (x, y) → (x', y').
    pub fn transform_point(&self, x: f64, y: f64) -> (f64, f64) {
        (self.a * x + self.b * y + self.tx,
         self.c * x + self.d * y + self.ty)
    }

    /// Transform an array of points.
    pub fn transform_points(&self, points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        points.iter().map(|(x, y)| self.transform_point(*x, *y)).collect()
    }

    /// Compose with another Affine2D: self * other.
    pub fn compose(&self, other: &Affine2D) -> Affine2D {
        Affine2D {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            tx: self.a * other.tx + self.b * other.ty + self.tx,
            ty: self.c * other.tx + self.d * other.ty + self.ty,
        }
    }

    /// Return the inverse transform.
    pub fn inverted(&self) -> Affine2D {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-15 {
            return Self::new(); // Return identity if singular
        }
        let inv_det = 1.0 / det;
        Affine2D {
            a: self.d * inv_det,
            b: -self.b * inv_det,
            c: -self.c * inv_det,
            d: self.a * inv_det,
            tx: (self.b * self.ty - self.d * self.tx) * inv_det,
            ty: (self.c * self.tx - self.a * self.ty) * inv_det,
        }
    }

    /// Check if this is the identity transform.
    pub fn is_identity(&self) -> bool {
        (self.a - 1.0).abs() < 1e-15
            && self.b.abs() < 1e-15
            && self.c.abs() < 1e-15
            && (self.d - 1.0).abs() < 1e-15
            && self.tx.abs() < 1e-15
            && self.ty.abs() < 1e-15
    }

    /// Get the matrix as a flat array [a, b, tx, c, d, ty].
    pub fn get_matrix(&self) -> Vec<f64> {
        vec![self.a, self.b, self.tx, self.c, self.d, self.ty]
    }
}

/// A blended transform that uses one transform for X and another for Y.
#[pyclass]
#[derive(Debug, Clone)]
pub struct BlendedTransform {
    pub x_transform: Affine2D,
    pub y_transform: Affine2D,
}

#[pymethods]
impl BlendedTransform {
    #[new]
    pub fn new(x_transform: Affine2D, y_transform: Affine2D) -> Self {
        Self { x_transform, y_transform }
    }

    /// Transform a point using x_transform for X and y_transform for Y.
    pub fn transform_point(&self, x: f64, y: f64) -> (f64, f64) {
        let (tx, _) = self.x_transform.transform_point(x, 0.0);
        let (_, ty) = self.y_transform.transform_point(0.0, y);
        (tx, ty)
    }

    /// Transform batch of points.
    pub fn transform_points(&self, points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        points.iter().map(|(x, y)| self.transform_point(*x, *y)).collect()
    }

    /// Return inverse blended transform.
    pub fn inverted(&self) -> BlendedTransform {
        BlendedTransform {
            x_transform: self.x_transform.inverted(),
            y_transform: self.y_transform.inverted(),
        }
    }
}
