use crate::artists::{Artist, LineStyle, MarkerStyle, draw_marker};
use crate::artists::legend::LegendEntry;
use crate::colors::Color;
use crate::svg_renderer::{SvgRenderer, color_to_svg, linestyle_to_dash};
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Stroke, Pixmap};
use std::f32::consts::PI;

pub struct Line2D {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub color: Color,
    pub linewidth: f32,
    pub linestyle: LineStyle,
    pub marker: MarkerStyle,
    pub marker_size: f32,
    pub marker_every: usize,
    pub label: Option<String>,
    pub alpha: f32,
    pub zorder: i32,
    pub outline_color: Option<Color>,
    pub outline_width: Option<f32>,
}

impl Line2D {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            y,
            color,
            linewidth: 1.5,
            linestyle: LineStyle::Solid,
            marker: MarkerStyle::None,
            marker_size: 6.0,
            marker_every: 1,
            label: None,
            alpha: 1.0,
            zorder: 2,
            outline_color: None,
            outline_width: None,
        }
    }
}

impl Artist for Line2D {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());

        // Set up paint with alpha
        let mut color = self.color;
        color.a = (self.alpha * 255.0) as u8;
        let ts_color = color.to_tiny_skia();

        let mut paint = Paint::default();
        paint.set_color(ts_color);
        paint.anti_alias = true;

        let ts = tiny_skia::Transform::identity();

        // Build line path (shared between outline and main draw)
        let line_path = if self.linestyle != LineStyle::None && n >= 2 {
            let mut pb = PathBuilder::new();
            let mut in_path = false;
            for i in 0..n {
                let x = self.x[i];
                let y = self.y[i];

                if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                    in_path = false;
                    continue;
                }

                let (px, py) = transform.transform_xy(x, y);
                if !in_path {
                    pb.move_to(px, py);
                    in_path = true;
                } else {
                    pb.line_to(px, py);
                }
            }
            pb.finish()
        } else {
            None
        };

        // Draw outline (path effect: withStroke) if set
        if let (Some(ref path), Some(ref out_color), Some(out_width)) =
            (&line_path, &self.outline_color, self.outline_width)
        {
            let mut out_paint = Paint::default();
            let mut oc = *out_color;
            oc.a = (self.alpha * 255.0) as u8;
            out_paint.set_color(oc.to_tiny_skia());
            out_paint.anti_alias = true;
            let mut out_stroke = Stroke::default();
            out_stroke.width = out_width;
            out_stroke.dash = self.linestyle.to_dash(out_width);
            pixmap.stroke_path(path, &out_paint, &out_stroke, ts, None);
        }

        // Draw main line segments
        if let Some(ref path) = line_path {
            let mut stroke = Stroke::default();
            stroke.width = self.linewidth;
            stroke.dash = self.linestyle.to_dash(self.linewidth);
            pixmap.stroke_path(path, &paint, &stroke, ts, None);
        }

        // Draw markers (respecting marker_every, skipping NaN/infinite points)
        if self.marker != MarkerStyle::None {
            let every = self.marker_every.max(1);
            for i in 0..n {
                if i % every == 0 {
                    let x = self.x[i];
                    let y = self.y[i];
                    if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                        continue;
                    }
                    let (px, py) = transform.transform_xy(x, y);
                    draw_marker(pixmap, self.marker, px, py, self.marker_size, self.color, self.alpha);
                }
            }
        }
    }

    fn draw_svg(&self, svg: &mut SvgRenderer, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());
        let color_str = color_to_svg(&self.color);
        let opacity = self.alpha;

        // Build segments (split on NaN gaps)
        if self.linestyle != LineStyle::None && n >= 2 {
            let dash = linestyle_to_dash(&self.linestyle, self.linewidth);
            let dash_ref = dash.as_deref();

            let mut segments: Vec<Vec<(f32, f32)>> = Vec::new();
            let mut segment: Vec<(f32, f32)> = Vec::new();
            for i in 0..n {
                let x = self.x[i];
                let y = self.y[i];
                if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                    if segment.len() >= 2 {
                        segments.push(std::mem::take(&mut segment));
                    }
                    segment.clear();
                    continue;
                }
                let (px, py) = transform.transform_xy(x, y);
                segment.push((px, py));
            }
            if segment.len() >= 2 {
                segments.push(segment);
            }

            // Draw outline (path effect) first
            if let (Some(ref out_color), Some(out_width)) = (&self.outline_color, self.outline_width) {
                let out_color_str = color_to_svg(out_color);
                let out_dash = linestyle_to_dash(&self.linestyle, out_width);
                let out_dash_ref = out_dash.as_deref();
                for seg in &segments {
                    svg.add_polyline(seg, &out_color_str, out_width, "none", out_dash_ref, opacity);
                }
            }

            // Draw main line
            for seg in &segments {
                svg.add_polyline(seg, &color_str, self.linewidth, "none", dash_ref, opacity);
            }
        }

        // Draw markers
        if self.marker != MarkerStyle::None {
            let every = self.marker_every.max(1);
            let r = self.marker_size / 2.0;
            for i in 0..n {
                if i % every != 0 {
                    continue;
                }
                let x = self.x[i];
                let y = self.y[i];
                if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                    continue;
                }
                let (px, py) = transform.transform_xy(x, y);
                draw_marker_svg(svg, self.marker, px, py, r, &color_str, opacity);
            }
        }
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
            let x = self.x[i];
            let y = self.y[i];
            if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
                continue;
            }
            if x < xmin { xmin = x; }
            if x > xmax { xmax = x; }
            if y < ymin { ymin = y; }
            if y > ymax { ymax = y; }
        }
        if xmin == f64::MAX {
            return (0.0, 1.0, 0.0, 1.0);
        }
        (xmin, xmax, ymin, ymax)
    }

    fn legend_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn legend_color(&self) -> Color {
        self.color
    }

    fn legend_entry(&self) -> Option<LegendEntry> {
        self.legend_label().map(|label| LegendEntry {
            label: label.to_string(),
            color: self.color,
            line_style: Some(self.linestyle),
            marker: if self.marker != MarkerStyle::None { Some(self.marker) } else { None },
            linewidth: self.linewidth,
        })
    }

    fn zorder(&self) -> i32 {
        self.zorder
    }
}

/// Draw a marker as SVG at pixel coordinates.
pub fn draw_marker_svg(
    svg: &mut SvgRenderer,
    marker: MarkerStyle,
    cx: f32,
    cy: f32,
    r: f32,
    color: &str,
    opacity: f32,
) {
    match marker {
        MarkerStyle::None => {}
        MarkerStyle::Point => {
            svg.add_circle(cx, cy, r * 0.5, color, "none", 0.0, opacity);
        }
        MarkerStyle::Circle => {
            svg.add_circle(cx, cy, r, "none", color, 1.0, opacity);
        }
        MarkerStyle::Square => {
            svg.add_rect(cx - r, cy - r, r * 2.0, r * 2.0, color, "none", 0.0, opacity);
        }
        MarkerStyle::TriangleUp => {
            let pts = vec![
                (cx, cy - r),
                (cx + r, cy + r),
                (cx - r, cy + r),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::TriangleDown => {
            let pts = vec![
                (cx, cy + r),
                (cx + r, cy - r),
                (cx - r, cy - r),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::Plus => {
            svg.add_line(cx, cy - r, cx, cy + r, color, 1.5, None, opacity);
            svg.add_line(cx - r, cy, cx + r, cy, color, 1.5, None, opacity);
        }
        MarkerStyle::Cross => {
            let d = r * 0.707;
            svg.add_line(cx - d, cy - d, cx + d, cy + d, color, 1.5, None, opacity);
            svg.add_line(cx + d, cy - d, cx - d, cy + d, color, 1.5, None, opacity);
        }
        MarkerStyle::Diamond => {
            let pts = vec![
                (cx, cy - r),
                (cx + r, cy),
                (cx, cy + r),
                (cx - r, cy),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::Star => {
            // 5-pointed star
            let inner_r = r * 0.38;
            let mut pts = Vec::new();
            for i in 0..10 {
                let angle = -PI / 2.0 + i as f32 * PI / 5.0;
                let radius = if i % 2 == 0 { r } else { inner_r };
                pts.push((cx + radius * angle.cos(), cy + radius * angle.sin()));
            }
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::TriangleLeft => {
            let pts = vec![
                (cx - r, cy),
                (cx + r, cy - r),
                (cx + r, cy + r),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::TriangleRight => {
            let pts = vec![
                (cx + r, cy),
                (cx - r, cy - r),
                (cx - r, cy + r),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::Pentagon => {
            let mut pts = Vec::new();
            for i in 0..5usize {
                let angle = -PI / 2.0 + i as f32 * 2.0 * PI / 5.0;
                pts.push((cx + r * angle.cos(), cy + r * angle.sin()));
            }
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::Hexagon => {
            let mut pts = Vec::new();
            for i in 0..6usize {
                let angle = -PI / 2.0 + i as f32 * 2.0 * PI / 6.0;
                pts.push((cx + r * angle.cos(), cy + r * angle.sin()));
            }
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::HexagonFlat => {
            let mut pts = Vec::new();
            for i in 0..6usize {
                let angle = i as f32 * 2.0 * PI / 6.0;
                pts.push((cx + r * angle.cos(), cy + r * angle.sin()));
            }
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::Octagon => {
            let start = -PI / 2.0 / 2.0;
            let mut pts = Vec::new();
            for i in 0..8usize {
                let angle = start + i as f32 * 2.0 * PI / 8.0;
                pts.push((cx + r * angle.cos(), cy + r * angle.sin()));
            }
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::VLine => {
            svg.add_line(cx, cy - r, cx, cy + r, color, 1.5, None, opacity);
        }
        MarkerStyle::HLine => {
            svg.add_line(cx - r, cy, cx + r, cy, color, 1.5, None, opacity);
        }
        MarkerStyle::PlusFilled => {
            let w = r * 0.4;
            let pts = vec![
                (cx - w, cy - r), (cx + w, cy - r), (cx + w, cy - w),
                (cx + r, cy - w), (cx + r, cy + w), (cx + w, cy + w),
                (cx + w, cy + r), (cx - w, cy + r), (cx - w, cy + w),
                (cx - r, cy + w), (cx - r, cy - w), (cx - w, cy - w),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::CrossFilled => {
            let w = r * 0.35;
            let d = r * 0.707;
            let wd = w * 0.707;
            let pts = vec![
                (cx, cy - wd * 2.0),
                (cx + d - wd, cy - d + wd),
                (cx + d + wd, cy - d - wd),
                (cx + wd * 2.0, cy),
                (cx + d + wd, cy + d + wd),
                (cx + d - wd, cy + d - wd),
                (cx, cy + wd * 2.0),
                (cx - d + wd, cy + d - wd),
                (cx - d - wd, cy + d + wd),
                (cx - wd * 2.0, cy),
                (cx - d - wd, cy - d - wd),
                (cx - d + wd, cy - d + wd),
            ];
            svg.add_polygon(&pts, color, "none", 0.0, opacity);
        }
        MarkerStyle::TriDown => {
            svg.add_line(cx, cy, cx, cy + r, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx - r * 0.866, cy - r * 0.5, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx + r * 0.866, cy - r * 0.5, color, 1.5, None, opacity);
        }
        MarkerStyle::TriUp => {
            svg.add_line(cx, cy, cx, cy - r, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx - r * 0.866, cy + r * 0.5, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx + r * 0.866, cy + r * 0.5, color, 1.5, None, opacity);
        }
        MarkerStyle::TriLeft => {
            svg.add_line(cx, cy, cx - r, cy, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx + r * 0.5, cy - r * 0.866, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx + r * 0.5, cy + r * 0.866, color, 1.5, None, opacity);
        }
        MarkerStyle::TriRight => {
            svg.add_line(cx, cy, cx + r, cy, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx - r * 0.5, cy - r * 0.866, color, 1.5, None, opacity);
            svg.add_line(cx, cy, cx - r * 0.5, cy + r * 0.866, color, 1.5, None, opacity);
        }
    }
}
