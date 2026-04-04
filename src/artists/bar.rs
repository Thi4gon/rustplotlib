use crate::artists::Artist;
use crate::colors::Color;
use crate::svg_renderer::{SvgRenderer, color_to_svg};
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Rect, Stroke, Pixmap};

pub struct Bar {
    pub x: Vec<f64>,
    pub heights: Vec<f64>,
    pub width: f64,
    pub color: Color,
    pub label: Option<String>,
    pub alpha: f32,
    pub bottom: f64,
    pub hatch: Option<String>,
    pub zorder: i32,
}

impl Bar {
    pub fn new(x: Vec<f64>, heights: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            heights,
            width: 0.8,
            color,
            label: None,
            alpha: 1.0,
            bottom: 0.0,
            hatch: None,
            zorder: 1,
        }
    }
}

/// Clip a line segment (x0,y0)-(x1,y1) to a rectangle. Returns None if fully outside.
fn clip_line_to_rect(
    mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32,
    rx: f32, ry: f32, rw: f32, rh: f32,
) -> Option<(f32, f32, f32, f32)> {
    let xmin = rx;
    let xmax = rx + rw;
    let ymin = ry;
    let ymax = ry + rh;

    // Cohen-Sutherland
    fn code(x: f32, y: f32, xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> u8 {
        let mut c = 0u8;
        if x < xmin { c |= 1; }
        if x > xmax { c |= 2; }
        if y < ymin { c |= 4; }
        if y > ymax { c |= 8; }
        c
    }

    let mut c0 = code(x0, y0, xmin, xmax, ymin, ymax);
    let mut c1 = code(x1, y1, xmin, xmax, ymin, ymax);

    for _ in 0..20 {
        if (c0 | c1) == 0 { return Some((x0, y0, x1, y1)); }
        if (c0 & c1) != 0 { return None; }
        let c_out = if c0 != 0 { c0 } else { c1 };
        let dx = x1 - x0;
        let dy = y1 - y0;
        let (nx, ny);
        if c_out & 8 != 0 {
            nx = x0 + dx * (ymax - y0) / dy;
            ny = ymax;
        } else if c_out & 4 != 0 {
            nx = x0 + dx * (ymin - y0) / dy;
            ny = ymin;
        } else if c_out & 2 != 0 {
            ny = y0 + dy * (xmax - x0) / dx;
            nx = xmax;
        } else {
            ny = y0 + dy * (xmin - x0) / dx;
            nx = xmin;
        }
        if c_out == c0 { x0 = nx; y0 = ny; c0 = code(x0, y0, xmin, xmax, ymin, ymax); }
        else { x1 = nx; y1 = ny; c1 = code(x1, y1, xmin, xmax, ymin, ymax); }
    }
    None
}

/// Draw a clipped line segment within a rect.
fn draw_clipped_line(
    pixmap: &mut Pixmap,
    x0: f32, y0: f32, x1: f32, y1: f32,
    rx: f32, ry: f32, rw: f32, rh: f32,
    paint: &Paint, stroke: &Stroke,
) {
    if let Some((cx0, cy0, cx1, cy1)) = clip_line_to_rect(x0, y0, x1, y1, rx, ry, rw, rh) {
        let mut pb = PathBuilder::new();
        pb.move_to(cx0, cy0);
        pb.line_to(cx1, cy1);
        if let Some(path) = pb.finish() {
            pixmap.stroke_path(&path, paint, stroke, tiny_skia::Transform::identity(), None);
        }
    }
}

/// Draw hatch pattern lines over a rectangle region.
pub fn draw_hatch(pixmap: &mut Pixmap, rect_x: f32, rect_y: f32, rect_w: f32, rect_h: f32, pattern: &str, color: Color) {
    let spacing = 6.0_f32;
    let mut paint = Paint::default();
    paint.set_color(color.to_tiny_skia());
    paint.anti_alias = true;
    let mut stroke = Stroke::default();
    stroke.width = 0.8;
    let ts = tiny_skia::Transform::identity();

    for ch in pattern.chars() {
        match ch {
            '/' => {
                // Diagonal lines from bottom-left to top-right
                let mut y = rect_y;
                while y < rect_y + rect_h + rect_w {
                    draw_clipped_line(pixmap, rect_x, y, rect_x + rect_w, y - rect_w,
                        rect_x, rect_y, rect_w, rect_h, &paint, &stroke);
                    y += spacing;
                }
            }
            '\\' => {
                // Diagonal lines from top-left to bottom-right
                let mut y = rect_y - rect_w;
                while y < rect_y + rect_h {
                    draw_clipped_line(pixmap, rect_x, y, rect_x + rect_w, y + rect_w,
                        rect_x, rect_y, rect_w, rect_h, &paint, &stroke);
                    y += spacing;
                }
            }
            '|' => {
                // Vertical lines
                let mut x = rect_x;
                while x < rect_x + rect_w {
                    let mut pb = PathBuilder::new();
                    pb.move_to(x, rect_y);
                    pb.line_to(x, rect_y + rect_h);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                    x += spacing;
                }
            }
            '-' => {
                // Horizontal lines
                let mut y = rect_y;
                while y < rect_y + rect_h {
                    let mut pb = PathBuilder::new();
                    pb.move_to(rect_x, y);
                    pb.line_to(rect_x + rect_w, y);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                    y += spacing;
                }
            }
            '+' => {
                // Grid: vertical + horizontal
                let mut x = rect_x;
                while x < rect_x + rect_w {
                    let mut pb = PathBuilder::new();
                    pb.move_to(x, rect_y);
                    pb.line_to(x, rect_y + rect_h);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                    x += spacing;
                }
                let mut y = rect_y;
                while y < rect_y + rect_h {
                    let mut pb = PathBuilder::new();
                    pb.move_to(rect_x, y);
                    pb.line_to(rect_x + rect_w, y);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                    }
                    y += spacing;
                }
            }
            'x' => {
                // Diagonal cross: both / and \ diagonals
                let mut y = rect_y;
                while y < rect_y + rect_h + rect_w {
                    draw_clipped_line(pixmap, rect_x, y, rect_x + rect_w, y - rect_w,
                        rect_x, rect_y, rect_w, rect_h, &paint, &stroke);
                    y += spacing;
                }
                let mut y2 = rect_y - rect_w;
                while y2 < rect_y + rect_h {
                    draw_clipped_line(pixmap, rect_x, y2, rect_x + rect_w, y2 + rect_w,
                        rect_x, rect_y, rect_w, rect_h, &paint, &stroke);
                    y2 += spacing;
                }
            }
            'o' | 'O' => {
                // Small circles
                let r = if ch == 'o' { 1.5 } else { 3.0 };
                let mut cy = rect_y + spacing / 2.0;
                while cy < rect_y + rect_h {
                    let mut cx = rect_x + spacing / 2.0;
                    while cx < rect_x + rect_w {
                        if let Some(circle) = crate::artists::circle_path(cx, cy, r) {
                            pixmap.stroke_path(&circle, &paint, &stroke, ts, None);
                        }
                        cx += spacing;
                    }
                    cy += spacing;
                }
            }
            '.' => {
                // Dots
                let mut cy = rect_y + spacing / 2.0;
                while cy < rect_y + rect_h {
                    let mut cx = rect_x + spacing / 2.0;
                    while cx < rect_x + rect_w {
                        if let Some(circle) = crate::artists::circle_path(cx, cy, 0.8) {
                            pixmap.fill_path(&circle, &paint, tiny_skia::FillRule::Winding, ts, None);
                        }
                        cx += spacing;
                    }
                    cy += spacing;
                }
            }
            '*' => {
                // Small stars
                let mut cy = rect_y + spacing / 2.0;
                while cy < rect_y + rect_h {
                    let mut cx = rect_x + spacing / 2.0;
                    while cx < rect_x + rect_w {
                        if let Some(star) = crate::artists::star_path(cx, cy, 2.5) {
                            pixmap.fill_path(&star, &paint, tiny_skia::FillRule::Winding, ts, None);
                        }
                        cx += spacing;
                    }
                    cy += spacing;
                }
            }
            _ => {}
        }
    }
}

impl Artist for Bar {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.heights.is_empty() {
            return;
        }
        let n = self.x.len().min(self.heights.len());

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;

        let mut fill_paint = Paint::default();
        fill_paint.set_color(fill_color.to_tiny_skia());
        fill_paint.anti_alias = true;

        // Border paint (slightly darker)
        let mut border_paint = Paint::default();
        let border_color = Color::new(
            (fill_color.r as u16 * 7 / 10) as u8,
            (fill_color.g as u16 * 7 / 10) as u8,
            (fill_color.b as u16 * 7 / 10) as u8,
            fill_color.a,
        );
        border_paint.set_color(border_color.to_tiny_skia());
        border_paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();
        let half_w = self.width / 2.0;

        for i in 0..n {
            let h = self.heights[i];
            if h.abs() < 1e-15 {
                continue;
            }

            let x_left = self.x[i] - half_w;
            let x_right = self.x[i] + half_w;

            // Bar goes from y=bottom to y=bottom+h
            let (y_bottom, y_top) = if h >= 0.0 {
                (self.bottom, self.bottom + h)
            } else {
                (self.bottom + h, self.bottom)
            };

            let (px_left, py_top) = transform.transform_xy(x_left, y_top);
            let (px_right, py_bottom) = transform.transform_xy(x_right, y_bottom);

            let rect_x = px_left.min(px_right);
            let rect_y = py_top.min(py_bottom);
            let rect_w = (px_right - px_left).abs().max(1.0);
            let rect_h = (py_bottom - py_top).abs().max(1.0);

            if let Some(rect) = Rect::from_xywh(rect_x, rect_y, rect_w, rect_h) {
                // Fill
                pixmap.fill_rect(rect, &fill_paint, ts, None);

                // Hatch pattern
                if let Some(ref hatch) = self.hatch {
                    let hatch_color = Color::new(0, 0, 0, fill_color.a);
                    draw_hatch(pixmap, rect_x, rect_y, rect_w, rect_h, hatch, hatch_color);
                }

                // Border stroke
                let path = PathBuilder::from_rect(rect);
                let mut stroke = Stroke::default();
                stroke.width = 0.5;
                pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
            }
        }
    }

    fn draw_svg(&self, svg: &mut SvgRenderer, transform: &Transform) {
        if self.x.is_empty() || self.heights.is_empty() {
            return;
        }
        let n = self.x.len().min(self.heights.len());

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;
        let fill_str = color_to_svg(&fill_color);

        let border_color = Color::new(
            (fill_color.r as u16 * 7 / 10) as u8,
            (fill_color.g as u16 * 7 / 10) as u8,
            (fill_color.b as u16 * 7 / 10) as u8,
            fill_color.a,
        );
        let border_str = color_to_svg(&border_color);

        let half_w = self.width / 2.0;

        for i in 0..n {
            let h = self.heights[i];
            if h.abs() < 1e-15 {
                continue;
            }

            let x_left = self.x[i] - half_w;
            let x_right = self.x[i] + half_w;

            let (y_bottom, y_top) = if h >= 0.0 {
                (self.bottom, self.bottom + h)
            } else {
                (self.bottom + h, self.bottom)
            };

            let (px_left, py_top) = transform.transform_xy(x_left, y_top);
            let (px_right, py_bottom) = transform.transform_xy(x_right, y_bottom);

            let rect_x = px_left.min(px_right);
            let rect_y = py_top.min(py_bottom);
            let rect_w = (px_right - px_left).abs().max(1.0);
            let rect_h = (py_bottom - py_top).abs().max(1.0);

            svg.add_rect(rect_x, rect_y, rect_w, rect_h, &fill_str, &border_str, 0.5, self.alpha);
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if self.x.is_empty() || self.heights.is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }
        let n = self.x.len().min(self.heights.len());
        let half_w = self.width / 2.0;

        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = self.bottom;
        let mut ymax = self.bottom;

        for i in 0..n {
            let xl = self.x[i] - half_w;
            let xr = self.x[i] + half_w;
            if xl < xmin { xmin = xl; }
            if xr > xmax { xmax = xr; }
            let y_top = self.bottom + self.heights[i];
            if y_top < ymin { ymin = y_top; }
            if y_top > ymax { ymax = y_top; }
            if self.bottom < ymin { ymin = self.bottom; }
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
