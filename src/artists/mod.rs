pub mod line2d;
pub mod scatter;
pub mod bar;
pub mod hist;
pub mod image;
pub mod legend;
pub mod fill_between;
pub mod step;
pub mod pie;
pub mod errorbar;
pub mod barh;
pub mod boxplot;
pub mod stem;
pub mod contour;
pub mod hexbin;
pub mod patches;
pub mod quiver;
pub mod streamplot;
pub mod line3d;
pub mod scatter3d;
pub mod surface3d;
pub mod wireframe3d;
pub mod bar3d;
pub mod contour3d;
pub mod violin;
pub mod fill_betweenx;
pub mod trisurf3d;
pub mod radar;
pub mod broken_barh;
pub mod eventplot;
pub mod fill_polygon;
pub mod pcolormesh;
pub mod sankey;
pub mod arrow;

use tiny_skia::{PathBuilder, StrokeDash};

use crate::colors::Color;
use crate::transforms::Transform;

use crate::artists::legend::LegendEntry;

/// Trait that all drawable artist types implement.
pub trait Artist: Send {
    /// Draw this artist onto the pixmap using the given transform.
    fn draw(&self, pixmap: &mut tiny_skia::Pixmap, transform: &Transform);

    /// Draw this artist as SVG elements using the given transform.
    fn draw_svg(&self, _svg: &mut crate::svg_renderer::SvgRenderer, _transform: &Transform) {
        // Default: no SVG rendering (artists that don't implement this are skipped)
    }

    /// Return the data-space bounding box (xmin, xmax, ymin, ymax).
    fn data_bounds(&self) -> (f64, f64, f64, f64);

    /// Return the legend label, if any.
    fn legend_label(&self) -> Option<&str>;

    /// Return the representative color for this artist (used in legends).
    fn legend_color(&self) -> Color;

    /// Return a full legend entry with line style, marker, and color information.
    fn legend_entry(&self) -> Option<LegendEntry> {
        // Default implementation: colored square swatch (for bar/hist)
        self.legend_label().map(|label| LegendEntry {
            label: label.to_string(),
            color: self.legend_color(),
            line_style: None,
            marker: None,
            linewidth: 1.5,
        })
    }

    /// Return the z-order for drawing priority (lower = drawn first = behind).
    fn zorder(&self) -> i32 {
        0
    }
}

// ---------------------------------------------------------------------------
// LineStyle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    DashDot,
    Dotted,
    None,
}

impl LineStyle {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "-" | "solid" => LineStyle::Solid,
            "--" | "dashed" => LineStyle::Dashed,
            "-." | "dashdot" => LineStyle::DashDot,
            ":" | "dotted" => LineStyle::Dotted,
            "none" | "" => LineStyle::None,
            _ => LineStyle::Solid,
        }
    }

    /// Convert to a `StrokeDash` for tiny_skia. Returns `None` for Solid.
    pub fn to_dash(&self, linewidth: f32) -> Option<StrokeDash> {
        match self {
            LineStyle::Solid | LineStyle::None => None,
            LineStyle::Dashed => {
                let d = linewidth * 4.0;
                let g = linewidth * 2.0;
                StrokeDash::new(vec![d, g], 0.0)
            }
            LineStyle::DashDot => {
                let d = linewidth * 4.0;
                let dot = linewidth;
                let g = linewidth * 2.0;
                StrokeDash::new(vec![d, g, dot, g], 0.0)
            }
            LineStyle::Dotted => {
                let dot = linewidth;
                let g = linewidth * 2.0;
                StrokeDash::new(vec![dot, g], 0.0)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// MarkerStyle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarkerStyle {
    None,
    Point,
    Circle,
    Square,
    TriangleUp,
    TriangleDown,
    Plus,
    Cross,
    Diamond,
    Star,
}

impl MarkerStyle {
    pub fn from_str(s: &str) -> Self {
        match s {
            "." => MarkerStyle::Point,
            "o" => MarkerStyle::Circle,
            "s" => MarkerStyle::Square,
            "^" => MarkerStyle::TriangleUp,
            "v" => MarkerStyle::TriangleDown,
            "+" => MarkerStyle::Plus,
            "x" => MarkerStyle::Cross,
            "D" | "d" => MarkerStyle::Diamond,
            "*" => MarkerStyle::Star,
            "none" | "" => MarkerStyle::None,
            _ => MarkerStyle::None,
        }
    }
}

// ---------------------------------------------------------------------------
// Marker drawing helpers
// ---------------------------------------------------------------------------

/// Build a circle path centered at (cx, cy) with the given radius.
pub fn circle_path(cx: f32, cy: f32, r: f32) -> Option<tiny_skia::Path> {
    // Approximate circle with 4 cubic bezier curves (kappa = 0.5522847498).
    let k = r * 0.5522847498;
    let mut pb = PathBuilder::new();
    pb.move_to(cx, cy - r);
    pb.cubic_to(cx + k, cy - r, cx + r, cy - k, cx + r, cy);
    pb.cubic_to(cx + r, cy + k, cx + k, cy + r, cx, cy + r);
    pb.cubic_to(cx - k, cy + r, cx - r, cy + k, cx - r, cy);
    pb.cubic_to(cx - r, cy - k, cx - k, cy - r, cx, cy - r);
    pb.close();
    pb.finish()
}

/// Build a 5-pointed star path centered at (cx, cy) with the given outer radius.
pub fn star_path(cx: f32, cy: f32, r: f32) -> Option<tiny_skia::Path> {
    let inner_r = r * 0.38;
    let mut pb = PathBuilder::new();
    for i in 0..10 {
        let angle = std::f32::consts::FRAC_PI_2 * -1.0 + i as f32 * std::f32::consts::PI / 5.0;
        let radius = if i % 2 == 0 { r } else { inner_r };
        let px = cx + radius * angle.cos();
        let py = cy + radius * angle.sin();
        if i == 0 {
            pb.move_to(px, py);
        } else {
            pb.line_to(px, py);
        }
    }
    pb.close();
    pb.finish()
}

/// Draw a single marker at pixel coordinates (cx, cy).
pub fn draw_marker(
    pixmap: &mut tiny_skia::Pixmap,
    marker: MarkerStyle,
    cx: f32,
    cy: f32,
    size: f32,
    color: Color,
    alpha: f32,
) {
    if marker == MarkerStyle::None {
        return;
    }

    let r = size / 2.0;
    let ts_color = {
        let mut c = color;
        c.a = (alpha * 255.0) as u8;
        c.to_tiny_skia()
    };

    let mut paint = tiny_skia::Paint::default();
    paint.set_color(ts_color);
    paint.anti_alias = true;

    let ts = tiny_skia::Transform::identity();

    match marker {
        MarkerStyle::None => {}
        MarkerStyle::Point => {
            if let Some(path) = circle_path(cx, cy, r * 0.5) {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            }
        }
        MarkerStyle::Circle => {
            if let Some(path) = circle_path(cx, cy, r) {
                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = 1.0;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }
        MarkerStyle::Square => {
            let rect = tiny_skia::Rect::from_xywh(cx - r, cy - r, size, size);
            if let Some(rect) = rect {
                let path = PathBuilder::from_rect(rect);
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            }
        }
        MarkerStyle::TriangleUp => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy - r);
            pb.line_to(cx + r, cy + r);
            pb.line_to(cx - r, cy + r);
            pb.close();
            if let Some(path) = pb.finish() {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            }
        }
        MarkerStyle::TriangleDown => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy + r);
            pb.line_to(cx + r, cy - r);
            pb.line_to(cx - r, cy - r);
            pb.close();
            if let Some(path) = pb.finish() {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            }
        }
        MarkerStyle::Plus => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy - r);
            pb.line_to(cx, cy + r);
            pb.move_to(cx - r, cy);
            pb.line_to(cx + r, cy);
            if let Some(path) = pb.finish() {
                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = 1.5;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }
        MarkerStyle::Cross => {
            let d = r * 0.707; // r / sqrt(2)
            let mut pb = PathBuilder::new();
            pb.move_to(cx - d, cy - d);
            pb.line_to(cx + d, cy + d);
            pb.move_to(cx + d, cy - d);
            pb.line_to(cx - d, cy + d);
            if let Some(path) = pb.finish() {
                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = 1.5;
                pixmap.stroke_path(&path, &paint, &stroke, ts, None);
            }
        }
        MarkerStyle::Diamond => {
            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy - r);
            pb.line_to(cx + r, cy);
            pb.line_to(cx, cy + r);
            pb.line_to(cx - r, cy);
            pb.close();
            if let Some(path) = pb.finish() {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            }
        }
        MarkerStyle::Star => {
            if let Some(path) = star_path(cx, cy, r) {
                pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            }
        }
    }
}
