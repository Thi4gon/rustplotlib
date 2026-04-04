use crate::artists::{Artist, MarkerStyle, draw_marker};
use crate::artists::legend::LegendEntry;
use crate::artists::line2d::draw_marker_svg;
use crate::colors::Color;
use crate::svg_renderer::{SvgRenderer, color_to_svg};
use crate::transforms::Transform;
use tiny_skia::Pixmap;

pub struct Scatter {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub sizes: Vec<f32>,
    pub color: Color,
    pub marker: MarkerStyle,
    pub label: Option<String>,
    pub alpha: f32,
}

impl Scatter {
    pub fn new(x: Vec<f64>, y: Vec<f64>, color: Color) -> Self {
        Self {
            x,
            y,
            sizes: vec![36.0],
            color,
            marker: MarkerStyle::Circle,
            label: None,
            alpha: 1.0,
        }
    }
}

impl Artist for Scatter {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());

        for i in 0..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
            let size_val = if self.sizes.len() == 1 {
                self.sizes[0]
            } else if i < self.sizes.len() {
                self.sizes[i]
            } else {
                36.0
            };
            // size is area-like, so marker render size = sqrt(size_val)
            let marker_size = size_val.sqrt();
            draw_marker(pixmap, self.marker, px, py, marker_size, self.color, self.alpha);
        }
    }

    fn draw_svg(&self, svg: &mut SvgRenderer, transform: &Transform) {
        if self.x.is_empty() || self.y.is_empty() {
            return;
        }
        let n = self.x.len().min(self.y.len());
        let color_str = color_to_svg(&self.color);

        for i in 0..n {
            let (px, py) = transform.transform_xy(self.x[i], self.y[i]);
            let size_val = if self.sizes.len() == 1 {
                self.sizes[0]
            } else if i < self.sizes.len() {
                self.sizes[i]
            } else {
                36.0
            };
            let marker_r = size_val.sqrt() / 2.0;
            draw_marker_svg(svg, self.marker, px, py, marker_r, &color_str, self.alpha);
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

    fn legend_entry(&self) -> Option<LegendEntry> {
        self.legend_label().map(|label| LegendEntry {
            label: label.to_string(),
            color: self.color,
            line_style: None,
            marker: Some(self.marker),
            linewidth: 1.5,
        })
    }
}
