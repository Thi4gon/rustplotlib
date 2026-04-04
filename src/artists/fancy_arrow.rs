use tiny_skia::{Paint, PathBuilder, Pixmap};

use crate::artists::Artist;
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::transforms::Transform;

/// Arrow style for FancyArrowPatch
#[derive(Clone, Debug)]
pub enum ArrowStyle {
    /// -> simple arrow
    Simple,
    /// -|> filled triangle head
    FilledHead,
    /// <- reverse arrow
    Reverse,
    /// <-> double-headed
    DoubleHead,
    /// -[ bracket
    Bracket,
    /// |- bar head
    Bar,
    /// wedge (wider at base)
    Wedge,
}

impl ArrowStyle {
    pub fn from_str(s: &str) -> Self {
        match s {
            "->" | "simple" => ArrowStyle::Simple,
            "-|>" | "filledhead" => ArrowStyle::FilledHead,
            "<-" | "reverse" => ArrowStyle::Reverse,
            "<->" | "doublehead" => ArrowStyle::DoubleHead,
            "-[" | "bracket" => ArrowStyle::Bracket,
            "|-" | "bar" => ArrowStyle::Bar,
            "wedge" => ArrowStyle::Wedge,
            _ => ArrowStyle::Simple,
        }
    }
}

/// Connection style for FancyArrowPatch
#[derive(Clone, Debug)]
pub enum ConnectionStyle {
    /// Straight line
    Arc3 { rad: f64 },
    /// Angle connector (right-angle)
    Angle { angleA: f64, angleB: f64 },
    /// Angle3 connector (smooth)
    Angle3 { angleA: f64, angleB: f64 },
}

impl ConnectionStyle {
    pub fn from_str(s: &str) -> Self {
        if s.starts_with("arc3") {
            let rad = s.split("rad=")
                .nth(1)
                .and_then(|v| v.trim().parse::<f64>().ok())
                .unwrap_or(0.0);
            ConnectionStyle::Arc3 { rad }
        } else if s.starts_with("angle3") {
            ConnectionStyle::Angle3 { angleA: 90.0, angleB: 0.0 }
        } else if s.starts_with("angle") {
            ConnectionStyle::Angle { angleA: 90.0, angleB: 0.0 }
        } else {
            ConnectionStyle::Arc3 { rad: 0.0 }
        }
    }
}

/// FancyArrowPatch — draws styled arrows between two points
pub struct FancyArrow {
    pub pos_a: (f64, f64),
    pub pos_b: (f64, f64),
    pub arrow_style: ArrowStyle,
    pub connection_style: ConnectionStyle,
    pub color: Color,
    pub linewidth: f32,
    pub head_width: f32,
    pub head_length: f32,
    pub shrink_a: f32,
    pub shrink_b: f32,
    pub mutation_scale: f32,
    pub alpha: f32,
    pub label: Option<String>,
    pub zorder: i32,
}

impl FancyArrow {
    pub fn new(pos_a: (f64, f64), pos_b: (f64, f64), color: Color) -> Self {
        FancyArrow {
            pos_a,
            pos_b,
            arrow_style: ArrowStyle::Simple,
            connection_style: ConnectionStyle::Arc3 { rad: 0.0 },
            color,
            linewidth: 1.5,
            head_width: 10.0,
            head_length: 8.0,
            shrink_a: 0.0,
            shrink_b: 0.0,
            mutation_scale: 20.0,
            alpha: 1.0,
            label: None,
            zorder: 3,
        }
    }

    /// Compute the arc path points for arc3 connection style
    fn arc3_points(&self, px1: f32, py1: f32, px2: f32, py2: f32, rad: f64) -> Vec<(f32, f32)> {
        if rad.abs() < 1e-10 {
            // Straight line
            return vec![(px1, py1), (px2, py2)];
        }

        let mid_x = (px1 + px2) / 2.0;
        let mid_y = (py1 + py2) / 2.0;
        let dx = px2 - px1;
        let dy = py2 - py1;
        // Perpendicular offset
        let ctrl_x = mid_x - dy * rad as f32;
        let ctrl_y = mid_y + dx * rad as f32;

        // Generate quadratic bezier points
        let n = 20;
        let mut points = Vec::with_capacity(n + 1);
        for i in 0..=n {
            let t = i as f32 / n as f32;
            let u = 1.0 - t;
            let x = u * u * px1 + 2.0 * u * t * ctrl_x + t * t * px2;
            let y = u * u * py1 + 2.0 * u * t * ctrl_y + t * t * py2;
            points.push((x, y));
        }
        points
    }

    fn draw_arrowhead(&self, pixmap: &mut Pixmap, tip_x: f32, tip_y: f32, dir_x: f32, dir_y: f32, paint: &Paint) {
        let ts = tiny_skia::Transform::identity();
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
        if len < 1e-6 {
            return;
        }
        let ux = dir_x / len;
        let uy = dir_y / len;
        let nx = -uy;
        let ny = ux;

        let hl = self.head_length * self.mutation_scale / 20.0;
        let hw = self.head_width * self.mutation_scale / 20.0 / 2.0;

        let base_x = tip_x - ux * hl;
        let base_y = tip_y - uy * hl;

        let mut pb = PathBuilder::new();
        pb.move_to(tip_x, tip_y);
        pb.line_to(base_x + nx * hw, base_y + ny * hw);
        pb.line_to(base_x - nx * hw, base_y - ny * hw);
        pb.close();
        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, paint, tiny_skia::FillRule::Winding, ts, None);
        }
    }
}

impl Artist for FancyArrow {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        let (px1, py1) = transform.transform_xy(self.pos_a.0, self.pos_a.1);
        let (px2, py2) = transform.transform_xy(self.pos_b.0, self.pos_b.1);

        // Apply shrink
        let vx = px2 - px1;
        let vy = py2 - py1;
        let total_len = (vx * vx + vy * vy).sqrt();
        if total_len < 1.0 {
            return;
        }
        let ux = vx / total_len;
        let uy = vy / total_len;
        let sx1 = px1 + ux * self.shrink_a;
        let sy1 = py1 + uy * self.shrink_a;
        let sx2 = px2 - ux * self.shrink_b;
        let sy2 = py2 - uy * self.shrink_b;

        let mut c = self.color;
        c.a = (self.alpha * 255.0) as u8;
        let mut paint = Paint::default();
        paint.set_color(c.to_tiny_skia());
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Get path points based on connection style
        let points = match &self.connection_style {
            ConnectionStyle::Arc3 { rad } => self.arc3_points(sx1, sy1, sx2, sy2, *rad),
            _ => vec![(sx1, sy1), (sx2, sy2)],
        };

        // Draw the path
        if points.len() >= 2 {
            let mut pb = PathBuilder::new();
            pb.move_to(points[0].0, points[0].1);
            for p in &points[1..] {
                pb.line_to(p.0, p.1);
            }
            if let Some(path) = pb.finish() {
                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = self.linewidth;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }

            // Draw arrowheads based on style
            let n = points.len();
            let tip = points[n - 1];
            let pre_tip = points[n - 2];
            let fwd_x = tip.0 - pre_tip.0;
            let fwd_y = tip.1 - pre_tip.1;

            match &self.arrow_style {
                ArrowStyle::Simple | ArrowStyle::FilledHead => {
                    self.draw_arrowhead(pixmap, tip.0, tip.1, fwd_x, fwd_y, &paint);
                }
                ArrowStyle::Reverse => {
                    let start = points[0];
                    let post_start = points[1];
                    let rev_x = start.0 - post_start.0;
                    let rev_y = start.1 - post_start.1;
                    self.draw_arrowhead(pixmap, start.0, start.1, rev_x, rev_y, &paint);
                }
                ArrowStyle::DoubleHead => {
                    self.draw_arrowhead(pixmap, tip.0, tip.1, fwd_x, fwd_y, &paint);
                    let start = points[0];
                    let post_start = points[1];
                    let rev_x = start.0 - post_start.0;
                    let rev_y = start.1 - post_start.1;
                    self.draw_arrowhead(pixmap, start.0, start.1, rev_x, rev_y, &paint);
                }
                ArrowStyle::Wedge => {
                    // Wedge: thick at base, thin at tip
                    let start = points[0];
                    let wedge_w = self.head_width * self.mutation_scale / 20.0 / 2.0;
                    let fwd_len = (fwd_x * fwd_x + fwd_y * fwd_y).sqrt();
                    if fwd_len > 1e-6 {
                        let nx = -fwd_y / fwd_len;
                        let ny = fwd_x / fwd_len;
                        let mut pb = PathBuilder::new();
                        pb.move_to(start.0 + nx * wedge_w, start.1 + ny * wedge_w);
                        pb.line_to(start.0 - nx * wedge_w, start.1 - ny * wedge_w);
                        pb.line_to(tip.0, tip.1);
                        pb.close();
                        if let Some(path) = pb.finish() {
                            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
                        }
                    }
                }
                ArrowStyle::Bar => {
                    // Bar: perpendicular line at the end
                    let bar_w = self.head_width * self.mutation_scale / 20.0 / 2.0;
                    let fwd_len = (fwd_x * fwd_x + fwd_y * fwd_y).sqrt();
                    if fwd_len > 1e-6 {
                        let nx = -fwd_y / fwd_len;
                        let ny = fwd_x / fwd_len;
                        let mut pb = PathBuilder::new();
                        pb.move_to(tip.0 + nx * bar_w, tip.1 + ny * bar_w);
                        pb.line_to(tip.0 - nx * bar_w, tip.1 - ny * bar_w);
                        if let Some(path) = pb.finish() {
                            let mut stroke = tiny_skia::Stroke::default();
                            stroke.width = self.linewidth * 2.0;
                            pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                        }
                    }
                }
                ArrowStyle::Bracket => {
                    // Bracket: [ shape at end
                    let bar_w = self.head_width * self.mutation_scale / 20.0 / 2.0;
                    let bracket_depth = self.head_length * self.mutation_scale / 20.0 * 0.5;
                    let fwd_len = (fwd_x * fwd_x + fwd_y * fwd_y).sqrt();
                    if fwd_len > 1e-6 {
                        let ux = fwd_x / fwd_len;
                        let uy = fwd_y / fwd_len;
                        let nx = -fwd_y / fwd_len;
                        let ny = fwd_x / fwd_len;
                        let mut pb = PathBuilder::new();
                        pb.move_to(tip.0 - ux * bracket_depth + nx * bar_w,
                                   tip.1 - uy * bracket_depth + ny * bar_w);
                        pb.line_to(tip.0 + nx * bar_w, tip.1 + ny * bar_w);
                        pb.line_to(tip.0 - nx * bar_w, tip.1 - ny * bar_w);
                        pb.line_to(tip.0 - ux * bracket_depth - nx * bar_w,
                                   tip.1 - uy * bracket_depth - ny * bar_w);
                        if let Some(path) = pb.finish() {
                            let mut stroke = tiny_skia::Stroke::default();
                            stroke.width = self.linewidth;
                            pixmap.stroke_path(&path, &paint, &stroke, ts, None);
                        }
                    }
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        let x1 = self.pos_a.0;
        let y1 = self.pos_a.1;
        let x2 = self.pos_b.0;
        let y2 = self.pos_b.1;
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
