use tiny_skia::{Paint, PathBuilder, Rect, Stroke, Pixmap};

use crate::artists::Artist;
use crate::artists::line2d::Line2D;
use crate::artists::scatter::Scatter;
use crate::artists::bar::Bar;
use crate::artists::hist::Histogram;
use crate::artists::image::Image;
use crate::artists::fill_between::FillBetween;
use crate::artists::fill_betweenx::FillBetweenX;
use crate::artists::violin::ViolinPlot;
use crate::artists::step::{Step, StepWhere};
use crate::artists::pie::PieChart;
use crate::artists::errorbar::ErrorBar;
use crate::artists::barh::BarH;
use crate::artists::boxplot::BoxPlot;
use crate::artists::stem::Stem;
use crate::artists::contour::Contour;
use crate::artists::hexbin::HexBin;
use crate::artists::patches::Patch;
use crate::artists::legend::draw_legend;
use crate::artists::{LineStyle, MarkerStyle};
use crate::colors::Color;
use crate::svg_renderer::{SvgRenderer, color_to_svg, color_alpha, linestyle_to_dash};
use crate::text::{draw_text, TextAnchorX, TextAnchorY};
use crate::ticker::{compute_auto_ticks, compute_log_ticks, format_tick_value, format_log_tick_value};
use crate::transforms::Transform;

/// Axis scale type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxisScale {
    Linear,
    Log,
}

/// Aspect ratio mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AspectRatio {
    Auto,
    Equal,
}

/// Tick direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TickDirection {
    Out,
    In,
    InOut,
}

impl TickDirection {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "in" => TickDirection::In,
            "inout" => TickDirection::InOut,
            _ => TickDirection::Out,
        }
    }
}

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

/// An annotation with arrow pointing from text to a data point.
pub struct Annotation {
    pub text: String,
    pub xy: (f64, f64),          // point being annotated
    pub xytext: (f64, f64),      // where the text goes
    pub fontsize: f32,
    pub color: Color,
    pub arrow_color: Color,
    pub arrow_width: f32,
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

/// A shaded region (axhspan / axvspan).
pub struct SpanRegion {
    pub horizontal: bool, // true = axhspan (horizontal band), false = axvspan (vertical band)
    pub vmin: f64,
    pub vmax: f64,
    pub color: Color,
    pub alpha: f32,
}

/// A bounded reference line (hlines / vlines).
pub struct BoundedRefLine {
    pub horizontal: bool, // true = hline, false = vline
    pub value: f64,
    pub bound_min: f64,
    pub bound_max: f64,
    pub color: Color,
    pub linestyle: LineStyle,
    pub linewidth: f32,
    pub alpha: f32,
}

/// Table data to draw on axes.
pub struct TableData {
    pub cell_text: Vec<Vec<String>>,
    pub col_labels: Option<Vec<String>>,
    pub row_labels: Option<Vec<String>>,
    pub loc: String,
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
    pub x_scale: AxisScale,
    pub y_scale: AxisScale,
    pub annotations: Vec<Annotation>,
    pub axes_visible: bool,
    pub custom_xticks: Option<Vec<f64>>,
    pub custom_yticks: Option<Vec<f64>>,
    pub custom_xtick_labels: Option<Vec<String>>,
    pub custom_ytick_labels: Option<Vec<String>>,
    pub aspect: AspectRatio,
    pub invert_x: bool,
    pub invert_y: bool,
    pub span_regions: Vec<SpanRegion>,
    pub polar: bool,
    pub twin_axes: Option<Box<Axes>>,
    // Style fields
    pub bg_color: Color,
    pub tick_color: Color,
    pub text_color: Color,
    // Spine customization
    pub spine_visible: [bool; 4], // [top, right, bottom, left]
    pub spine_color: Color,
    pub spine_linewidth: f32,
    // Tick params
    pub tick_direction: TickDirection,
    pub tick_length: f32,
    pub tick_width: f32,
    pub tick_label_size: f32,
    // Colorbar
    pub show_colorbar: bool,
    pub colorbar_cmap: String,
    pub colorbar_vmin: f64,
    pub colorbar_vmax: f64,
    // Grid which (Major/Minor/Both)
    pub grid_which: GridWhich,
    // Bounded reference lines (hlines / vlines)
    pub bounded_ref_lines: Vec<BoundedRefLine>,
    // Table data
    pub table_data: Option<TableData>,
}

/// Which grid lines to show.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridWhich {
    Major,
    Minor,
    Both,
}

impl GridWhich {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "minor" => GridWhich::Minor,
            "both" => GridWhich::Both,
            _ => GridWhich::Major,
        }
    }
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
            x_scale: AxisScale::Linear,
            y_scale: AxisScale::Linear,
            annotations: Vec::new(),
            axes_visible: true,
            custom_xticks: None,
            custom_yticks: None,
            custom_xtick_labels: None,
            custom_ytick_labels: None,
            aspect: AspectRatio::Auto,
            invert_x: false,
            invert_y: false,
            span_regions: Vec::new(),
            polar: false,
            twin_axes: None,
            bg_color: Color::new(255, 255, 255, 255),
            tick_color: Color::new(0, 0, 0, 255),
            text_color: Color::new(0, 0, 0, 255),
            spine_visible: [true, true, true, true],
            spine_color: Color::new(0, 0, 0, 255),
            spine_linewidth: 1.0,
            tick_direction: TickDirection::Out,
            tick_length: 5.0,
            tick_width: 1.0,
            tick_label_size: 10.0,
            show_colorbar: false,
            colorbar_cmap: "viridis".to_string(),
            colorbar_vmin: 0.0,
            colorbar_vmax: 1.0,
            grid_which: GridWhich::Major,
            bounded_ref_lines: Vec::new(),
            table_data: None,
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
        bottom: Option<f64>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut b = Bar::new(x, heights, c);
        if let Some(w) = width { b.width = w; }
        b.label = label;
        if let Some(a) = alpha { b.alpha = a; }
        if let Some(bot) = bottom { b.bottom = bot; }
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

    /// Set the X axis scale ("linear" or "log").
    pub fn set_xscale(&mut self, scale: &str) {
        self.x_scale = match scale.to_lowercase().as_str() {
            "log" => AxisScale::Log,
            _ => AxisScale::Linear,
        };
    }

    /// Set the Y axis scale ("linear" or "log").
    pub fn set_yscale(&mut self, scale: &str) {
        self.y_scale = match scale.to_lowercase().as_str() {
            "log" => AxisScale::Log,
            _ => AxisScale::Linear,
        };
    }

    /// Add an error bar plot.
    pub fn errorbar(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        yerr: Option<Vec<f64>>,
        xerr: Option<Vec<f64>>,
        color: Option<Color>,
        linewidth: Option<f32>,
        capsize: Option<f32>,
        marker: Option<&str>,
        marker_size: Option<f32>,
        label: Option<String>,
        alpha: Option<f32>,
        linestyle: Option<&str>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut eb = ErrorBar::new(x, y, c);
        eb.yerr = yerr;
        eb.xerr = xerr;
        if let Some(lw) = linewidth { eb.linewidth = lw; }
        if let Some(cs) = capsize { eb.capsize = cs; }
        if let Some(m) = marker { eb.marker = MarkerStyle::from_str(m); }
        if let Some(ms) = marker_size { eb.marker_size = ms; }
        eb.label = label;
        if let Some(a) = alpha { eb.alpha = a; }
        if let Some(ls) = linestyle { eb.linestyle = LineStyle::from_str(ls); }
        self.artists.push(Box::new(eb));
    }

    /// Add a horizontal bar chart.
    pub fn barh(
        &mut self,
        y: Vec<f64>,
        widths: Vec<f64>,
        color: Option<Color>,
        height: Option<f64>,
        label: Option<String>,
        alpha: Option<f32>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut b = BarH::new(y, widths, c);
        if let Some(h) = height { b.height = h; }
        b.label = label;
        if let Some(a) = alpha { b.alpha = a; }
        self.artists.push(Box::new(b));
    }

    /// Add a box plot.
    pub fn boxplot(
        &mut self,
        data: Vec<Vec<f64>>,
        positions: Option<Vec<f64>>,
        widths: Option<f64>,
        color: Option<Color>,
        median_color: Option<Color>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mc = median_color.unwrap_or(Color::new(255, 127, 14, 255));
        let pos = positions.unwrap_or_else(|| (1..=data.len()).map(|i| i as f64).collect());
        let w = widths.unwrap_or(0.5);
        let bp = BoxPlot::new(data, pos, w, c, mc);
        self.artists.push(Box::new(bp));
    }

    /// Add a stem plot.
    pub fn stem(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        color: Option<Color>,
        linewidth: Option<f32>,
        marker: Option<&str>,
        marker_size: Option<f32>,
        label: Option<String>,
        baseline: Option<f64>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let mut s = Stem::new(x, y, c);
        if let Some(lw) = linewidth { s.linewidth = lw; }
        if let Some(m) = marker { s.marker = MarkerStyle::from_str(m); }
        if let Some(ms) = marker_size { s.marker_size = ms; }
        s.label = label;
        if let Some(bl) = baseline { s.baseline = bl; }
        self.artists.push(Box::new(s));
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
        // Polar mode: use dedicated polar drawing
        if self.polar {
            self.draw_polar(pixmap, left, top, right, bottom);
            return;
        }

        let (mut xmin, mut xmax, mut ymin, mut ymax) = self.compute_bounds();

        // Handle axis inversion
        if self.invert_x {
            std::mem::swap(&mut xmin, &mut xmax);
        }
        if self.invert_y {
            std::mem::swap(&mut ymin, &mut ymax);
        }

        let log_x = self.x_scale == AxisScale::Log;
        let log_y = self.y_scale == AxisScale::Log;

        // For log scale, compute ticks in data space before transforming bounds
        let x_ticks_data: Vec<f64>;
        let y_ticks_data: Vec<f64>;

        // Use absolute values for log tick computation (handle inverted)
        let (xmin_abs, xmax_abs) = if xmin <= xmax { (xmin, xmax) } else { (xmax, xmin) };
        let (ymin_abs, ymax_abs) = if ymin <= ymax { (ymin, ymax) } else { (ymax, ymin) };

        if log_x {
            xmin = if xmin > 0.0 { xmin } else { 1e-15_f64.copysign(1.0) };
            xmax = if xmax > 0.0 { xmax } else { 1e-15_f64.copysign(1.0) };
            x_ticks_data = compute_log_ticks(xmin_abs.max(1e-15), xmax_abs.max(1e-15));
        } else {
            x_ticks_data = Vec::new(); // will compute later
        }
        if log_y {
            ymin = if ymin > 0.0 { ymin } else { 1e-15_f64.copysign(1.0) };
            ymax = if ymax > 0.0 { ymax } else { 1e-15_f64.copysign(1.0) };
            y_ticks_data = compute_log_ticks(ymin_abs.max(1e-15), ymax_abs.max(1e-15));
        } else {
            y_ticks_data = Vec::new(); // will compute later
        }

        // Build data bounds in log space if needed
        let (dxmin, dxmax) = if log_x {
            (xmin.max(1e-15).log10(), xmax.max(1e-15).log10())
        } else {
            (xmin, xmax)
        };
        let (dymin, dymax) = if log_y {
            (ymin.max(1e-15).log10(), ymax.max(1e-15).log10())
        } else {
            (ymin, ymax)
        };

        // Handle equal aspect ratio: adjust plot area so 1 data unit = 1 data unit in pixels
        let (left, top, right, bottom) = if self.aspect == AspectRatio::Equal {
            let data_w = (dxmax - dxmin).abs();
            let data_h = (dymax - dymin).abs();
            let pixel_w = right - left;
            let pixel_h = bottom - top;

            if data_w > 0.0 && data_h > 0.0 {
                let data_aspect = data_w / data_h;
                let pixel_aspect = pixel_w as f64 / pixel_h as f64;

                if data_aspect > pixel_aspect {
                    // Data is wider than pixel area — shrink height
                    let new_pixel_h = pixel_w as f64 / data_aspect;
                    let offset = (pixel_h as f64 - new_pixel_h) / 2.0;
                    (left, top + offset as f32, right, bottom - offset as f32)
                } else {
                    // Data is taller than pixel area — shrink width
                    let new_pixel_w = pixel_h as f64 * data_aspect;
                    let offset = (pixel_w as f64 - new_pixel_w) / 2.0;
                    (left + offset as f32, top, right - offset as f32, bottom)
                }
            } else {
                (left, top, right, bottom)
            }
        } else {
            (left, top, right, bottom)
        };

        let transform = Transform::new(
            (dxmin, dxmax),
            (dymin, dymax),
            left as f64,
            right as f64,
            top as f64,
            bottom as f64,
            log_x,
            log_y,
        );

        let ts = tiny_skia::Transform::identity();

        // Use xmin_abs/ymin_abs for tick computations (always lo < hi)
        let tick_xmin = xmin_abs;
        let tick_xmax = xmax_abs;
        let tick_ymin = ymin_abs;
        let tick_ymax = ymax_abs;

        // 1. Draw background
        if self.axes_visible {
            if let Some(rect) = Rect::from_xywh(left, top, right - left, bottom - top) {
                let mut bg_paint = Paint::default();
                bg_paint.set_color(self.bg_color.to_tiny_skia());
                pixmap.fill_rect(rect, &bg_paint, ts, None);
            }
        }

        // 2. Draw grid if enabled
        if self.grid_visible && self.axes_visible {
            let x_ticks: Vec<f64> = if log_x { x_ticks_data.clone() } else {
                self.custom_xticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_xmin, tick_xmax, 10))
            };
            let y_ticks: Vec<f64> = if log_y { y_ticks_data.clone() } else {
                self.custom_yticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_ymin, tick_ymax, 8))
            };

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

        // 2c. Draw span regions (axhspan / axvspan)
        for span in &self.span_regions {
            let mut span_color = span.color;
            span_color.a = (span.alpha * 255.0) as u8;
            let mut span_paint = Paint::default();
            span_paint.set_color(span_color.to_tiny_skia());
            span_paint.anti_alias = true;

            let (sx, sy, sw, sh) = if span.horizontal {
                // Horizontal band: full width, y from vmin to vmax
                let (_, py_min) = transform.transform_xy(xmin, span.vmin);
                let (_, py_max) = transform.transform_xy(xmin, span.vmax);
                let y_top = py_min.min(py_max);
                let y_bot = py_min.max(py_max);
                (left, y_top.max(top), right - left, (y_bot.min(bottom) - y_top.max(top)).max(0.0))
            } else {
                // Vertical band: full height, x from vmin to vmax
                let (px_min, _) = transform.transform_xy(span.vmin, ymin);
                let (px_max, _) = transform.transform_xy(span.vmax, ymin);
                let x_left = px_min.min(px_max);
                let x_right = px_min.max(px_max);
                (x_left.max(left), top, (x_right.min(right) - x_left.max(left)).max(0.0), bottom - top)
            };

            if sw > 0.0 && sh > 0.0 {
                if let Some(rect) = Rect::from_xywh(sx, sy, sw, sh) {
                    pixmap.fill_rect(rect, &span_paint, ts, None);
                }
            }
        }

        // 2d. Draw bounded reference lines (hlines / vlines)
        for brl in &self.bounded_ref_lines {
            let mut brl_color = brl.color;
            brl_color.a = (brl.alpha * 255.0) as u8;
            let mut brl_paint = Paint::default();
            brl_paint.set_color(brl_color.to_tiny_skia());
            brl_paint.anti_alias = true;

            let mut brl_stroke = Stroke::default();
            brl_stroke.width = brl.linewidth;
            brl_stroke.dash = brl.linestyle.to_dash(brl.linewidth);

            let mut pb = PathBuilder::new();
            if brl.horizontal {
                let (px_min, py) = transform.transform_xy(brl.bound_min, brl.value);
                let (px_max, _) = transform.transform_xy(brl.bound_max, brl.value);
                pb.move_to(px_min, py);
                pb.line_to(px_max, py);
            } else {
                let (px, py_min) = transform.transform_xy(brl.value, brl.bound_min);
                let (_, py_max) = transform.transform_xy(brl.value, brl.bound_max);
                pb.move_to(px, py_min);
                pb.line_to(px, py_max);
            }
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &brl_paint, &brl_stroke, ts, None);
            }
        }

        // 3. Draw each artist
        for artist in &self.artists {
            artist.draw(pixmap, &transform);
        }

        // 3b. Draw annotations (arrow + text)
        for ann in &self.annotations {
            let (target_px, target_py) = transform.transform_xy(ann.xy.0, ann.xy.1);
            let (text_px, text_py) = transform.transform_xy(ann.xytext.0, ann.xytext.1);

            // Draw arrow line from text to target point
            let mut arrow_paint = Paint::default();
            arrow_paint.set_color(ann.arrow_color.to_tiny_skia());
            arrow_paint.anti_alias = true;

            let mut arrow_stroke = Stroke::default();
            arrow_stroke.width = ann.arrow_width;

            let mut pb = PathBuilder::new();
            pb.move_to(text_px, text_py);
            pb.line_to(target_px, target_py);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &arrow_paint, &arrow_stroke, ts, None);
            }

            // Draw a small arrowhead at the target point
            let dx = target_px - text_px;
            let dy = target_py - text_py;
            let len = (dx * dx + dy * dy).sqrt();
            if len > 1.0 {
                let ux = dx / len;
                let uy = dy / len;
                let head_size = 6.0_f32;
                // Perpendicular
                let px_perp = -uy;
                let py_perp = ux;
                let base_x = target_px - ux * head_size;
                let base_y = target_py - uy * head_size;

                let mut pb = PathBuilder::new();
                pb.move_to(target_px, target_py);
                pb.line_to(base_x + px_perp * head_size * 0.4, base_y + py_perp * head_size * 0.4);
                pb.line_to(base_x - px_perp * head_size * 0.4, base_y - py_perp * head_size * 0.4);
                pb.close();
                if let Some(path) = pb.finish() {
                    let mut fill_paint = Paint::default();
                    fill_paint.set_color(ann.arrow_color.to_tiny_skia());
                    fill_paint.anti_alias = true;
                    pixmap.fill_path(&path, &fill_paint, tiny_skia::FillRule::Winding, ts, None);
                }
            }

            // Draw text at xytext position
            draw_text(
                pixmap,
                &ann.text,
                text_px,
                text_py - 4.0, // slight offset above the arrow start
                ann.fontsize,
                ann.color,
                TextAnchorX::Center,
                TextAnchorY::Bottom,
                0.0,
            );
        }

        if self.axes_visible {
            // 4. Draw spines (individual axis borders)
            {
                let mut spine_paint = Paint::default();
                spine_paint.set_color(self.spine_color.to_tiny_skia());
                spine_paint.anti_alias = true;
                let mut spine_stroke = Stroke::default();
                spine_stroke.width = self.spine_linewidth;

                // bottom spine
                if self.spine_visible[2] {
                    let mut pb = PathBuilder::new();
                    pb.move_to(left, bottom);
                    pb.line_to(right, bottom);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &spine_paint, &spine_stroke, ts, None);
                    }
                }
                // top spine
                if self.spine_visible[0] {
                    let mut pb = PathBuilder::new();
                    pb.move_to(left, top);
                    pb.line_to(right, top);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &spine_paint, &spine_stroke, ts, None);
                    }
                }
                // left spine
                if self.spine_visible[3] {
                    let mut pb = PathBuilder::new();
                    pb.move_to(left, top);
                    pb.line_to(left, bottom);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &spine_paint, &spine_stroke, ts, None);
                    }
                }
                // right spine
                if self.spine_visible[1] {
                    let mut pb = PathBuilder::new();
                    pb.move_to(right, top);
                    pb.line_to(right, bottom);
                    if let Some(path) = pb.finish() {
                        pixmap.stroke_path(&path, &spine_paint, &spine_stroke, ts, None);
                    }
                }
            }

            // 5. Draw tick marks and labels
            let x_ticks: Vec<f64> = if log_x { x_ticks_data.clone() } else {
                self.custom_xticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_xmin, tick_xmax, 10))
            };
            let y_ticks: Vec<f64> = if log_y { y_ticks_data.clone() } else {
                self.custom_yticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_ymin, tick_ymax, 8))
            };
            let tick_len = self.tick_length;
            let tick_color = self.tick_color;

            let mut tick_paint = Paint::default();
            tick_paint.set_color(tick_color.to_tiny_skia());
            tick_paint.anti_alias = true;

            let mut tick_stroke = Stroke::default();
            tick_stroke.width = self.tick_width;

            // Compute tick offsets based on direction
            let (tick_out, tick_in) = match self.tick_direction {
                TickDirection::Out => (tick_len, 0.0_f32),
                TickDirection::In => (0.0_f32, tick_len),
                TickDirection::InOut => (tick_len, tick_len),
            };

            // X ticks
            for (i, &tx) in x_ticks.iter().enumerate() {
                let (px, _) = transform.transform_xy(tx, ymin);
                if px < left || px > right { continue; }

                // Tick mark
                let mut pb = PathBuilder::new();
                pb.move_to(px, bottom - tick_in);
                pb.line_to(px, bottom + tick_out);
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &tick_paint, &tick_stroke, ts, None);
                }

                // Tick label: use custom labels if available, otherwise format value
                let label = if let Some(ref labels) = self.custom_xtick_labels {
                    labels.get(i).cloned().unwrap_or_default()
                } else if log_x {
                    format_log_tick_value(tx)
                } else {
                    format_tick_value(tx)
                };
                draw_text(
                    pixmap,
                    &label,
                    px,
                    bottom + tick_out + 2.0,
                    self.tick_label_size,
                    tick_color,
                    TextAnchorX::Center,
                    TextAnchorY::Top,
                    0.0,
                );
            }

            // Y ticks
            for (i, &ty) in y_ticks.iter().enumerate() {
                let (_, py) = transform.transform_xy(xmin, ty);
                if py < top || py > bottom { continue; }

                // Tick mark
                let mut pb = PathBuilder::new();
                pb.move_to(left + tick_in, py);
                pb.line_to(left - tick_out, py);
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &tick_paint, &tick_stroke, ts, None);
                }

                // Tick label: use custom labels if available, otherwise format value
                let label = if let Some(ref labels) = self.custom_ytick_labels {
                    labels.get(i).cloned().unwrap_or_default()
                } else if log_y {
                    format_log_tick_value(ty)
                } else {
                    format_tick_value(ty)
                };
                draw_text(
                    pixmap,
                    &label,
                    left - tick_out - 3.0,
                    py,
                    self.tick_label_size,
                    tick_color,
                    TextAnchorX::Right,
                    TextAnchorY::Center,
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
                    bottom + tick_out + self.tick_label_size + 10.0,
                    self.label_size,
                    self.text_color,
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
                    left - tick_out - 35.0,
                    cy,
                    self.label_size,
                    self.text_color,
                    TextAnchorX::Center,
                    TextAnchorY::Center,
                    -std::f32::consts::FRAC_PI_2,
                );
            }
        }

        // 6. Draw title (always, even with axes off)
        if let Some(ref title) = self.title {
            let cx = (left + right) / 2.0;
            draw_text(
                pixmap,
                title,
                cx,
                top - 8.0,
                self.title_size,
                self.text_color,
                TextAnchorX::Center,
                TextAnchorY::Bottom,
                0.0,
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

        // 11. Draw twin axes (twinx) if present
        if let Some(ref twin) = self.twin_axes {
            twin.draw_as_twin(pixmap, left, top, right, bottom);
        }

        // 12. Draw colorbar if enabled
        if self.show_colorbar {
            self.draw_colorbar(pixmap, right, top, bottom);
        }

        // 13. Draw table if present
        if let Some(ref table) = self.table_data {
            self.draw_table(pixmap, table, left, top, right, bottom);
        }
    }

    /// Draw this axes as native SVG elements.
    pub fn draw_svg(&self, svg: &mut SvgRenderer, left: f32, top: f32, right: f32, bottom: f32) {
        // Polar mode: skip SVG for now (complex, rare)
        if self.polar {
            return;
        }

        let (mut xmin, mut xmax, mut ymin, mut ymax) = self.compute_bounds();

        // Handle axis inversion
        if self.invert_x {
            std::mem::swap(&mut xmin, &mut xmax);
        }
        if self.invert_y {
            std::mem::swap(&mut ymin, &mut ymax);
        }

        let log_x = self.x_scale == AxisScale::Log;
        let log_y = self.y_scale == AxisScale::Log;

        let x_ticks_data: Vec<f64>;
        let y_ticks_data: Vec<f64>;

        let (xmin_abs, xmax_abs) = if xmin <= xmax { (xmin, xmax) } else { (xmax, xmin) };
        let (ymin_abs, ymax_abs) = if ymin <= ymax { (ymin, ymax) } else { (ymax, ymin) };

        if log_x {
            xmin = if xmin > 0.0 { xmin } else { 1e-15_f64.copysign(1.0) };
            xmax = if xmax > 0.0 { xmax } else { 1e-15_f64.copysign(1.0) };
            x_ticks_data = compute_log_ticks(xmin_abs.max(1e-15), xmax_abs.max(1e-15));
        } else {
            x_ticks_data = Vec::new();
        }
        if log_y {
            ymin = if ymin > 0.0 { ymin } else { 1e-15_f64.copysign(1.0) };
            ymax = if ymax > 0.0 { ymax } else { 1e-15_f64.copysign(1.0) };
            y_ticks_data = compute_log_ticks(ymin_abs.max(1e-15), ymax_abs.max(1e-15));
        } else {
            y_ticks_data = Vec::new();
        }

        let (dxmin, dxmax) = if log_x {
            (xmin.max(1e-15).log10(), xmax.max(1e-15).log10())
        } else {
            (xmin, xmax)
        };
        let (dymin, dymax) = if log_y {
            (ymin.max(1e-15).log10(), ymax.max(1e-15).log10())
        } else {
            (ymin, ymax)
        };

        // Handle equal aspect ratio
        let (left, top, right, bottom) = if self.aspect == AspectRatio::Equal {
            let data_w = (dxmax - dxmin).abs();
            let data_h = (dymax - dymin).abs();
            let pixel_w = right - left;
            let pixel_h = bottom - top;

            if data_w > 0.0 && data_h > 0.0 {
                let data_aspect = data_w / data_h;
                let pixel_aspect = pixel_w as f64 / pixel_h as f64;

                if data_aspect > pixel_aspect {
                    let new_pixel_h = pixel_w as f64 / data_aspect;
                    let offset = (pixel_h as f64 - new_pixel_h) / 2.0;
                    (left, top + offset as f32, right, bottom - offset as f32)
                } else {
                    let new_pixel_w = pixel_h as f64 * data_aspect;
                    let offset = (pixel_w as f64 - new_pixel_w) / 2.0;
                    (left + offset as f32, top, right - offset as f32, bottom)
                }
            } else {
                (left, top, right, bottom)
            }
        } else {
            (left, top, right, bottom)
        };

        let transform = Transform::new(
            (dxmin, dxmax),
            (dymin, dymax),
            left as f64,
            right as f64,
            top as f64,
            bottom as f64,
            log_x,
            log_y,
        );

        let tick_xmin = xmin_abs;
        let tick_xmax = xmax_abs;
        let tick_ymin = ymin_abs;
        let tick_ymax = ymax_abs;

        // 1. Background rect
        if self.axes_visible {
            let bg_str = color_to_svg(&self.bg_color);
            svg.add_rect(left, top, right - left, bottom - top, &bg_str, "none", 0.0, 1.0);
        }

        // 2. Grid lines
        if self.grid_visible && self.axes_visible {
            let x_ticks: Vec<f64> = if log_x {
                x_ticks_data.clone()
            } else {
                self.custom_xticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_xmin, tick_xmax, 10))
            };
            let y_ticks: Vec<f64> = if log_y {
                y_ticks_data.clone()
            } else {
                self.custom_yticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_ymin, tick_ymax, 8))
            };

            let mut grid_color = self.grid_color;
            grid_color.a = (self.grid_alpha * 255.0) as u8;
            let grid_str = color_to_svg(&grid_color);

            for &tx in &x_ticks {
                let (px, _) = transform.transform_xy(tx, ymin);
                svg.add_line(px, top, px, bottom, &grid_str, self.grid_linewidth, None, self.grid_alpha);
            }

            for &ty in &y_ticks {
                let (_, py) = transform.transform_xy(xmin, ty);
                svg.add_line(left, py, right, py, &grid_str, self.grid_linewidth, None, self.grid_alpha);
            }
        }

        // 2b. Reference lines (axhline / axvline)
        for rl in &self.ref_lines {
            let mut rl_color = rl.color;
            rl_color.a = (rl.alpha * 255.0) as u8;
            let rl_str = color_to_svg(&rl_color);
            let dash = linestyle_to_dash(&rl.linestyle, rl.linewidth);
            let dash_ref = dash.as_deref();

            if rl.horizontal {
                let (_, py) = transform.transform_xy(xmin, rl.value);
                svg.add_line(left, py, right, py, &rl_str, rl.linewidth, dash_ref, rl.alpha);
            } else {
                let (px, _) = transform.transform_xy(rl.value, ymin);
                svg.add_line(px, top, px, bottom, &rl_str, rl.linewidth, dash_ref, rl.alpha);
            }
        }

        // 2c. Span regions
        for span in &self.span_regions {
            let mut span_color = span.color;
            span_color.a = (span.alpha * 255.0) as u8;
            let span_str = color_to_svg(&span_color);

            let (sx, sy, sw, sh) = if span.horizontal {
                let (_, py_min) = transform.transform_xy(xmin, span.vmin);
                let (_, py_max) = transform.transform_xy(xmin, span.vmax);
                let y_top = py_min.min(py_max);
                let y_bot = py_min.max(py_max);
                (left, y_top.max(top), right - left, (y_bot.min(bottom) - y_top.max(top)).max(0.0))
            } else {
                let (px_min, _) = transform.transform_xy(span.vmin, ymin);
                let (px_max, _) = transform.transform_xy(span.vmax, ymin);
                let x_left = px_min.min(px_max);
                let x_right = px_min.max(px_max);
                (x_left.max(left), top, (x_right.min(right) - x_left.max(left)).max(0.0), bottom - top)
            };

            if sw > 0.0 && sh > 0.0 {
                svg.add_rect(sx, sy, sw, sh, &span_str, "none", 0.0, span.alpha);
            }
        }

        // Add clip path for the plot area
        svg.add_clip_rect("plot-area", left, top, right - left, bottom - top);
        svg.begin_group(Some("plot-area"));

        // 3. Draw each artist
        for artist in &self.artists {
            artist.draw_svg(svg, &transform);
        }

        svg.end_group();

        if self.axes_visible {
            // 4. Draw spines
            let spine_str = color_to_svg(&self.spine_color);

            if self.spine_visible[2] {
                svg.add_line(left, bottom, right, bottom, &spine_str, self.spine_linewidth, None, 1.0);
            }
            if self.spine_visible[0] {
                svg.add_line(left, top, right, top, &spine_str, self.spine_linewidth, None, 1.0);
            }
            if self.spine_visible[3] {
                svg.add_line(left, top, left, bottom, &spine_str, self.spine_linewidth, None, 1.0);
            }
            if self.spine_visible[1] {
                svg.add_line(right, top, right, bottom, &spine_str, self.spine_linewidth, None, 1.0);
            }

            // 5. Tick marks and labels
            let x_ticks: Vec<f64> = if log_x {
                x_ticks_data.clone()
            } else {
                self.custom_xticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_xmin, tick_xmax, 10))
            };
            let y_ticks: Vec<f64> = if log_y {
                y_ticks_data.clone()
            } else {
                self.custom_yticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_ymin, tick_ymax, 8))
            };

            let tick_len = self.tick_length;
            let tick_str = color_to_svg(&self.tick_color);
            let text_str = color_to_svg(&self.text_color);

            let (tick_out, tick_in) = match self.tick_direction {
                TickDirection::Out => (tick_len, 0.0_f32),
                TickDirection::In => (0.0_f32, tick_len),
                TickDirection::InOut => (tick_len, tick_len),
            };

            // X ticks
            for (i, &tx) in x_ticks.iter().enumerate() {
                let (px, _) = transform.transform_xy(tx, ymin);
                if px < left || px > right {
                    continue;
                }

                // Tick mark
                svg.add_line(px, bottom - tick_in, px, bottom + tick_out, &tick_str, self.tick_width, None, 1.0);

                // Tick label
                let label = if let Some(ref labels) = self.custom_xtick_labels {
                    labels.get(i).cloned().unwrap_or_default()
                } else if log_x {
                    format_log_tick_value(tx)
                } else {
                    format_tick_value(tx)
                };
                svg.add_text(px, bottom + tick_out + 2.0 + self.tick_label_size * 0.6, &label, self.tick_label_size, &text_str, "middle", 0.0);
            }

            // Y ticks
            for (i, &ty) in y_ticks.iter().enumerate() {
                let (_, py) = transform.transform_xy(xmin, ty);
                if py < top || py > bottom {
                    continue;
                }

                // Tick mark
                svg.add_line(left + tick_in, py, left - tick_out, py, &tick_str, self.tick_width, None, 1.0);

                // Tick label
                let label = if let Some(ref labels) = self.custom_ytick_labels {
                    labels.get(i).cloned().unwrap_or_default()
                } else if log_y {
                    format_log_tick_value(ty)
                } else {
                    format_tick_value(ty)
                };
                svg.add_text(left - tick_out - 3.0, py, &label, self.tick_label_size, &text_str, "end", 0.0);
            }

            // 7. xlabel
            if let Some(ref xlabel) = self.xlabel {
                let cx = (left + right) / 2.0;
                svg.add_text(cx, bottom + tick_out + self.tick_label_size + 12.0, xlabel, self.label_size, &text_str, "middle", 0.0);
            }

            // 8. ylabel (rotated)
            if let Some(ref ylabel) = self.ylabel {
                let cy = (top + bottom) / 2.0;
                svg.add_text(left - tick_out - 35.0, cy, ylabel, self.label_size, &text_str, "middle", -std::f32::consts::FRAC_PI_2);
            }
        }

        // 6. Title
        if let Some(ref title) = self.title {
            let cx = (left + right) / 2.0;
            let text_str = color_to_svg(&self.text_color);
            svg.add_text(cx, top - 8.0, title, self.title_size, &text_str, "middle", 0.0);
        }

        // 9. Text annotations
        for ann in &self.texts {
            let (px, py) = transform.transform_xy(ann.x, ann.y);
            let ann_color = color_to_svg(&ann.color);
            svg.add_text(px, py, &ann.text, ann.fontsize, &ann_color, "start", 0.0);
        }

        // 10. Legend
        if self.show_legend {
            let mut entries: Vec<(&str, crate::colors::Color)> = Vec::new();
            for artist in &self.artists {
                if let Some(entry) = artist.legend_entry() {
                    entries.push((Box::leak(entry.label.into_boxed_str()), entry.color));
                }
            }
            if !entries.is_empty() {
                let legend_w = 120.0_f32;
                let legend_margin = 10.0_f32;
                let (legend_x, legend_y) = match self.legend_loc.as_str() {
                    "upper left" => (left + legend_margin, top + legend_margin),
                    "lower right" => {
                        let legend_h = 12.0 + entries.len() as f32 * 15.0;
                        (right - legend_margin - legend_w, bottom - legend_margin - legend_h)
                    }
                    "lower left" => {
                        let legend_h = 12.0 + entries.len() as f32 * 15.0;
                        (left + legend_margin, bottom - legend_margin - legend_h)
                    }
                    _ => (right - legend_margin - legend_w, top + legend_margin),
                };

                let legend_h = 12.0 + entries.len() as f32 * 15.0;
                // Legend background
                svg.add_rect(legend_x, legend_y, legend_w, legend_h, "white", "rgb(200,200,200)", 0.5, 0.9);

                // Legend entries
                let font_size = 11.0_f32;
                let padding = 6.0;
                let swatch_size = 16.0;
                let line_height = font_size + 4.0;

                for (i, (label, color)) in entries.iter().enumerate() {
                    let y = legend_y + padding + i as f32 * line_height + line_height / 2.0;
                    let swatch_x = legend_x + padding;
                    let swatch_y = y - swatch_size / 2.0 + 1.0;
                    let color_str = color_to_svg(color);

                    // Color swatch
                    svg.add_rect(swatch_x, swatch_y, swatch_size, swatch_size * 0.6, &color_str, "none", 0.0, 1.0);
                    // Label
                    svg.add_text(swatch_x + swatch_size + 4.0, y, label, font_size, "rgb(0,0,0)", "start", 0.0);
                }
            }
        }

        // 11. Twin axes
        if let Some(ref twin) = self.twin_axes {
            twin.draw_svg(svg, left, top, right, bottom);
        }
    }

    /// Draw a table on the axes.
    fn draw_table(
        &self,
        pixmap: &mut Pixmap,
        table: &TableData,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    ) {
        let ts = tiny_skia::Transform::identity();

        let nrows = table.cell_text.len();
        if nrows == 0 {
            return;
        }
        let ncols = table.cell_text.iter().map(|r| r.len()).max().unwrap_or(0);
        if ncols == 0 {
            return;
        }

        let has_col_labels = table.col_labels.is_some();
        let has_row_labels = table.row_labels.is_some();
        let total_rows = nrows + if has_col_labels { 1 } else { 0 };
        let total_cols = ncols + if has_row_labels { 1 } else { 0 };

        // Compute cell dimensions
        let table_width = right - left;
        let cell_h = 20.0_f32;
        let table_height = cell_h * total_rows as f32;
        let cell_w = table_width / total_cols as f32;

        // Position table based on loc
        let table_top = match table.loc.as_str() {
            "top" => top,
            "center" => (top + bottom) / 2.0 - table_height / 2.0,
            _ => bottom, // "bottom" is default
        };
        let table_left = left;

        // Draw grid and cell text
        let mut paint_border = Paint::default();
        paint_border.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 255));
        paint_border.anti_alias = true;
        let mut stroke = Stroke::default();
        stroke.width = 0.5;

        let mut paint_header_bg = Paint::default();
        paint_header_bg.set_color(tiny_skia::Color::from_rgba8(220, 220, 220, 255));

        let font_size = 9.0_f32;
        let text_color = Color::new(0, 0, 0, 255);

        for row in 0..total_rows {
            for col in 0..total_cols {
                let cx = table_left + col as f32 * cell_w;
                let cy = table_top + row as f32 * cell_h;

                // Determine if this is a header cell
                let is_header_row = has_col_labels && row == 0;
                let is_header_col = has_row_labels && col == 0;
                let is_header = is_header_row || is_header_col;

                // Draw cell background
                if let Some(rect) = Rect::from_xywh(cx, cy, cell_w, cell_h) {
                    if is_header {
                        pixmap.fill_rect(rect, &paint_header_bg, ts, None);
                    } else {
                        let mut white_paint = Paint::default();
                        white_paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 240));
                        pixmap.fill_rect(rect, &white_paint, ts, None);
                    }
                }

                // Draw cell border
                let mut pb = PathBuilder::new();
                pb.move_to(cx, cy);
                pb.line_to(cx + cell_w, cy);
                pb.line_to(cx + cell_w, cy + cell_h);
                pb.line_to(cx, cy + cell_h);
                pb.close();
                if let Some(path) = pb.finish() {
                    pixmap.stroke_path(&path, &paint_border, &stroke, ts, None);
                }

                // Determine cell text
                let cell_str = if is_header_row && is_header_col {
                    String::new()
                } else if is_header_row {
                    let data_col = if has_row_labels { col - 1 } else { col };
                    table.col_labels.as_ref()
                        .and_then(|labels| labels.get(data_col))
                        .cloned()
                        .unwrap_or_default()
                } else if is_header_col {
                    let data_row = if has_col_labels { row - 1 } else { row };
                    table.row_labels.as_ref()
                        .and_then(|labels| labels.get(data_row))
                        .cloned()
                        .unwrap_or_default()
                } else {
                    let data_row = if has_col_labels { row - 1 } else { row };
                    let data_col = if has_row_labels { col - 1 } else { col };
                    table.cell_text.get(data_row)
                        .and_then(|r| r.get(data_col))
                        .cloned()
                        .unwrap_or_default()
                };

                if !cell_str.is_empty() {
                    draw_text(
                        pixmap,
                        &cell_str,
                        cx + cell_w / 2.0,
                        cy + cell_h / 2.0,
                        font_size,
                        text_color,
                        TextAnchorX::Center,
                        TextAnchorY::Center,
                        0.0,
                    );
                }
            }
        }
    }

    /// Draw a vertical colorbar to the right of the plot area.
    fn draw_colorbar(&self, pixmap: &mut Pixmap, right: f32, top: f32, bottom: f32) {
        use crate::artists::image::colormap_lookup;

        let ts = tiny_skia::Transform::identity();
        let bar_width = 15.0_f32;
        let bar_left = right + 10.0;
        let bar_right = bar_left + bar_width;
        let bar_top = top;
        let bar_bottom = bottom;
        let bar_height = bar_bottom - bar_top;

        if bar_height <= 0.0 { return; }

        // Draw gradient strip
        let n_steps = (bar_height as usize).max(2);
        for i in 0..n_steps {
            let frac = i as f32 / (n_steps - 1) as f32;
            // Top = vmax (t=1), bottom = vmin (t=0)
            let t = 1.0 - frac;
            let color = colormap_lookup(&self.colorbar_cmap, t as f64);

            let y = bar_top + frac * bar_height;
            let row_h = (bar_height / n_steps as f32).max(1.0);

            if let Some(rect) = Rect::from_xywh(bar_left, y, bar_width, row_h) {
                let mut paint = Paint::default();
                paint.set_color(color.to_tiny_skia());
                paint.anti_alias = false;
                pixmap.fill_rect(rect, &paint, ts, None);
            }
        }

        // Draw border around colorbar
        if let Some(rect) = Rect::from_xywh(bar_left, bar_top, bar_width, bar_height) {
            let path = PathBuilder::from_rect(rect);
            let mut border_paint = Paint::default();
            border_paint.set_color(self.spine_color.to_tiny_skia());
            border_paint.anti_alias = true;
            let mut stroke = Stroke::default();
            stroke.width = 0.5;
            pixmap.stroke_path(&path, &border_paint, &stroke, ts, None);
        }

        // Draw tick labels on the colorbar (5 ticks)
        let n_ticks = 5;
        let label_x = bar_right + 4.0;
        for i in 0..n_ticks {
            let frac = i as f32 / (n_ticks - 1) as f32;
            let y = bar_top + frac * bar_height;
            let val = self.colorbar_vmax + frac as f64 * (self.colorbar_vmin - self.colorbar_vmax);
            let label = crate::ticker::format_tick_value(val);

            // Tick mark
            let mut pb = PathBuilder::new();
            pb.move_to(bar_right, y);
            pb.line_to(bar_right + 3.0, y);
            if let Some(path) = pb.finish() {
                let mut tick_paint = Paint::default();
                tick_paint.set_color(self.spine_color.to_tiny_skia());
                tick_paint.anti_alias = true;
                let mut tick_stroke = Stroke::default();
                tick_stroke.width = 0.5;
                pixmap.stroke_path(&path, &tick_paint, &tick_stroke, ts, None);
            }

            draw_text(
                pixmap,
                &label,
                label_x,
                y,
                8.0,
                self.text_color,
                TextAnchorX::Left,
                TextAnchorY::Center,
                0.0,
            );
        }
    }

    /// Draw this axes as a twin (right y-axis, shared x-axis).
    fn draw_as_twin(&self, pixmap: &mut Pixmap, left: f32, top: f32, right: f32, bottom: f32) {
        let (xmin, xmax, ymin, ymax) = self.compute_bounds();

        let log_x = self.x_scale == AxisScale::Log;
        let log_y = self.y_scale == AxisScale::Log;

        let (dxmin, dxmax) = if log_x { (xmin.max(1e-15).log10(), xmax.max(1e-15).log10()) } else { (xmin, xmax) };
        let (dymin, dymax) = if log_y { (ymin.max(1e-15).log10(), ymax.max(1e-15).log10()) } else { (ymin, ymax) };

        let transform = Transform::new(
            (dxmin, dxmax),
            (dymin, dymax),
            left as f64,
            right as f64,
            top as f64,
            bottom as f64,
            log_x,
            log_y,
        );

        let ts = tiny_skia::Transform::identity();

        // Draw artists
        for artist in &self.artists {
            artist.draw(pixmap, &transform);
        }

        // Draw right-side Y axis ticks
        let tick_ymin = ymin;
        let tick_ymax = ymax;
        let y_ticks: Vec<f64> = if log_y {
            compute_log_ticks(tick_ymin.max(1e-15), tick_ymax.max(1e-15))
        } else {
            self.custom_yticks.clone().unwrap_or_else(|| compute_auto_ticks(tick_ymin, tick_ymax, 8))
        };

        let tick_len = 5.0_f32;
        let tick_color = Color::new(0, 0, 0, 255);

        let mut tick_paint = Paint::default();
        tick_paint.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 255));
        tick_paint.anti_alias = true;

        let mut tick_stroke = Stroke::default();
        tick_stroke.width = 1.0;

        // Right-side Y ticks
        for (i, &ty) in y_ticks.iter().enumerate() {
            let (_, py) = transform.transform_xy(xmin, ty);
            if py < top || py > bottom { continue; }

            // Tick mark on right side
            let mut pb = PathBuilder::new();
            pb.move_to(right, py);
            pb.line_to(right + tick_len, py);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &tick_paint, &tick_stroke, ts, None);
            }

            let label = if let Some(ref labels) = self.custom_ytick_labels {
                labels.get(i).cloned().unwrap_or_default()
            } else if log_y {
                format_log_tick_value(ty)
            } else {
                format_tick_value(ty)
            };
            draw_text(
                pixmap,
                &label,
                right + tick_len + 3.0,
                py,
                self.tick_size,
                tick_color,
                TextAnchorX::Left,
                TextAnchorY::Center,
                0.0,
            );
        }

        // Right-side Y label
        if let Some(ref ylabel) = self.ylabel {
            let cy = (top + bottom) / 2.0;
            draw_text(
                pixmap,
                ylabel,
                right + tick_len + 35.0,
                cy,
                self.label_size,
                tick_color,
                TextAnchorX::Center,
                TextAnchorY::Center,
                -std::f32::consts::FRAC_PI_2,
            );
        }

        // Legend for twin axes
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
                // Place twin legend on lower right by default
                let entry_count = entries.len() as f32;
                let legend_h = 12.0 + entry_count * 15.0;
                let legend_x = right - legend_margin - legend_w;
                let legend_y = bottom - legend_margin - legend_h;
                draw_legend(pixmap, &entries, legend_x, legend_y);
            }
        }
    }

    /// Draw polar plot.
    fn draw_polar(&self, pixmap: &mut Pixmap, left: f32, top: f32, right: f32, bottom: f32) {
        let ts = tiny_skia::Transform::identity();

        // White background
        if let Some(rect) = Rect::from_xywh(left, top, right - left, bottom - top) {
            let mut bg_paint = Paint::default();
            bg_paint.set_color(tiny_skia::Color::from_rgba8(255, 255, 255, 255));
            pixmap.fill_rect(rect, &bg_paint, ts, None);
        }

        // Center and radius of the polar plot in pixel space
        let cx = (left + right) / 2.0;
        let cy = (top + bottom) / 2.0;
        let plot_radius = ((right - left).min(bottom - top)) / 2.0 - 10.0;

        if plot_radius <= 0.0 { return; }

        // Find max radius from data
        let mut r_max: f64 = 1.0;
        for artist in &self.artists {
            let (_, _, _, ymax) = artist.data_bounds();
            if ymax > r_max { r_max = ymax; }
        }
        if let Some((_, ylim_max)) = self.ylim {
            r_max = ylim_max;
        }

        // Draw concentric circles (grid)
        let n_circles = 5usize;
        let mut grid_paint = Paint::default();
        grid_paint.set_color(tiny_skia::Color::from_rgba8(200, 200, 200, 255));
        grid_paint.anti_alias = true;
        let mut grid_stroke = Stroke::default();
        grid_stroke.width = 0.5;

        let tick_color = Color::new(0, 0, 0, 255);

        for i in 1..=n_circles {
            let frac = i as f32 / n_circles as f32;
            let r = plot_radius * frac;
            if let Some(circle) = crate::artists::circle_path(cx, cy, r) {
                pixmap.stroke_path(&circle, &grid_paint, &grid_stroke, ts, None);
            }
            // Radius tick label
            let val = r_max * frac as f64;
            draw_text(
                pixmap,
                &format_tick_value(val),
                cx + 3.0,
                cy - r - 2.0,
                8.0,
                tick_color,
                TextAnchorX::Left,
                TextAnchorY::Bottom,
                0.0,
            );
        }

        // Draw radial lines and angle labels
        let angles_deg = [0.0_f32, 30.0, 60.0, 90.0, 120.0, 150.0, 180.0, 210.0, 240.0, 270.0, 300.0, 330.0];
        for &deg in &angles_deg {
            let rad = deg * std::f32::consts::PI / 180.0;
            let ex = cx + plot_radius * rad.cos();
            let ey = cy - plot_radius * rad.sin();

            let mut pb = PathBuilder::new();
            pb.move_to(cx, cy);
            pb.line_to(ex, ey);
            if let Some(path) = pb.finish() {
                pixmap.stroke_path(&path, &grid_paint, &grid_stroke, ts, None);
            }

            // Angle label
            let label_r = plot_radius + 15.0;
            let lx = cx + label_r * rad.cos();
            let ly = cy - label_r * rad.sin();
            draw_text(
                pixmap,
                &format!("{:.0}°", deg),
                lx,
                ly,
                9.0,
                tick_color,
                TextAnchorX::Center,
                TextAnchorY::Center,
                0.0,
            );
        }

        // Draw outer circle border
        let mut border_paint = Paint::default();
        border_paint.set_color(tiny_skia::Color::from_rgba8(0, 0, 0, 255));
        border_paint.anti_alias = true;
        let mut border_stroke = Stroke::default();
        border_stroke.width = 1.0;
        if let Some(circle) = crate::artists::circle_path(cx, cy, plot_radius) {
            pixmap.stroke_path(&circle, &border_paint, &border_stroke, ts, None);
        }

        // Draw artists using polar→cartesian transform
        // Create a transform that maps (-r_max..r_max) in both x/y to the plot area
        let polar_transform = Transform::new(
            (-r_max, r_max),
            (-r_max, r_max),
            (cx - plot_radius) as f64,
            (cx + plot_radius) as f64,
            (cy - plot_radius) as f64,
            (cy + plot_radius) as f64,
            false,
            false,
        );

        // For polar plots, we convert the data ourselves and draw
        // Each artist has (angle, radius) data — we need to convert to cartesian
        // Since artists draw using transform, we build a cartesian transform and
        // convert the data inline. For simplicity, we'll draw line segments manually.
        for artist in &self.artists {
            artist.draw(pixmap, &polar_transform);
        }

        // Title
        if let Some(ref title) = self.title {
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
    }

    /// Add an annotation with arrow.
    pub fn annotate(
        &mut self,
        text: String,
        xy: (f64, f64),
        xytext: (f64, f64),
        fontsize: f32,
        color: Color,
        arrow_color: Color,
        arrow_width: f32,
    ) {
        self.annotations.push(Annotation {
            text,
            xy,
            xytext,
            fontsize,
            color,
            arrow_color,
            arrow_width,
        });
    }

    /// Add a horizontal shaded span region.
    pub fn axhspan(
        &mut self,
        ymin: f64,
        ymax: f64,
        color: Option<Color>,
        alpha: f32,
    ) {
        self.span_regions.push(SpanRegion {
            horizontal: true,
            vmin: ymin,
            vmax: ymax,
            color: color.unwrap_or(Color::new(0, 0, 255, 255)),
            alpha,
        });
    }

    /// Add a vertical shaded span region.
    pub fn axvspan(
        &mut self,
        xmin: f64,
        xmax: f64,
        color: Option<Color>,
        alpha: f32,
    ) {
        self.span_regions.push(SpanRegion {
            horizontal: false,
            vmin: xmin,
            vmax: xmax,
            color: color.unwrap_or(Color::new(0, 0, 255, 255)),
            alpha,
        });
    }

    /// Add multiple horizontal lines with bounded x range.
    pub fn hlines(
        &mut self,
        y_values: Vec<f64>,
        xmin: f64,
        xmax: f64,
        color: Option<Color>,
        linestyle: &str,
        linewidth: f32,
        alpha: f32,
    ) {
        let c = color.unwrap_or(Color::new(0, 0, 0, 255));
        for y in y_values {
            self.bounded_ref_lines.push(BoundedRefLine {
                horizontal: true,
                value: y,
                bound_min: xmin,
                bound_max: xmax,
                color: c,
                linestyle: LineStyle::from_str(linestyle),
                linewidth,
                alpha,
            });
        }
    }

    /// Add multiple vertical lines with bounded y range.
    pub fn vlines(
        &mut self,
        x_values: Vec<f64>,
        ymin: f64,
        ymax: f64,
        color: Option<Color>,
        linestyle: &str,
        linewidth: f32,
        alpha: f32,
    ) {
        let c = color.unwrap_or(Color::new(0, 0, 0, 255));
        for x in x_values {
            self.bounded_ref_lines.push(BoundedRefLine {
                horizontal: false,
                value: x,
                bound_min: ymin,
                bound_max: ymax,
                color: c,
                linestyle: LineStyle::from_str(linestyle),
                linewidth,
                alpha,
            });
        }
    }

    /// Add a violin plot.
    pub fn violinplot(
        &mut self,
        data: Vec<Vec<f64>>,
        positions: Option<Vec<f64>>,
        width: Option<f64>,
        color: Option<Color>,
        show_means: bool,
        show_medians: bool,
        alpha: Option<f32>,
        label: Option<String>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let pos = positions.unwrap_or_else(|| (1..=data.len()).map(|i| i as f64).collect());
        let w = width.unwrap_or(0.5);
        let a = alpha.unwrap_or(0.7);
        let mut vp = ViolinPlot::new(data, pos, w, c, show_means, show_medians, a);
        vp.label = label;
        self.artists.push(Box::new(vp));
    }

    /// Add a fill_betweenx area (horizontal band).
    pub fn fill_betweenx(
        &mut self,
        y: Vec<f64>,
        x1: Vec<f64>,
        x2: Vec<f64>,
        color: Option<Color>,
        alpha: Option<f32>,
        label: Option<String>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let a = alpha.unwrap_or(0.3);
        let mut fb = FillBetweenX::new(y, x1, x2, c, a);
        fb.label = label;
        self.artists.push(Box::new(fb));
    }

    /// Set table data to be drawn on the axes.
    pub fn set_table(
        &mut self,
        cell_text: Vec<Vec<String>>,
        col_labels: Option<Vec<String>>,
        row_labels: Option<Vec<String>>,
        loc: String,
    ) {
        self.table_data = Some(TableData {
            cell_text,
            col_labels,
            row_labels,
            loc,
        });
    }

    /// Add a contour plot (lines).
    pub fn contour(
        &mut self,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        levels: Option<Vec<f64>>,
        colors: Option<Vec<Color>>,
        linewidth: f32,
    ) {
        let c = Contour::new(x, y, z, levels, colors, false, linewidth);
        self.artists.push(Box::new(c));
    }

    /// Add a filled contour plot.
    pub fn contourf(
        &mut self,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        z: Vec<Vec<f64>>,
        levels: Option<Vec<f64>>,
        colors: Option<Vec<Color>>,
    ) {
        let c = Contour::new(x, y, z, levels, colors, true, 1.0);
        self.artists.push(Box::new(c));
    }

    /// Add a hexbin plot.
    pub fn hexbin(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        gridsize: usize,
        cmap: String,
        mincnt: usize,
    ) {
        let hb = HexBin::new(x, y, gridsize, cmap, mincnt);
        self.artists.push(Box::new(hb));
    }

    /// Add a patch (rectangle, circle, or polygon).
    pub fn add_patch(&mut self, patch: Patch) {
        self.artists.push(Box::new(patch));
    }

    /// Set this axes to polar mode.
    pub fn set_polar(&mut self, polar: bool) {
        self.polar = polar;
    }

    /// Create a twin y-axis (twinx). Returns a mutable reference to the twin Axes.
    pub fn twinx(&mut self) -> &mut Axes {
        self.twin_axes = Some(Box::new(Axes::new()));
        self.twin_axes.as_mut().unwrap()
    }

    /// Set axis visibility.
    pub fn set_axis_visible(&mut self, visible: bool) {
        self.axes_visible = visible;
    }

    /// Set tick parameters.
    pub fn set_tick_params(&mut self, direction: &str, length: f32, width: f32, labelsize: f32) {
        self.tick_direction = TickDirection::from_str(direction);
        self.tick_length = length;
        self.tick_width = width;
        self.tick_label_size = labelsize;
    }

    /// Set spine visibility.
    pub fn set_spine_visible(&mut self, which: &str, visible: bool) {
        match which {
            "top" => self.spine_visible[0] = visible,
            "right" => self.spine_visible[1] = visible,
            "bottom" => self.spine_visible[2] = visible,
            "left" => self.spine_visible[3] = visible,
            _ => {}
        }
    }

    /// Set background color.
    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    /// Set text color for title and labels.
    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    /// Set tick color.
    pub fn set_tick_color(&mut self, color: Color) {
        self.tick_color = color;
    }

    /// Set spine color.
    pub fn set_spine_color(&mut self, color: Color) {
        self.spine_color = color;
    }

    /// Set spine linewidth.
    pub fn set_spine_linewidth(&mut self, lw: f32) {
        self.spine_linewidth = lw;
    }

    /// Enable colorbar with the given colormap and value range.
    pub fn colorbar(&mut self, cmap: &str, vmin: f64, vmax: f64) {
        self.show_colorbar = true;
        self.colorbar_cmap = cmap.to_string();
        self.colorbar_vmin = vmin;
        self.colorbar_vmax = vmax;
    }

    /// Add a quiver (vector arrow) plot.
    pub fn quiver(
        &mut self,
        x: Vec<f64>,
        y: Vec<f64>,
        u: Vec<f64>,
        v: Vec<f64>,
        color: Option<Color>,
        scale: Option<f64>,
        width: Option<f32>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let q = crate::artists::quiver::Quiver::new(x, y, u, v, c, scale.unwrap_or(1.0), width.unwrap_or(1.5));
        self.artists.push(Box::new(q));
    }

    /// Add a streamplot (streamlines for vector fields).
    pub fn streamplot(
        &mut self,
        x: Vec<Vec<f64>>,
        y: Vec<Vec<f64>>,
        u: Vec<Vec<f64>>,
        v: Vec<Vec<f64>>,
        color: Option<Color>,
        density: Option<f64>,
        linewidth: Option<f32>,
    ) {
        let c = color.unwrap_or_else(|| self.next_color());
        let sp = crate::artists::streamplot::StreamPlot::new(
            x, y, u, v, c,
            density.unwrap_or(1.0),
            linewidth.unwrap_or(1.0),
        );
        self.artists.push(Box::new(sp));
    }
}
