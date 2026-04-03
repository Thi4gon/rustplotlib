use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use tiny_skia::{Paint, Rect, Pixmap};

/// Linearly interpolate between control points.
/// `stops` is a slice of (position, r, g, b) where position is in [0, 1].
fn lerp_colormap(t: f32, stops: &[(f32, f32, f32, f32)]) -> Color {
    if stops.is_empty() {
        return Color::new(0, 0, 0, 255);
    }
    if t <= stops[0].0 {
        return Color::from_f32(stops[0].1, stops[0].2, stops[0].3, 1.0);
    }
    if t >= stops[stops.len() - 1].0 {
        let s = &stops[stops.len() - 1];
        return Color::from_f32(s.1, s.2, s.3, 1.0);
    }
    for i in 0..stops.len() - 1 {
        let (t0, r0, g0, b0) = stops[i];
        let (t1, r1, g1, b1) = stops[i + 1];
        if t >= t0 && t <= t1 {
            let frac = if (t1 - t0).abs() < 1e-10 { 0.0 } else { (t - t0) / (t1 - t0) };
            let r = r0 + frac * (r1 - r0);
            let g = g0 + frac * (g1 - g0);
            let b = b0 + frac * (b1 - b0);
            return Color::from_f32(r, g, b, 1.0);
        }
    }
    Color::new(0, 0, 0, 255)
}

/// Look up a color in a named colormap at parameter t in [0, 1].
pub fn colormap_lookup(name: &str, t: f64) -> Color {
    let t = t.clamp(0.0, 1.0) as f32;
    match name {
        "gray" | "grey" => {
            let v = (t * 255.0) as u8;
            Color::new(v, v, v, 255)
        }
        "hot" => {
            let r = (t * 3.0).min(1.0);
            let g = ((t - 1.0 / 3.0) * 3.0).clamp(0.0, 1.0);
            let b = ((t - 2.0 / 3.0) * 3.0).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "cool" => {
            Color::from_f32(t, 1.0 - t, 1.0, 1.0)
        }
        "jet" => {
            let r = ((1.5 - (t - 0.75).abs() * 4.0) as f32).clamp(0.0, 1.0);
            let g = ((1.5 - (t - 0.5).abs() * 4.0) as f32).clamp(0.0, 1.0);
            let b = ((1.5 - (t - 0.25).abs() * 4.0) as f32).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "Blues" => {
            Color::from_f32(1.0 - t * 0.8, 1.0 - t * 0.6, 1.0, 1.0)
        }
        "Reds" => {
            Color::from_f32(1.0, 1.0 - t * 0.8, 1.0 - t * 0.9, 1.0)
        }
        "Greens" => {
            Color::from_f32(1.0 - t * 0.8, 1.0 - t * 0.2, 1.0 - t * 0.8, 1.0)
        }
        // --- Perceptually uniform colormaps ---
        "plasma" => {
            lerp_colormap(t, &[
                (0.0, 0.050, 0.030, 0.528),
                (0.25, 0.494, 0.012, 0.658),
                (0.5, 0.798, 0.280, 0.470),
                (0.75, 0.973, 0.585, 0.254),
                (1.0, 0.940, 0.975, 0.131),
            ])
        }
        "inferno" => {
            lerp_colormap(t, &[
                (0.0, 0.001, 0.000, 0.014),
                (0.25, 0.320, 0.060, 0.480),
                (0.5, 0.730, 0.210, 0.330),
                (0.75, 0.980, 0.530, 0.120),
                (1.0, 0.988, 0.998, 0.645),
            ])
        }
        "magma" => {
            lerp_colormap(t, &[
                (0.0, 0.001, 0.000, 0.014),
                (0.25, 0.270, 0.060, 0.530),
                (0.5, 0.720, 0.150, 0.430),
                (0.75, 0.990, 0.490, 0.370),
                (1.0, 0.987, 0.991, 0.750),
            ])
        }
        "cividis" => {
            lerp_colormap(t, &[
                (0.0, 0.000, 0.135, 0.305),
                (0.25, 0.226, 0.290, 0.404),
                (0.5, 0.471, 0.457, 0.420),
                (0.75, 0.734, 0.643, 0.329),
                (1.0, 0.995, 0.863, 0.196),
            ])
        }
        "twilight" => {
            lerp_colormap(t, &[
                (0.0, 0.886, 0.851, 0.887),
                (0.25, 0.332, 0.384, 0.683),
                (0.5, 0.141, 0.125, 0.259),
                (0.75, 0.584, 0.310, 0.397),
                (1.0, 0.886, 0.851, 0.887),
            ])
        }
        "turbo" => {
            lerp_colormap(t, &[
                (0.0, 0.190, 0.072, 0.232),
                (0.15, 0.120, 0.390, 0.910),
                (0.35, 0.200, 0.750, 0.630),
                (0.5, 0.560, 0.920, 0.300),
                (0.65, 0.920, 0.810, 0.190),
                (0.85, 0.980, 0.420, 0.100),
                (1.0, 0.480, 0.010, 0.010),
            ])
        }
        // --- Seasonal colormaps ---
        "spring" => {
            Color::from_f32(1.0, t, 1.0 - t, 1.0)
        }
        "summer" => {
            Color::from_f32(t, (0.5 + t * 0.5).min(1.0), 0.4, 1.0)
        }
        "autumn" => {
            Color::from_f32(1.0, t, 0.0, 1.0)
        }
        "winter" => {
            Color::from_f32(0.0, t, (1.0 - t * 0.5).min(1.0), 1.0)
        }
        // --- Miscellaneous sequential colormaps ---
        "copper" => {
            let r = (t * 1.25).min(1.0);
            let g = t * 0.7812;
            let b = t * 0.4975;
            Color::from_f32(r, g, b, 1.0)
        }
        "bone" => {
            lerp_colormap(t, &[
                (0.0, 0.0, 0.0, 0.0),
                (0.33, 0.245, 0.245, 0.350),
                (0.66, 0.490, 0.595, 0.595),
                (1.0, 1.0, 1.0, 1.0),
            ])
        }
        "pink" => {
            let r = (t * 2.0).sqrt().min(1.0) * 0.7 + t * 0.3;
            let g = (t * 0.6).sqrt().min(1.0) * 0.5 + t * 0.5;
            let b = t;
            Color::from_f32(r.min(1.0), g.min(1.0), b.min(1.0), 1.0)
        }
        "binary" => {
            let v = 1.0 - t;
            Color::from_f32(v, v, v, 1.0)
        }
        "gist_heat" => {
            let r = (t * 2.0).min(1.0);
            let g = ((t - 0.5) * 2.0).clamp(0.0, 1.0);
            let b = ((t - 0.75) * 4.0).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "ocean" => {
            lerp_colormap(t, &[
                (0.0, 0.0, 0.502, 0.0),
                (0.33, 0.0, 0.0, 0.502),
                (0.67, 0.0, 0.502, 0.502),
                (1.0, 1.0, 1.0, 1.0),
            ])
        }
        "terrain" => {
            lerp_colormap(t, &[
                (0.0, 0.200, 0.200, 0.600),
                (0.15, 0.000, 0.600, 1.000),
                (0.25, 0.000, 0.800, 0.400),
                (0.50, 1.000, 1.000, 0.600),
                (0.75, 0.500, 0.360, 0.330),
                (1.0, 1.000, 1.000, 1.000),
            ])
        }
        // --- Diverging colormaps ---
        "YlOrRd" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 1.000, 0.800),
                (0.25, 0.996, 0.851, 0.463),
                (0.5, 0.992, 0.553, 0.235),
                (0.75, 0.890, 0.180, 0.153),
                (1.0, 0.502, 0.000, 0.149),
            ])
        }
        "YlGnBu" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 1.000, 0.851),
                (0.25, 0.631, 0.906, 0.694),
                (0.5, 0.255, 0.714, 0.769),
                (0.75, 0.133, 0.369, 0.659),
                (1.0, 0.031, 0.114, 0.345),
            ])
        }
        "RdYlBu" => {
            lerp_colormap(t, &[
                (0.0, 0.647, 0.000, 0.149),
                (0.25, 0.969, 0.427, 0.263),
                (0.5, 1.000, 1.000, 0.749),
                (0.75, 0.525, 0.737, 0.827),
                (1.0, 0.192, 0.212, 0.584),
            ])
        }
        "RdBu" => {
            lerp_colormap(t, &[
                (0.0, 0.404, 0.000, 0.122),
                (0.25, 0.839, 0.376, 0.302),
                (0.5, 0.969, 0.969, 0.969),
                (0.75, 0.420, 0.682, 0.839),
                (1.0, 0.020, 0.188, 0.380),
            ])
        }
        "PiYG" => {
            lerp_colormap(t, &[
                (0.0, 0.557, 0.004, 0.322),
                (0.25, 0.871, 0.467, 0.682),
                (0.5, 0.969, 0.969, 0.969),
                (0.75, 0.514, 0.761, 0.376),
                (1.0, 0.153, 0.392, 0.098),
            ])
        }
        "PRGn" => {
            lerp_colormap(t, &[
                (0.0, 0.251, 0.000, 0.294),
                (0.25, 0.600, 0.439, 0.671),
                (0.5, 0.969, 0.969, 0.969),
                (0.75, 0.498, 0.737, 0.455),
                (1.0, 0.000, 0.267, 0.106),
            ])
        }
        "BrBG" => {
            lerp_colormap(t, &[
                (0.0, 0.329, 0.188, 0.020),
                (0.25, 0.749, 0.506, 0.176),
                (0.5, 0.961, 0.961, 0.961),
                (0.75, 0.353, 0.706, 0.675),
                (1.0, 0.000, 0.235, 0.188),
            ])
        }
        "Spectral" => {
            lerp_colormap(t, &[
                (0.0, 0.620, 0.004, 0.259),
                (0.25, 0.957, 0.427, 0.263),
                (0.5, 1.000, 1.000, 0.749),
                (0.75, 0.533, 0.769, 0.506),
                (1.0, 0.369, 0.310, 0.635),
            ])
        }
        // --- Qualitative colormaps ---
        "Set1" => {
            let colors: [(f32, f32, f32); 9] = [
                (0.894, 0.102, 0.110), (0.216, 0.494, 0.722), (0.302, 0.686, 0.290),
                (0.596, 0.306, 0.639), (1.000, 0.498, 0.000), (1.000, 1.000, 0.200),
                (0.651, 0.337, 0.157), (0.969, 0.506, 0.749), (0.600, 0.600, 0.600),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Set2" => {
            let colors: [(f32, f32, f32); 8] = [
                (0.400, 0.761, 0.647), (0.988, 0.553, 0.384), (0.553, 0.627, 0.796),
                (0.906, 0.541, 0.765), (0.651, 0.847, 0.329), (1.000, 0.851, 0.184),
                (0.898, 0.769, 0.580), (0.702, 0.702, 0.702),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Set3" => {
            let colors: [(f32, f32, f32); 12] = [
                (0.553, 0.827, 0.780), (1.000, 1.000, 0.702), (0.745, 0.729, 0.855),
                (0.984, 0.502, 0.447), (0.502, 0.694, 0.827), (0.992, 0.706, 0.384),
                (0.702, 0.871, 0.412), (0.988, 0.804, 0.898), (0.851, 0.851, 0.851),
                (0.737, 0.502, 0.741), (0.800, 0.922, 0.773), (1.000, 0.929, 0.435),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Pastel1" => {
            let colors: [(f32, f32, f32); 9] = [
                (0.984, 0.706, 0.682), (0.702, 0.804, 0.890), (0.800, 0.922, 0.773),
                (0.871, 0.796, 0.894), (0.996, 0.851, 0.651), (1.000, 1.000, 0.800),
                (0.898, 0.847, 0.741), (0.992, 0.855, 0.925), (0.949, 0.949, 0.949),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Pastel2" => {
            let colors: [(f32, f32, f32); 8] = [
                (0.702, 0.886, 0.804), (0.992, 0.804, 0.675), (0.796, 0.835, 0.910),
                (0.957, 0.792, 0.894), (0.839, 0.918, 0.710), (1.000, 0.949, 0.682),
                (0.945, 0.886, 0.800), (0.851, 0.851, 0.851),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "tab20" => {
            let colors: [(f32, f32, f32); 20] = [
                (0.122, 0.467, 0.706), (0.682, 0.780, 0.910),
                (1.000, 0.498, 0.055), (1.000, 0.733, 0.471),
                (0.173, 0.627, 0.173), (0.596, 0.875, 0.541),
                (0.839, 0.153, 0.157), (1.000, 0.596, 0.588),
                (0.580, 0.404, 0.741), (0.773, 0.690, 0.835),
                (0.549, 0.337, 0.294), (0.769, 0.612, 0.580),
                (0.890, 0.467, 0.761), (0.969, 0.714, 0.824),
                (0.498, 0.498, 0.498), (0.780, 0.780, 0.780),
                (0.737, 0.741, 0.133), (0.859, 0.859, 0.553),
                (0.090, 0.745, 0.812), (0.620, 0.855, 0.898),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        // "viridis" and default
        "viridis" | _ => {
            // Simplified viridis approximation
            let r = (0.267004 + t * (0.993248 - 0.267004)) as f32;
            let g_val = if t < 0.5 {
                (0.004874 + t * 2.0 * (0.554906 - 0.004874)) as f32
            } else {
                (0.554906 + (t - 0.5) * 2.0 * (0.906157 - 0.554906)) as f32
            };
            let b_val = if t < 0.5 {
                (0.329415 + t * 2.0 * (0.554906 - 0.329415)) as f32
            } else {
                (0.554906 - (t - 0.5) * 2.0 * (0.554906 - 0.143936)) as f32
            };
            Color::from_f32(
                r.clamp(0.0, 1.0),
                g_val.clamp(0.0, 1.0),
                b_val.clamp(0.0, 1.0),
                1.0,
            )
        }
    }
}

pub struct Image {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
    pub cmap: String,
    pub vmin: f64,
    pub vmax: f64,
}

impl Image {
    pub fn new(data: Vec<Vec<f64>>, cmap: String) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };

        // Compute vmin/vmax from data
        let mut vmin = f64::MAX;
        let mut vmax = f64::MIN;
        for row in &data {
            for &v in row {
                if v < vmin { vmin = v; }
                if v > vmax { vmax = v; }
            }
        }
        if (vmax - vmin).abs() < 1e-15 {
            vmax = vmin + 1.0;
        }

        Image { data, rows, cols, cmap, vmin, vmax }
    }
}

impl Artist for Image {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }

        let ts = tiny_skia::Transform::identity();

        for r in 0..self.rows {
            for c in 0..self.cols {
                let val = self.data[r][c];
                let t = (val - self.vmin) / (self.vmax - self.vmin);
                let color = colormap_lookup(&self.cmap, t);

                let mut paint = Paint::default();
                paint.set_color(color.to_tiny_skia());
                paint.anti_alias = false;

                // Each cell spans from (c - 0.5) to (c + 0.5) in x, (r - 0.5) to (r + 0.5) in y
                let x0 = c as f64 - 0.5;
                let x1 = c as f64 + 0.5;
                let y0 = r as f64 - 0.5;
                let y1 = r as f64 + 0.5;

                let (px0, py0) = transform.transform_xy(x0, y0);
                let (px1, py1) = transform.transform_xy(x1, y1);

                let rx = px0.min(px1);
                let ry = py0.min(py1);
                let rw = (px1 - px0).abs().max(1.0);
                let rh = (py1 - py0).abs().max(1.0);

                if let Some(rect) = Rect::from_xywh(rx, ry, rw, rh) {
                    pixmap.fill_rect(rect, &paint, ts, None);
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        (
            -0.5,
            self.cols as f64 - 0.5,
            -0.5,
            self.rows as f64 - 0.5,
        )
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        Color::new(0, 0, 0, 255)
    }
}
