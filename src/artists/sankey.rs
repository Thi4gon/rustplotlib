use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke};

/// A basic Sankey diagram showing flows as arrows of varying width.
///
/// Flows > 0 are inputs (entering from the left).
/// Flows < 0 are outputs (leaving to the right).
///
/// The diagram draws a central horizontal bar with input arrows on the left
/// and output arrows on the right, widths proportional to flow magnitude.
pub struct Sankey {
    pub flows: Vec<f64>,
    pub labels: Vec<String>,
    pub orientations: Vec<i32>, // -1=left, 0=down, 1=right
    pub color: Color,
    pub alpha: f32,
    pub label: Option<String>,
    pub zorder: i32,
}

impl Sankey {
    pub fn new(flows: Vec<f64>, labels: Vec<String>) -> Self {
        let n = flows.len();
        Self {
            flows,
            labels,
            orientations: vec![0; n],
            color: Color::new(31, 119, 180, 255),
            alpha: 0.7,
            label: None,
            zorder: 0,
        }
    }
}

impl Artist for Sankey {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.flows.is_empty() {
            return;
        }

        let ts = tiny_skia::Transform::identity();

        // Sum of inputs and outputs
        let total_in: f64 = self.flows.iter().filter(|&&f| f > 0.0).sum();
        let total_out: f64 = self.flows.iter().filter(|&&f| f < 0.0).map(|f| f.abs()).sum();
        let total = total_in.max(total_out).max(1.0);

        // The central bar spans data coords roughly [0.3, 0.7] in x, centered at y=0.5
        // Scale flows to fit nicely
        let bar_left = 0.3;
        let bar_right = 0.7;
        let bar_center_y = 0.5;
        let bar_half_height = 0.35; // max half height

        let mut fill_color = self.color;
        fill_color.a = (self.alpha * 255.0) as u8;

        let mut paint = Paint::default();
        paint.set_color(fill_color.to_tiny_skia());
        paint.anti_alias = true;

        let mut border_paint = Paint::default();
        border_paint.set_color(Color::new(0, 0, 0, 180).to_tiny_skia());
        border_paint.anti_alias = true;
        let mut stroke = Stroke::default();
        stroke.width = 0.8;

        // Draw central bar
        let bar_top = bar_center_y + bar_half_height * (total_in / total);
        let bar_bot = bar_center_y - bar_half_height * (total_in / total);

        let (px_bl, py_bl) = transform.transform_xy(bar_left, bar_bot);
        let (px_br, py_br) = transform.transform_xy(bar_right, bar_bot);
        let (px_tr, py_tr) = transform.transform_xy(bar_right, bar_top);
        let (px_tl, py_tl) = transform.transform_xy(bar_left, bar_top);

        let mut pb = PathBuilder::new();
        pb.move_to(px_bl, py_bl);
        pb.line_to(px_br, py_br);
        pb.line_to(px_tr, py_tr);
        pb.line_to(px_tl, py_tl);
        pb.close();
        if let Some(path) = pb.finish() {
            pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, ts, None);
            pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
        }

        // Draw input arrows (positive flows) on the left
        let mut y_offset = bar_bot;
        for (i, &flow) in self.flows.iter().enumerate() {
            if flow <= 0.0 { continue; }
            let arrow_height = (flow / total) * (bar_top - bar_bot);
            let arrow_top = y_offset + arrow_height;

            let arrow_color = Self::flow_color(i);
            let mut fc = arrow_color;
            fc.a = (self.alpha * 255.0) as u8;
            let mut ap = Paint::default();
            ap.set_color(fc.to_tiny_skia());
            ap.anti_alias = true;

            // Arrow from left edge to bar_left
            let (px0, py0) = transform.transform_xy(0.05, (y_offset + arrow_top) / 2.0);
            let (px1, py1) = transform.transform_xy(bar_left, y_offset);
            let (px2, py2) = transform.transform_xy(bar_left, arrow_top);

            let mut pb = PathBuilder::new();
            pb.move_to(px0, py0); // tip
            pb.line_to(px1, py1); // bottom of bar junction
            pb.line_to(px2, py2); // top of bar junction
            pb.close();
            if let Some(path) = pb.finish() {
                pixmap.fill_path(&path, &ap, tiny_skia::FillRule::Winding, ts, None);
                pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
            }

            y_offset = arrow_top;
        }

        // Draw output arrows (negative flows) on the right
        y_offset = bar_bot;
        for (i, &flow) in self.flows.iter().enumerate() {
            if flow >= 0.0 { continue; }
            let arrow_height = (flow.abs() / total) * (bar_top - bar_bot);
            let arrow_top = y_offset + arrow_height;

            let arrow_color = Self::flow_color(i);
            let mut fc = arrow_color;
            fc.a = (self.alpha * 255.0) as u8;
            let mut ap = Paint::default();
            ap.set_color(fc.to_tiny_skia());
            ap.anti_alias = true;

            let (px0, py0) = transform.transform_xy(0.95, (y_offset + arrow_top) / 2.0);
            let (px1, py1) = transform.transform_xy(bar_right, y_offset);
            let (px2, py2) = transform.transform_xy(bar_right, arrow_top);

            let mut pb = PathBuilder::new();
            pb.move_to(px1, py1);
            pb.line_to(px2, py2);
            pb.line_to(px0, py0);
            pb.close();
            if let Some(path) = pb.finish() {
                pixmap.fill_path(&path, &ap, tiny_skia::FillRule::Winding, ts, None);
                pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
            }

            y_offset = arrow_top;
        }

        // Draw labels
        // Labels are drawn as text annotations — handled at the Python level
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        (0.0, 1.0, 0.0, 1.0)
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

impl Sankey {
    /// Get a distinct color for each flow arrow.
    fn flow_color(idx: usize) -> Color {
        const COLORS: [(u8, u8, u8); 10] = [
            (31, 119, 180),
            (255, 127, 14),
            (44, 160, 44),
            (214, 39, 40),
            (148, 103, 189),
            (140, 86, 75),
            (227, 119, 194),
            (127, 127, 127),
            (188, 189, 34),
            (23, 190, 207),
        ];
        let (r, g, b) = COLORS[idx % COLORS.len()];
        Color::new(r, g, b, 255)
    }
}
