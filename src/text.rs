use ab_glyph::{point, Font, FontRef, PxScale, ScaleFont};
use tiny_skia::PremultipliedColorU8;

use crate::colors::Color;

static FONT_DATA: &[u8] = include_bytes!("fonts/DejaVuSans.ttf");

/// Horizontal text anchor.
#[derive(Debug, Clone, Copy)]
pub enum TextAnchorX {
    Left,
    Center,
    Right,
}

/// Vertical text anchor.
#[derive(Debug, Clone, Copy)]
pub enum TextAnchorY {
    Top,
    Center,
    Bottom,
    Baseline,
}

/// Measure the width and height of a text string at a given pixel size.
/// Returns (width, height).
pub fn measure_text(text: &str, size: f32) -> (f32, f32) {
    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scaled = font.as_scaled(PxScale::from(size));

    let mut width = 0.0f32;
    let mut prev_glyph_id = None;
    for ch in text.chars() {
        let glyph_id = scaled.glyph_id(ch);
        if let Some(prev) = prev_glyph_id {
            width += scaled.kern(prev, glyph_id);
        }
        width += scaled.h_advance(glyph_id);
        prev_glyph_id = Some(glyph_id);
    }

    let height = scaled.height();
    (width, height)
}

/// Draw text onto a tiny_skia Pixmap.
///
/// - `pixmap`: target pixmap to draw on
/// - `text`: the string to render
/// - `x`, `y`: position in pixel coordinates (anchor point)
/// - `size`: font size in pixels
/// - `color`: text color
/// - `anchor_x`: horizontal anchor
/// - `anchor_y`: vertical anchor
/// - `rotation`: rotation angle in radians (0 = normal, positive = counter-clockwise)
pub fn draw_text(
    pixmap: &mut tiny_skia::Pixmap,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    anchor_x: TextAnchorX,
    anchor_y: TextAnchorY,
    rotation: f32,
) {
    if text.is_empty() {
        return;
    }

    let font = FontRef::try_from_slice(FONT_DATA).expect("Failed to load embedded font");
    let scaled = font.as_scaled(PxScale::from(size));

    let (text_width, _text_height) = measure_text(text, size);

    // Compute the offset from the anchor point to the top-left baseline origin.
    let offset_x = match anchor_x {
        TextAnchorX::Left => 0.0,
        TextAnchorX::Center => -text_width / 2.0,
        TextAnchorX::Right => -text_width,
    };

    let ascent = scaled.ascent();
    let descent = scaled.descent();
    let offset_y = match anchor_y {
        TextAnchorY::Top => ascent,
        TextAnchorY::Center => (ascent + descent) / 2.0,
        TextAnchorY::Bottom => descent,
        TextAnchorY::Baseline => 0.0,
    };

    let pw = pixmap.width() as i32;
    let ph = pixmap.height() as i32;

    if rotation.abs() < 1e-6 {
        // No rotation: direct rendering for performance.
        draw_text_glyphs(pixmap, text, x + offset_x, y + offset_y, &scaled, &font, color, pw, ph);
    } else {
        // With rotation: render to a temporary pixmap, then composite with rotation.
        // We render the text at (0, ascent) in a scratch buffer, then blit with rotation.
        let scratch_w = (text_width.ceil() as u32).max(1);
        let scratch_h = ((ascent - descent).ceil() as u32).max(1);

        if let Some(mut scratch) = tiny_skia::Pixmap::new(scratch_w, scratch_h) {
            draw_text_glyphs(
                &mut scratch,
                text,
                0.0,
                ascent,
                &scaled,
                &font,
                color,
                scratch_w as i32,
                scratch_h as i32,
            );

            // Blit the scratch pixmap onto the main pixmap with rotation around (x, y).
            let cos_r = rotation.cos();
            let sin_r = rotation.sin();

            let src_pixels = scratch.pixels();

            // The anchor point in scratch coords
            let ax = -offset_x;
            let ay = ascent - offset_y;

            // Determine bounding box of rotated scratch in destination coords
            let corners = [
                (0.0 - ax, 0.0 - ay),
                (scratch_w as f32 - ax, 0.0 - ay),
                (0.0 - ax, scratch_h as f32 - ay),
                (scratch_w as f32 - ax, scratch_h as f32 - ay),
            ];

            let mut min_dx = f32::MAX;
            let mut max_dx = f32::MIN;
            let mut min_dy = f32::MAX;
            let mut max_dy = f32::MIN;

            for (cx, cy) in &corners {
                let rx = cx * cos_r - cy * sin_r + x;
                let ry = cx * sin_r + cy * cos_r + y;
                min_dx = min_dx.min(rx);
                max_dx = max_dx.max(rx);
                min_dy = min_dy.min(ry);
                max_dy = max_dy.max(ry);
            }

            let dst_x0 = (min_dx.floor() as i32).max(0);
            let dst_x1 = (max_dx.ceil() as i32).min(pw);
            let dst_y0 = (min_dy.floor() as i32).max(0);
            let dst_y1 = (max_dy.ceil() as i32).min(ph);

            let dst_pixels = pixmap.pixels_mut();

            for dy in dst_y0..dst_y1 {
                for dx in dst_x0..dst_x1 {
                    // Inverse-rotate to find source coordinates
                    let rel_x = dx as f32 - x;
                    let rel_y = dy as f32 - y;
                    let sx = rel_x * cos_r + rel_y * sin_r + ax;
                    let sy = -rel_x * sin_r + rel_y * cos_r + ay;

                    let sxi = sx as i32;
                    let syi = sy as i32;

                    if sxi >= 0 && sxi < scratch_w as i32 && syi >= 0 && syi < scratch_h as i32 {
                        let src_idx = (syi as u32 * scratch_w + sxi as u32) as usize;
                        let src = src_pixels[src_idx];
                        if src.alpha() > 0 {
                            let dst_idx = (dy as u32 * pw as u32 + dx as u32) as usize;
                            dst_pixels[dst_idx] = alpha_blend_premul(src, dst_pixels[dst_idx]);
                        }
                    }
                }
            }
        }
    }
}

/// Draw text glyphs directly onto a pixmap at a given baseline position.
fn draw_text_glyphs(
    pixmap: &mut tiny_skia::Pixmap,
    text: &str,
    base_x: f32,
    base_y: f32,
    scaled: &ab_glyph::PxScaleFont<&FontRef>,
    font: &FontRef,
    color: Color,
    pw: i32,
    ph: i32,
) {
    let mut cursor_x = base_x;
    let mut prev_glyph_id = None;

    for ch in text.chars() {
        let glyph_id = scaled.glyph_id(ch);
        if let Some(prev) = prev_glyph_id {
            cursor_x += scaled.kern(prev, glyph_id);
        }

        let glyph = glyph_id.with_scale_and_position(scaled.scale(), point(cursor_x, base_y));

        if let Some(outlined) = font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            let bx = bounds.min.x as i32;
            let by = bounds.min.y as i32;

            let pixels = pixmap.pixels_mut();

            outlined.draw(|gx, gy, coverage| {
                if coverage < 1e-4 {
                    return;
                }
                let px = bx + gx as i32;
                let py = by + gy as i32;
                if px < 0 || px >= pw || py < 0 || py >= ph {
                    return;
                }

                let idx = (py as u32 * pw as u32 + px as u32) as usize;
                let cov = coverage.min(1.0);
                let src_a = (color.a as f32 / 255.0 * cov * 255.0) as u8;
                if src_a == 0 {
                    return;
                }

                // Premultiply source color
                let src_r = (color.r as u16 * src_a as u16 / 255) as u8;
                let src_g = (color.g as u16 * src_a as u16 / 255) as u8;
                let src_b = (color.b as u16 * src_a as u16 / 255) as u8;

                if let Some(src_premul) = PremultipliedColorU8::from_rgba(src_r, src_g, src_b, src_a) {
                    pixels[idx] = alpha_blend_premul(src_premul, pixels[idx]);
                }
            });
        }

        cursor_x += scaled.h_advance(glyph_id);
        prev_glyph_id = Some(glyph_id);
    }
}

/// Alpha-blend source (premultiplied) over destination (premultiplied).
/// Standard Porter-Duff "source over" compositing.
fn alpha_blend_premul(src: PremultipliedColorU8, dst: PremultipliedColorU8) -> PremultipliedColorU8 {
    let sa = src.alpha() as u16;
    let inv_sa = 255 - sa;

    let r = src.red() as u16 + (dst.red() as u16 * inv_sa / 255);
    let g = src.green() as u16 + (dst.green() as u16 * inv_sa / 255);
    let b = src.blue() as u16 + (dst.blue() as u16 * inv_sa / 255);
    let a = sa + (dst.alpha() as u16 * inv_sa / 255);

    // These values should always satisfy r,g,b <= a for valid premultiplied inputs
    PremultipliedColorU8::from_rgba(r as u8, g as u8, b as u8, a as u8)
        .unwrap_or(PremultipliedColorU8::TRANSPARENT)
}
