/// Native SVG renderer that produces real vector SVG elements.
use crate::colors::Color;

pub struct SvgRenderer {
    pub width: u32,
    pub height: u32,
    elements: Vec<String>,
}

impl SvgRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn add_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        fill: &str,
        stroke: &str,
        stroke_width: f32,
        opacity: f32,
    ) {
        self.elements.push(format!(
            r#"<rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}" fill="{}" stroke="{}" stroke-width="{:.2}" opacity="{:.3}"/>"#,
            x, y, w, h, fill, stroke, stroke_width, opacity
        ));
    }

    pub fn add_line(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: &str,
        stroke_width: f32,
        dash: Option<&str>,
        opacity: f32,
    ) {
        let dash_attr = match dash {
            Some(d) => format!(r#" stroke-dasharray="{}""#, d),
            None => String::new(),
        };
        self.elements.push(format!(
            r#"<line x1="{:.2}" y1="{:.2}" x2="{:.2}" y2="{:.2}" stroke="{}" stroke-width="{:.2}"{} opacity="{:.3}"/>"#,
            x1, y1, x2, y2, stroke, stroke_width, dash_attr, opacity
        ));
    }

    pub fn add_polyline(
        &mut self,
        points: &[(f32, f32)],
        stroke: &str,
        stroke_width: f32,
        fill: &str,
        dash: Option<&str>,
        opacity: f32,
    ) {
        if points.is_empty() {
            return;
        }
        let pts: String = points
            .iter()
            .map(|(x, y)| format!("{:.2},{:.2}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        let dash_attr = match dash {
            Some(d) => format!(r#" stroke-dasharray="{}""#, d),
            None => String::new(),
        };
        self.elements.push(format!(
            r#"<polyline points="{}" stroke="{}" stroke-width="{:.2}" fill="{}"{} opacity="{:.3}"/>"#,
            pts, stroke, stroke_width, fill, dash_attr, opacity
        ));
    }

    pub fn add_polygon(
        &mut self,
        points: &[(f32, f32)],
        fill: &str,
        stroke: &str,
        stroke_width: f32,
        opacity: f32,
    ) {
        if points.is_empty() {
            return;
        }
        let pts: String = points
            .iter()
            .map(|(x, y)| format!("{:.2},{:.2}", x, y))
            .collect::<Vec<_>>()
            .join(" ");
        self.elements.push(format!(
            r#"<polygon points="{}" fill="{}" stroke="{}" stroke-width="{:.2}" opacity="{:.3}"/>"#,
            pts, fill, stroke, stroke_width, opacity
        ));
    }

    pub fn add_circle(
        &mut self,
        cx: f32,
        cy: f32,
        r: f32,
        fill: &str,
        stroke: &str,
        stroke_width: f32,
        opacity: f32,
    ) {
        self.elements.push(format!(
            r#"<circle cx="{:.2}" cy="{:.2}" r="{:.2}" fill="{}" stroke="{}" stroke-width="{:.2}" opacity="{:.3}"/>"#,
            cx, cy, r, fill, stroke, stroke_width, opacity
        ));
    }

    pub fn add_text(
        &mut self,
        x: f32,
        y: f32,
        text: &str,
        font_size: f32,
        fill: &str,
        anchor: &str,
        rotation: f32,
    ) {
        let escaped = text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;");
        let transform = if rotation.abs() > 0.01 {
            format!(
                r#" transform="rotate({:.2},{:.2},{:.2})""#,
                -rotation.to_degrees(),
                x,
                y
            )
        } else {
            String::new()
        };
        self.elements.push(format!(
            r#"<text x="{:.2}" y="{:.2}" font-size="{:.1}" fill="{}" text-anchor="{}" dominant-baseline="central" font-family="sans-serif"{}>{}</text>"#,
            x, y, font_size, fill, anchor, transform, escaped
        ));
    }

    pub fn add_clip_rect(&mut self, id: &str, x: f32, y: f32, w: f32, h: f32) {
        self.elements.push(format!(
            r#"<clipPath id="{}"><rect x="{:.2}" y="{:.2}" width="{:.2}" height="{:.2}"/></clipPath>"#,
            id, x, y, w, h
        ));
    }

    pub fn begin_group(&mut self, clip_id: Option<&str>) {
        match clip_id {
            Some(id) => self
                .elements
                .push(format!(r#"<g clip-path="url(#{})">"#, id)),
            None => self.elements.push("<g>".to_string()),
        }
    }

    pub fn end_group(&mut self) {
        self.elements.push("</g>".to_string());
    }

    pub fn to_svg(&self, bg_color: &str) -> String {
        let mut svg = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
<rect width="100%" height="100%" fill="{}"/>
<defs>
"#,
            self.width, self.height, self.width, self.height, bg_color
        );

        // Separate defs (clipPaths) from other elements
        let mut defs = Vec::new();
        let mut body = Vec::new();
        for elem in &self.elements {
            if elem.starts_with("<clipPath") {
                defs.push(elem.as_str());
            } else {
                body.push(elem.as_str());
            }
        }

        for d in &defs {
            svg.push_str(d);
            svg.push('\n');
        }
        svg.push_str("</defs>\n");

        for b in &body {
            svg.push_str(b);
            svg.push('\n');
        }

        svg.push_str("</svg>");
        svg
    }
}

/// Helper: convert a Color to an SVG color string.
pub fn color_to_svg(c: &Color) -> String {
    format!("rgb({},{},{})", c.r, c.g, c.b)
}

/// Helper: convert a Color with alpha to CSS opacity value (0.0-1.0).
pub fn color_alpha(c: &Color) -> f32 {
    c.a as f32 / 255.0
}

/// Helper: convert a LineStyle to an SVG stroke-dasharray string.
pub fn linestyle_to_dash(ls: &crate::artists::LineStyle, linewidth: f32) -> Option<String> {
    match ls {
        crate::artists::LineStyle::Solid | crate::artists::LineStyle::None => None,
        crate::artists::LineStyle::Dashed => {
            let d = linewidth * 4.0;
            let g = linewidth * 2.0;
            Some(format!("{:.1},{:.1}", d, g))
        }
        crate::artists::LineStyle::DashDot => {
            let d = linewidth * 4.0;
            let dot = linewidth;
            let g = linewidth * 2.0;
            Some(format!("{:.1},{:.1},{:.1},{:.1}", d, g, dot, g))
        }
        crate::artists::LineStyle::Dotted => {
            let dot = linewidth;
            let g = linewidth * 2.0;
            Some(format!("{:.1},{:.1}", dot, g))
        }
    }
}
