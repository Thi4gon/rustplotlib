use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

pub enum PatchShape {
    Rectangle { x: f64, y: f64, width: f64, height: f64 },
    Circle { center: (f64, f64), radius: f64 },
    Polygon { points: Vec<(f64, f64)> },
}

pub struct Patch {
    pub shape: PatchShape,
    pub facecolor: Option<Color>,
    pub edgecolor: Color,
    pub linewidth: f32,
    pub alpha: f32,
    pub label: Option<String>,
}

impl Patch {
    pub fn new_rectangle(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        facecolor: Option<Color>,
        edgecolor: Color,
        linewidth: f32,
        alpha: f32,
        label: Option<String>,
    ) -> Self {
        Patch {
            shape: PatchShape::Rectangle { x, y, width, height },
            facecolor,
            edgecolor,
            linewidth,
            alpha,
            label,
        }
    }

    pub fn new_circle(
        center: (f64, f64),
        radius: f64,
        facecolor: Option<Color>,
        edgecolor: Color,
        linewidth: f32,
        alpha: f32,
        label: Option<String>,
    ) -> Self {
        Patch {
            shape: PatchShape::Circle { center, radius },
            facecolor,
            edgecolor,
            linewidth,
            alpha,
            label,
        }
    }

    pub fn new_polygon(
        points: Vec<(f64, f64)>,
        facecolor: Option<Color>,
        edgecolor: Color,
        linewidth: f32,
        alpha: f32,
        label: Option<String>,
    ) -> Self {
        Patch {
            shape: PatchShape::Polygon { points },
            facecolor,
            edgecolor,
            linewidth,
            alpha,
            label,
        }
    }
}

impl Artist for Patch {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let ts = tiny_skia::Transform::identity();

        match &self.shape {
            PatchShape::Rectangle { x, y, width, height } => {
                // Rectangle corners in data space
                let (p1x, p1y) = transform.transform_xy(*x, *y);
                let (p2x, p2y) = transform.transform_xy(*x + *width, *y + *height);

                let rx = p1x.min(p2x);
                let ry = p1y.min(p2y);
                let rw = (p2x - p1x).abs();
                let rh = (p2y - p1y).abs();

                if rw <= 0.0 || rh <= 0.0 {
                    return;
                }

                let mut pb = PathBuilder::new();
                pb.move_to(rx, ry);
                pb.line_to(rx + rw, ry);
                pb.line_to(rx + rw, ry + rh);
                pb.line_to(rx, ry + rh);
                pb.close();

                if let Some(path) = pb.finish() {
                    // Fill
                    if let Some(fc) = self.facecolor {
                        let mut fill_color = fc;
                        fill_color.a = (self.alpha * 255.0) as u8;
                        let mut paint = Paint::default();
                        paint.set_color(fill_color.to_tiny_skia());
                        paint.anti_alias = true;
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
                    }

                    // Stroke
                    if self.linewidth > 0.0 {
                        let mut ec = self.edgecolor;
                        ec.a = (self.alpha * 255.0) as u8;
                        let mut paint = Paint::default();
                        paint.set_color(ec.to_tiny_skia());
                        paint.anti_alias = true;
                        let mut stroke = Stroke::default();
                        stroke.width = self.linewidth;
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                }
            }
            PatchShape::Circle { center, radius } => {
                // Approximate circle with polygon points in data space
                let n_segments = 64;
                let mut pb = PathBuilder::new();

                for i in 0..n_segments {
                    let angle = 2.0 * std::f64::consts::PI * i as f64 / n_segments as f64;
                    let dx = center.0 + radius * angle.cos();
                    let dy = center.1 + radius * angle.sin();
                    let (px, py) = transform.transform_xy(dx, dy);
                    if i == 0 {
                        pb.move_to(px, py);
                    } else {
                        pb.line_to(px, py);
                    }
                }
                pb.close();

                if let Some(path) = pb.finish() {
                    if let Some(fc) = self.facecolor {
                        let mut fill_color = fc;
                        fill_color.a = (self.alpha * 255.0) as u8;
                        let mut paint = Paint::default();
                        paint.set_color(fill_color.to_tiny_skia());
                        paint.anti_alias = true;
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
                    }

                    if self.linewidth > 0.0 {
                        let mut ec = self.edgecolor;
                        ec.a = (self.alpha * 255.0) as u8;
                        let mut paint = Paint::default();
                        paint.set_color(ec.to_tiny_skia());
                        paint.anti_alias = true;
                        let mut stroke = Stroke::default();
                        stroke.width = self.linewidth;
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                }
            }
            PatchShape::Polygon { points } => {
                if points.is_empty() { return; }

                let mut pb = PathBuilder::new();
                for (i, &(dx, dy)) in points.iter().enumerate() {
                    let (px, py) = transform.transform_xy(dx, dy);
                    if i == 0 {
                        pb.move_to(px, py);
                    } else {
                        pb.line_to(px, py);
                    }
                }
                pb.close();

                if let Some(path) = pb.finish() {
                    if let Some(fc) = self.facecolor {
                        let mut fill_color = fc;
                        fill_color.a = (self.alpha * 255.0) as u8;
                        let mut paint = Paint::default();
                        paint.set_color(fill_color.to_tiny_skia());
                        paint.anti_alias = true;
                        pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
                    }

                    if self.linewidth > 0.0 {
                        let mut ec = self.edgecolor;
                        ec.a = (self.alpha * 255.0) as u8;
                        let mut paint = Paint::default();
                        paint.set_color(ec.to_tiny_skia());
                        paint.anti_alias = true;
                        let mut stroke = Stroke::default();
                        stroke.width = self.linewidth;
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        match &self.shape {
            PatchShape::Rectangle { x, y, width, height } => {
                (*x, *x + *width, *y, *y + *height)
            }
            PatchShape::Circle { center, radius } => {
                (center.0 - radius, center.0 + radius, center.1 - radius, center.1 + radius)
            }
            PatchShape::Polygon { points } => {
                if points.is_empty() {
                    return (0.0, 1.0, 0.0, 1.0);
                }
                let mut xmin = f64::MAX;
                let mut xmax = f64::MIN;
                let mut ymin = f64::MAX;
                let mut ymax = f64::MIN;
                for &(x, y) in points {
                    if x < xmin { xmin = x; }
                    if x > xmax { xmax = x; }
                    if y < ymin { ymin = y; }
                    if y > ymax { ymax = y; }
                }
                (xmin, xmax, ymin, ymax)
            }
        }
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.facecolor.unwrap_or(self.edgecolor)
    }
}
