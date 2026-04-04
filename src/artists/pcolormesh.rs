use crate::artists::Artist;
use crate::artists::image::colormap_lookup;
use crate::colors::Color;
use crate::svg_renderer::{SvgRenderer, color_to_svg};
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

/// Pseudocolor plot on an irregular grid.
/// x, y are vertex coordinates (nrows+1 x ncols+1).
/// c holds cell values (nrows x ncols).
pub struct PColorMesh {
    pub x: Vec<Vec<f64>>,
    pub y: Vec<Vec<f64>>,
    pub c: Vec<Vec<f64>>,
    pub cmap: String,
    pub vmin: f64,
    pub vmax: f64,
    pub alpha: f32,
    pub edgecolors: Option<Color>,
    pub label: Option<String>,
    pub zorder: i32,
}

impl PColorMesh {
    pub fn new(x: Vec<Vec<f64>>, y: Vec<Vec<f64>>, c: Vec<Vec<f64>>, cmap: String) -> Self {
        // Compute vmin/vmax from c
        let mut vmin = f64::MAX;
        let mut vmax = f64::MIN;
        for row in &c {
            for &v in row {
                if v < vmin { vmin = v; }
                if v > vmax { vmax = v; }
            }
        }
        if vmin >= vmax {
            vmin = 0.0;
            vmax = 1.0;
        }
        Self {
            x, y, c,
            cmap,
            vmin, vmax,
            alpha: 1.0,
            edgecolors: None,
            label: None,
            zorder: 0,
        }
    }

    /// Build a PColorMesh from just data (auto-generate regular grid coordinates).
    pub fn from_data(c: Vec<Vec<f64>>, cmap: String) -> Self {
        let nrows = c.len();
        let ncols = if nrows > 0 { c[0].len() } else { 0 };
        // Generate vertex coords: x from 0..ncols, y from 0..nrows
        let mut x = Vec::with_capacity(nrows + 1);
        let mut y = Vec::with_capacity(nrows + 1);
        for r in 0..=nrows {
            let mut xrow = Vec::with_capacity(ncols + 1);
            let mut yrow = Vec::with_capacity(ncols + 1);
            for col in 0..=ncols {
                xrow.push(col as f64);
                yrow.push(r as f64);
            }
            x.push(xrow);
            y.push(yrow);
        }
        Self::new(x, y, c, cmap)
    }
}

impl Artist for PColorMesh {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let nrows = self.c.len();
        if nrows == 0 { return; }
        let ncols = self.c[0].len();
        if ncols == 0 { return; }

        let ts = tiny_skia::Transform::identity();
        let range = self.vmax - self.vmin;

        for r in 0..nrows {
            for col in 0..ncols {
                if r + 1 >= self.x.len() || col + 1 >= self.x[r].len() { continue; }
                if r + 1 >= self.y.len() || col + 1 >= self.y[r].len() { continue; }

                let t = if range.abs() < 1e-15 { 0.5 } else {
                    ((self.c[r][col] - self.vmin) / range).clamp(0.0, 1.0)
                };
                let mut color = colormap_lookup(&self.cmap, t);
                color.a = (self.alpha * 255.0) as u8;

                let mut paint = Paint::default();
                paint.set_color(color.to_tiny_skia());
                paint.anti_alias = true;

                // Quad vertices: top-left, top-right, bottom-right, bottom-left
                let (px0, py0) = transform.transform_xy(self.x[r][col], self.y[r][col]);
                let (px1, py1) = transform.transform_xy(self.x[r][col + 1], self.y[r][col + 1]);
                let (px2, py2) = transform.transform_xy(self.x[r + 1][col + 1], self.y[r + 1][col + 1]);
                let (px3, py3) = transform.transform_xy(self.x[r + 1][col], self.y[r + 1][col]);

                let mut pb = PathBuilder::new();
                pb.move_to(px0, py0);
                pb.line_to(px1, py1);
                pb.line_to(px2, py2);
                pb.line_to(px3, py3);
                pb.close();

                if let Some(path) = pb.finish() {
                    pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);

                    // Draw edge if requested
                    if let Some(edge_color) = self.edgecolors {
                        let mut edge_paint = Paint::default();
                        let mut ec = edge_color;
                        ec.a = (self.alpha * 255.0) as u8;
                        edge_paint.set_color(ec.to_tiny_skia());
                        edge_paint.anti_alias = true;
                        let mut stroke = Stroke::default();
                        stroke.width = 0.5;
                        pixmap.stroke_path(&path, &edge_paint, &stroke, ts, None);
                    }
                }
            }
        }
    }

    fn draw_svg(&self, svg: &mut SvgRenderer, transform: &Transform) {
        let nrows = self.c.len();
        if nrows == 0 { return; }
        let ncols = self.c[0].len();
        if ncols == 0 { return; }

        let range = self.vmax - self.vmin;

        for r in 0..nrows {
            for col in 0..ncols {
                if r + 1 >= self.x.len() || col + 1 >= self.x[r].len() { continue; }
                if r + 1 >= self.y.len() || col + 1 >= self.y[r].len() { continue; }

                let t = if range.abs() < 1e-15 { 0.5 } else {
                    ((self.c[r][col] - self.vmin) / range).clamp(0.0, 1.0)
                };
                let mut color = colormap_lookup(&self.cmap, t);
                color.a = (self.alpha * 255.0) as u8;
                let fill_str = color_to_svg(&color);

                let (px0, py0) = transform.transform_xy(self.x[r][col], self.y[r][col]);
                let (px1, py1) = transform.transform_xy(self.x[r][col + 1], self.y[r][col + 1]);
                let (px2, py2) = transform.transform_xy(self.x[r + 1][col + 1], self.y[r + 1][col + 1]);
                let (px3, py3) = transform.transform_xy(self.x[r + 1][col], self.y[r + 1][col]);

                let points = vec![(px0, py0), (px1, py1), (px2, py2), (px3, py3)];
                let stroke_str = if self.edgecolors.is_some() { "black" } else { "none" };
                svg.add_polygon(&points, &fill_str, stroke_str, 0.5, self.alpha);
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for row in &self.x {
            for &v in row {
                if v < xmin { xmin = v; }
                if v > xmax { xmax = v; }
            }
        }
        for row in &self.y {
            for &v in row {
                if v < ymin { ymin = v; }
                if v > ymax { ymax = v; }
            }
        }
        if xmin >= xmax { xmin = 0.0; xmax = 1.0; }
        if ymin >= ymax { ymin = 0.0; ymax = 1.0; }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        Color::new(100, 100, 100, 255)
    }

    fn zorder(&self) -> i32 {
        self.zorder
    }
}
