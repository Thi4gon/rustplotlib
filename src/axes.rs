use tiny_skia::{Paint, PathBuilder, Rect, Stroke, Pixmap};

use crate::artists::Artist;
use crate::artists::line2d::Line2D;
use crate::artists::scatter::Scatter;
use crate::artists::bar::Bar;
use crate::artists::hist::Histogram;
use crate::artists::image::Image;
use crate::artists::fill_between::FillBetween;
use crate::artists::step::{Step, StepWhere};
use crate::artists::pie::PieChart;
use crate::artists::legend::draw_legend;
use crate::artists::{LineStyle, MarkerStyle};
use crate::colors::Color;
use crate::text::{draw_text, TextAnchorX, TextAnchorY};
use crate::ticker::{compute_auto_ticks, format_tick_value};
use crate::transforms::Transform;

/// Tab10 color palette (same as matplotlib's default).
const TAB10: [(u8, u8, u8); 10] = [
    (31, 119, 180),   // blue
    (255, 127, 14),   // orange
    (44, 160, 44),    // green
    (214, 39, 40),    // red
    (148, 103, 189),  // purple
    (140, 86, 75),    // brown
    (227, 119, 194),  // pink
    (127, 127, 127),  // gray
    (188, 189, 34),   // olive
    (23, 190, 207),   // cyan
];

/// A text annotation drawn on the axes.
pub struct TextAnnotation {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub fontsize: f32,
    pub color: Color,
}

/// A reference line (axhline / axvline).
pub struct RefLine {
    pub horizontal: bool, // true = axhline, false = axvline
    pub value: f64,
    pub color: Color,
    pub linestyle: LineStyle,
    pub linewidth: f32,
    pub alpha: f32,
}

pub struct Axes {
    artists: Vec<Box<dyn Artist>>,
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub xlim: Option<(f64, f64)>,
    pub ylim: Option<(f64, f64)>,
    pub grid_visible: bool,
    pub grid_color: Color,
    pub grid_linewidth: f32,
    pub grid_alpha: f32,
    pub show_legend: bool,
    pub legend_loc: String,
    color_cycle_idx: usize,
    pub title_size: f32,
    pub label_size: f32,
    pub tick_size: f32,
    pub texts: Vec<TextAnnotation>,
    pub ref_lines: Vec<RefLine>,
}

impl Axes {
    pub fn new() -> Self {
        Axes {
            artists: Vec::new(),
            title: None,
            xlabel: None,
            ylabel: None,
            xlim: None,
            ylim: None,
            grid_visible: false,
            grid_color: Color::new(220, 220, 220, 255),
            grid_linewidth: 0.5,
            grid_alpha: 1.0,
            show_legend: false,
            legend_loc: "upper right".to_string(),
            color_cycle_idx: 0,
            title_size: 14.0,
            label_size: 12.0,
            tick_size: 10.0,
            texts: Vec::new(),
            ref_lines: Vec::new(),
        }
    }

    /// Get the next color from the tab10 cycle.
    pub fn next_color(&mut self) -> Color {
        let (r, g, b) = TAB10[self.color_cycle_idx % TAB10.len()];
        self.color_cycle_idx += 1;
        Color::new(r, g, b, 255)
    }

    /// Add a line plot.
    pub fn plot(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        color: Option<Color>,
        linewidth: Option<f32>,
        linestyle: Option<&str>,
        marker: Option<&str>,
        marker_size: Option<f32>,
        marker_every: Option<usize>,
        label: Option<String>,
        alpha: Option<f32>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut line = Line2D::new(x, y, c);
        if let Some(lw) = linewidth { line.linewidth = lw; }
        if let Some(ls) = linestyle { line.linestyle = LineStyle::from_str(ls); }
        if let Some(m) = marker { line.marker = MarkerStyle::from_str(m); }
        if let Some(ms) = marker_size {
            if ms <= 0.0 {
                line.marker = MarkerStyle::None;
            } else {
                line.marker_size = ms;
            }
        }
        if let Some(me) = marker_every { line.marker_every = me; }
        line.label = label;
        if let Some(a) = alpha { line.alpha = a; }
        self.artists.push(Box::new(line));
    }

    /// Add a scatter plot.
    pub fn scatter(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        color: Option<Color>,
        sizes: Option<Vec<f32>>,
        marker: Option<&str>,
        label: Option<String>,
        alpha: Option<f32>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut sc = Scatter::new(x, y, c);
        if let Some(s) = sizes { sc.sizes = s; }
        if let Some(m) = marker { sc.marker = MarkerStyle::from_str(m); }
        sc.label = label;
        if let Some(a) = alpha { sc.alpha = a; }
        self.artists.push(Box::new(sc));
    }

    /// Add a bar chart.
    pub fn bar(
        &mut self,
        x: Vec<f64>,
        heights: Vec<f64>,
        color: Option<Color>,
        width: Option<f64>,
        label: Option<String>,
        alpha: Option<f32>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut b = Bar::new(x, heights, c);
        if let Some(w) = width { b.width = w; }
        b.label = label;
        if let Some(a) = alpha { b.alpha = a; }
        self.artists.push(Box::new(b));
    }

    /// Add a histogram.
    pub fn hist(
        &mut self,
        data: &[f64],
        bins: usize,
        color: Option<Color>,
        alpha: Option<f32>,
        label: Option<String>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let a = alpha.unwrap_or(1.0);
        let h = Histogram::new(data, bins, c, a, label);
        self.artists.push(Box::new(h));
    }

    /// Add an image display.
    pub fn imshow(&mut self, data: Vec<Vec<f64>>, cmap: Option<String>) {
        let cm = cmap.unwrap_or_else(|| "viridis".to_string());
        let img = Image::new(data, cm);
        self.artists.push(Box::new(img));
    }

    /// Add a fill_between area.
    pub fn fill_between(
        &mut self,
        x: Vec<f64>,
        y1: Vec<f64>,
        y2: Vec<f64>,
        color: Option<Color>,
        alpha: Option<f32>,
        label: Option<String>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let a = alpha.unwrap_or(0.3);
        let mut fb = FillBetween::new(x, y1, y2, c, a);
        fb.label = label;
        self.artists.push(Box::new(fb));
    }

    /// Add a step plot.
    pub fn step(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        color: Option<Color>,
        linewidth: Option<f32>,
        linestyle: Option<&str>,
        label: Option<String>,
        alpha: Option<f32>,
        where_style: Option<&str>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut s = Step::new(x, y, c);
        if let Some(lw) = linewidth { s.linewidth = lw; }
        if let Some(ls) = linestyle { s.linestyle = LineStyle::from_str(ls); }
        s.label = label;
        if let Some(a) = alpha { s.alpha = a; }
        if let Some(ws) = where_style { s.where_style = StepWhere::from_str(ws); }
        self.artists.push(Box::new(s));
    }

    /// Add a pie chart.
    pub fn pie(
        &mut self,
        sizes: Vec<f64>,
        labels: Vec<String>,
        colors: Vec<Color>,
        start_angle: f32,
    ) {
        let pie_colors = if colors.is_empty() {
            PieChart::default_colors(sizes.len())
        } else {
            colors
        };
        let chart = PieChart::new(sizes, labels, pie_colors, start_angle);
        self.artists.push(Box::new(chart));
    }

    /// Add a horizontal reference line.
    pub fn axhline(
        &mut self,
        y: f64,
        color: Option<Color>,
        linestyle: &str,
        linewidth: f32,
        alpha: f32,
    ) {
        self.ref_lines.push(RefLine {
            horizontal: true,
            value: y,
            color: color.unwrap_or(Color::new(0, 0, 0, 255)),
            linestyle: LineStyle::from_str(linestyle),
            linewidth,
            alpha,
        });
    }

    /// Add a vertical reference line.
    pub fn axvline(
        &mut self,
        x: f64,
        color: Option<Color>,
        linestyle: &str,
        linewidth: f32,
        alpha: f32,
    ) {
        self.ref_lines.push(RefLine {
            horizontal: false,
            value: x,
            color: color.unwrap_or(Color::new(0, 0, 0, 255)),
            linestyle: LineStyle::from_str(linestyle),
            linewidth,
            alpha,
        });
    }

    /// Compute the combined data bounds from all artists with 5% margin.
    fn compute_bounds(&self) -> (f64, f64, f64, f64) {
        let mut xmin = f64::MAX;
        let mut xmax = f64::MIN;
        let mut ymin = f64::MAX;
        let mut ymax = f64::MIN;

        for artist in &self.artists {
            let (ax, bx, ay, by) = artist.data_bounds();
            if ax < xmin { xmin = ax; }
            if bx > xmax { xmax = bx; }
            if ay < ymin { ymin = ay; }
            if by > ymax { ymax = by; }
        }

        // Fallback if no artists
        if xmin >= xmax { xmin = 0.0; xmax = 1.0; }
        if ymin >= ymax { ymin = 0.0; ymax = 1.0; }

        // 5% margin
        let x_margin = (xmax - xmin) * 0.05;
        let y_margin = (ymax - ymin) * 0.05;
        xmin -= x_margin;
        xmax += x_margin;
        ymin -= y_margin;
        ymax += y_margin;

        // Override with user-specified limits
        let (xmin, xmax) = self.xlim.unwrap_or((xmin, xmax));
        let (ymin, ymax) = self.ylim.unwrap_or((ymin, ymax));

        (xmin, xmax, ymin, ymax)
    }

    /// Draw this axes and all its artists onto the pixmap.
    /// left, top, right, bottom are pixel coordinates of the plot area.
    pub fn draw(&self, pixmap: &mut Pixmap, left: f32, top: f32, right: f32, bottom: f32) {
        let (xmin, xmax, ymin, ymax) = self.compute_bounds();

        let transform = Transform::new(
            (xmin, xmax),
            (ymin, ymax),
            left as f64,
            right as f64,
            top as f64,
            bottom as f64,
        );

        let ts = tiny_skia::Transform::identity();

        // 1. Draw white background
        if let Some(rect) = Rect::from_xywh(left, top, right - left, bottom - top) {
            let mut bg_paint = Paint::default();
            bg_paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 255));
            pixmap.fill_rect(rect, &bg_paint, ts, None);
        }

        // 2. Draw grid if enabled
        if self.grid_visible {
            let x_ticks = compute_auto_ticks(xmin, xmax, 10);
            let y_ticks = compute_auto_ticks(ymin, ymax, 8);

            let mut grid_color = self.grid_color;
            grid_color.a = (self.grid_alpha * 255.0) as u8;
            let mut grid_paint = Paint::default();
            grid_paint.set_color(grid_color.to_tiny_skia());
            grid_paint.anti_alias = true;

            let mut grid_stroke = Stroke::default();
            grid_stroke.width = self.grid_linewidth;

            // Vertical grid lines
            for &tx in &x_ticks {
                let (px, _) = transform.transform_xy(tx, ymin);
                let mut pb = PathBuilder::new();
                pb.move_to(px, top);
                pb.line_to(px, bottom);
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &grid_paint, &grid_stroke, ts, None);
                }
            }

            // Horizontal grid lines
            for &ty in &y_ticks {
                let (_, py) = transform.transform_xy(xmin, ty);
                let mut pb = PathBuilder::new();
                pb.move_to(left, py);
                pb.line_to(right, py);
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &grid_paint, &grid_stroke, ts, None);
                }
            }
        }

        // 2b. Draw reference lines (axhline / axvline)
        for rl in &self.ref_lines {
            let mut rl_color = rl.color;
            rl_color.a = (rl.alpha * 255.0) as u8;
            let mut rl_paint = Paint::default();
            rl_paint.set_color(rl_color.to_tiny_skia());
            rl_paint.anti_alias = true;

            let mut rl_stroke = Stroke::default();
            rl_stroke.width = rl.linewidth;
            rl_stroke.dash = rl.linestyle.to_dash(rl.linewidth);

            let mut pb = PathBuilder::new();
            if rl.horizontal {
                let (_, py) = transform.transform_xy(xmin, rl.value);
                pb.move_to(left, py);
                pb.line_to(right, py);
            } else {
                let (px, _) = transform.transform_xy(rl.value, ymin);
                pb.move_to(px, top);
                pb.line_to(px, bottom);
            }
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &rl_paint, &rl_stroke, ts, None);
            }
        }

        // 3. Draw each artist
        for artist in &self.artists {
            artist.draw(pixmap, &transform);
        }

        // 4. Draw axes border
        if let Some(rect) = Rect::from_xywh(left, top, right - left, bottom - top) {
            let border_path = PathBuilder::from_rect(rect);
            let mut border_paint = Paint::default();
            border_paint.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 255));
            border_paint.anti_alias = true;
            let mut stroke = Stroke::default();
            stroke.width = 1.0;
            pixmap.stroke_path(&border_path, &border_paint, &stroke, ts, None);
        }

        // 5. Draw tick marks and labels
        let x_ticks = compute_auto_ticks(xmin, xmax, 10);
        let y_ticks = compute_auto_ticks(ymin, ymax, 8);
        let tick_len = 5.0_f32;
        let tick_color = Color::new(0, 0, 0, 255);

        let mut tick_paint = Paint::default();
        tick_paint.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 255));
        tick_paint.anti_alias = true;

        let mut tick_stroke = Stroke::default();
        tick_stroke.width = 1.0;

        // X ticks
        for &tx in &x_ticks {
            let (px, _) = transform.transform_xy(tx, ymin);
            if px < left || px > right { continue; }

            // Tick mark
            let mut pb = PathBuilder::new();
            pb.move_to(px, bottom);
            pb.line_to(px, bottom + tick_len);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &tick_paint, &tick_stroke, ts, None);
            }

            // Tick label
            let label = format_tick_value(tx);
            draw_text(
                pixmap,
                &label,
                px,
                bottom + tick_len + 2.0,
                self.tick_size,
                tick_color,
                TextAnchorX::Center,
                TextAnchorY::Top,
                0.0,
            );
        }

        // Y ticks
        for &ty in &y_ticks {
            let (_, py) = transform.transform_xy(xmin, ty);
            if py < top || py > bottom { continue; }

            // Tick mark
            let mut pb = PathBuilder::new();
            pb.move_to(left, py);
            pb.line_to(left - tick_len, py);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &tick_paint, &tick_stroke, ts, None);
            }

            // Tick label
            let label = format_tick_value(ty);
            draw_text(
                pixmap,
                &label,
                left - tick_len - 3.0,
                py,
                self.tick_size,
                tick_color,
                TextAnchorX::Right,
                TextAnchorY::Center,
                0.0,
            );
        }

        // 6. Draw title
        if let Some(ref title) = self.title {
            let cx = (left + right) / 2.0;
            draw_text(
                pixmap,
                title,
                cx,
                top - 8.0,
                self.title_size,
                tick_color,
                TextAnchorX::Center,
                TextAnchorY::Bottom,
                0.0,
            );
        }

        // 7. Draw xlabel
        if let Some(ref xlabel) = self.xlabel {
            let cx = (left + right) / 2.0;
            draw_text(
                pixmap,
                xlabel,
                cx,
                bottom + tick_len + self.tick_size + 10.0,
                self.label_size,
                tick_color,
                TextAnchorX::Center,
                TextAnchorY::Top,
                0.0,
            );
        }

        // 8. Draw ylabel (rotated 90 degrees)
        if let Some(ref ylabel) = self.ylabel {
            let cy = (top + bottom) / 2.0;
            draw_text(
                pixmap,
                ylabel,
                left - tick_len - 35.0,
                cy,
                self.label_size,
                tick_color,
                TextAnchorX::Center,
                TextAnchorY::Center,
                std::f32::consts::FRAC_PI_2,
            );
        }

        // 9. Draw text annotations
        for ann in &self.texts {
            let (px, py) = transform.transform_xy(ann.x, ann.y);
            draw_text(
                pixmap,
                &ann.text,
                px,
                py,
                ann.fontsize,
                ann.color,
                TextAnchorX::Left,
                TextAnchorY::Center,
                0.0,
            );
        }

        // 10. Draw legend if enabled
        if self.show_legend {
            let mut entries = Vec::new();
            for artist in &self.artists {
                if let Some(entry) = artist.legend_entry() {
                    entries.push(entry);
                }
            }
            if !entries.is_empty() {
                let legend_w = 120.0_f32;
                let legend_margin = 10.0_f32;
                let (legend_x, legend_y) = match self.legend_loc.as_str() {
                    "upper left" => (left + legend_margin, top + legend_margin),
                    "lower right" => {
                        let entry_count = entries.len() as f32;
                        let legend_h = 12.0 + entry_count * 15.0;
                        (right - legend_margin - legend_w, bottom - legend_margin - legend_h)
                    }
                    "lower left" => {
                        let entry_count = entries.len() as f32;
                        let legend_h = 12.0 + entry_count * 15.0;
                        (left + legend_margin, bottom - legend_margin - legend_h)
                    }
                    _ => {
                        // "upper right" (default)
                        (right - legend_margin - legend_w, top + legend_margin)
                    }
                };
                draw_legend(pixmap, &entries, legend_x, legend_y);
            }
        }
    }
}
