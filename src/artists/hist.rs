use crate::artists::Artist;
use crate::artists::bar::Bar;
use crate::colors::Color;
use crate::svg_renderer::SvgRenderer;
use crate::transforms::Transform;
use tiny_skia::Pixmap;

/// Compute histogram bins from raw data.
/// Returns (bin_edges, counts) where bin_edges has len = num_bins + 1.
pub fn compute_bins(data: &[f64], num_bins: usize) -> (Vec<f64>, Vec<usize>) {
    if data.is_empty() || num_bins == 0 {
        return (vec![], vec![]);
    }

    let mut dmin = f64::MAX;
    let mut dmax = f64::MIN;
    for &v in data {
        if v < dmin { dmin = v; }
        if v > dmax { dmax = v; }
    }

    // Handle case where all values are the same
    if (dmax - dmin).abs() < 1e-15 {
        dmax = dmin + 1.0;
    }

    let bin_width = (dmax - dmin) / num_bins as f64;
    let mut edges = Vec::with_capacity(num_bins + 1);
    for i in 0..=num_bins {
        edges.push(dmin + i as f64 * bin_width);
    }

    let mut counts = vec![0usize; num_bins];
    for &v in data {
        let mut idx = ((v - dmin) / bin_width) as usize;
        if idx >= num_bins {
            idx = num_bins - 1;
        }
        counts[idx] += 1;
    }

    (edges, counts)
}

pub struct Histogram {
    bar: Bar,
}

impl Histogram {
    pub fn new(data: &[f64], num_bins: usize, color: Color, alpha: f32, label: Option<String>) -> Self {
        let (edges, counts) = compute_bins(data, num_bins);

        // Bar centers and heights
        let mut x = Vec::new();
        let mut heights = Vec::new();
        let mut width = 0.8;

        if edges.len() >= 2 {
            width = edges[1] - edges[0];
            for i in 0..counts.len() {
                let center = (edges[i] + edges[i + 1]) / 2.0;
                x.push(center);
                heights.push(counts[i] as f64);
            }
        }

        let mut bar = Bar::new(x, heights, color);
        bar.width = width;
        bar.alpha = alpha;
        bar.label = label;

        Histogram { bar }
    }
}

impl Artist for Histogram {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        self.bar.draw(pixmap, transform);
    }

    fn draw_svg(&self, svg: &mut SvgRenderer, transform: &Transform) {
        self.bar.draw_svg(svg, transform);
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        self.bar.data_bounds()
    }

    fn legend_label(&self) -> Option<&str> {
        self.bar.legend_label()
    }

    fn legend_color(&self) -> Color {
        self.bar.legend_color()
    }
}
