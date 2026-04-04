use crate::artists::Artist;
use crate::colors::Color;
use crate::svg_renderer::{SvgRenderer, color_to_svg};
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap};

/// A filled polygon defined by x/y vertex coordinates.
pub struct FillPolygon {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Color,
    pub alpha: f32,
    pub label: Option<String>,
    pub zorder: i32,
}

impl FillPolygon {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color, alpha: f32) -> Self {
        Self {
            x,
            y,
            color,
            alpha,
            label: None,
            zorder: 0,
        }
    }
}

impl Artist for FillPolygon {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.len() < 3 || self.y.len() < 3 {
            return;
        }
        let n = self.x.len().min(self.y.len());

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;

        let mut paint = Paint::default();
        paint.set_color(fill_color.to_tiny_skia());
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        let mut pb = PathBuilder::new();
        let (px, py) = transform.transform_xy(self.x[0], self.y[0]);
        pb.move_to(px, py);
        for i in 1..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
            pb.line_to(px, py);
        }
        pb.close();

        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
        }
    }

    fn draw_svg(&self, svg: &mut SvgRenderer, transform: &Transform) {
        if self.x.len() < 3 || self.y.len() < 3 {
            return;
        }
        let n = self.x.len().min(self.y.len());

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;
        let fill_str = color_to_svg(&fill_color);

        let mut points: Vec<(f32, f32)> = Vec::with_capacity(n);
        for i in 0..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
            points.push((px, py));
        }

        svg.add_polygon(&points, &fill_str, "none", 0.0, self.alpha);
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.x.is_empty() || self.y.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let n = self.x.len().min(self.y.len());
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;
        for i in 0..n {
            if self.x[i] < xmin { xmin = self.x[i]; }
            if self.x[i] > xmax { xmax = self.x[i]; }
            if self.y[i] < ymin { ymin = self.y[i]; }
            if self.y[i] > ymax { ymax = self.y[i]; }
        }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }

    fn zorder(&self) -> i32 {
        self.zorder
    }
}
