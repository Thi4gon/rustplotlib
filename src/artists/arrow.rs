use tiny_skia::{Paint, PathBuilder, Pixmap};

use crate::artists::Artist;
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;

/// A FancyArrowPatch — draws an arrow from (x, y) to (x+dx, y+dy).
pub struct Arrow {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub width: f32,
    pub head_width: f32,
    pub head_length: f32,
    pub color: Color,
    pub alpha: f32,
    pub label: Option<String>,
    pub zorder: i32,
}

impl Arrow {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64, color: Color) -> Self {
        Arrow {
            x,
            y,
            dx,
            dy,
            width: 2.0,
            head_width: 10.0,
            head_length: 8.0,
            color,
            alpha: 1.0,
            label: None,
            zorder: 1,
        }
    }
}

impl Artist for Arrow {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let (px1, py1) = transform.transform_xy(self.x, self.y);
        let (px2, py2) = transform.transform_xy(self.x + self.dx, self.y + self.dy);

        let mut c = self.color;
        c.a = (self.alpha * 255.0) as u8;
        let mut paint = Paint::default();
        paint.set_color(c.to_tiny_skia());
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Direction vector in pixel space
        let vx = px2 - px1;
        let vy = py2 - py1;
        let length = (vx * vx + vy * vy).sqrt();
        if length < 1.0 {
            return;
        }

        // Unit direction
        let ux = vx / length;
        let uy = vy / length;

        // Perpendicular
        let nx = -uy;
        let ny = ux;

        // Shaft end (where head begins)
        let head_len = self.head_length.min(length * 0.8);
        let shaft_end_x = px2 - ux * head_len;
        let shaft_end_y = py2 - uy * head_len;

        let hw = self.width / 2.0;

        // Draw shaft as a filled rectangle
        let mut pb = PathBuilder::new();
        pb.move_to(px1 + nx * hw, py1 + ny * hw);
        pb.line_to(shaft_end_x + nx * hw, shaft_end_y + ny * hw);
        pb.line_to(shaft_end_x - nx * hw, shaft_end_y - ny * hw);
        pb.line_to(px1 - nx * hw, py1 - ny * hw);
        pb.close();
        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
        }

        // Draw arrowhead as a triangle
        let ahw = self.head_width / 2.0;
        let mut pb = PathBuilder::new();
        pb.move_to(px2, py2); // tip
        pb.line_to(shaft_end_x + nx * ahw, shaft_end_y + ny * ahw);
        pb.line_to(shaft_end_x - nx * ahw, shaft_end_y - ny * ahw);
        pb.close();
        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
        }
    }

    fn draw_svg(&self, svg: &mut crate::svg_renderer::SvgRenderer, transform: &Transform) {
        let (px1, py1) = transform.transform_xy(self.x, self.y);
        let (px2, py2) = transform.transform_xy(self.x + self.dx, self.y + self.dy);

        let vx = px2 - px1;
        let vy = py2 - py1;
        let length = (vx * vx + vy * vy).sqrt();
        if length < 1.0 {
            return;
        }

        let ux = vx / length;
        let uy = vy / length;
        let nx = -uy;
        let ny = ux;

        let head_len = self.head_length.min(length * 0.8);
        let shaft_end_x = px2 - ux * head_len;
        let shaft_end_y = py2 - uy * head_len;

        let hw = self.width / 2.0;
        let ahw = self.head_width / 2.0;

        let fill = crate::svg_renderer::color_to_svg(&self.color);

        // Shaft polygon
        let shaft_pts = vec![
            (px1 + nx * hw, py1 + ny * hw),
            (shaft_end_x + nx * hw, shaft_end_y + ny * hw),
            (shaft_end_x - nx * hw, shaft_end_y - ny * hw),
            (px1 - nx * hw, py1 - ny * hw),
        ];
        svg.add_polygon(&shaft_pts, &fill, "none", 0.0, self.alpha);

        // Head triangle
        let head_pts = vec![
            (px2, py2),
            (shaft_end_x + nx * ahw, shaft_end_y + ny * ahw),
            (shaft_end_x - nx * ahw, shaft_end_y - ny * ahw),
        ];
        svg.add_polygon(&head_pts, &fill, "none", 0.0, self.alpha);
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let x1 = self.x;
        let y1 = self.y;
        let x2 = self.x + self.dx;
        let y2 = self.y + self.dy;
        (x1.min(x2), x1.max(x2), y1.min(y2), y1.max(y2))
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
