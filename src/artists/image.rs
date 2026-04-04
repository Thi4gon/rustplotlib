use crate::artists::Artist;
use crate::colors::Color;
use crate::transforms::Transform;
use crate::text::{draw_text, TextAnchorX, TextAnchorY};
use tiny_skia::{Paint, Rect, Pixmap};

/// Linearly interpolate between uniformly-spaced RGB control points.
/// `stops` is a slice of (r, g, b) where the position is implicit: t = i / (len - 1).
fn lerp_colormap_uniform(t: f32, stops: &[(f32, f32, f32)]) -> Color {
    if stops.is_empty() {
        return Color::new(0, 0, 0, 255);
    }
    if stops.len() == 1 {
        return Color::from_f32(stops[0].0, stops[0].1, stops[0].2, 1.0);
    }
    let t = t.clamp(0.0, 1.0);
    let n = stops.len() - 1;
    let scaled = t * n as f32;
    let idx = (scaled.floor() as usize).min(n - 1);
    let frac = scaled - idx as f32;
    let (r0, g0, b0) = stops[idx];
    let (r1, g1, b1) = stops[idx + 1];
    let r = r0 + frac * (r1 - r0);
    let g = g0 + frac * (g1 - g0);
    let b = b0 + frac * (b1 - b0);
    Color::from_f32(r, g, b, 1.0)
}

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
/// If the name ends with "_r", the colormap is reversed.
pub fn colormap_lookup(name: &str, t: f64) -> Color {
    let (base_name, reversed) = if name.ends_with("_r") {
        (&name[..name.len() - 2], true)
    } else {
        (name, false)
    };
    let t_val = if reversed { 1.0 - t } else { t };
    colormap_lookup_base(base_name, t_val)
}

/// Look up a color in a base (non-reversed) colormap.
fn colormap_lookup_base(name: &str, t: f64) -> Color {
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
        // --- Perceptually uniform colormaps (9 control points from matplotlib source) ---
        "plasma" => {
            lerp_colormap_uniform(t, &[
                (0.050, 0.030, 0.528),  // t=0.000
                (0.229, 0.029, 0.630),  // t=0.125
                (0.417, 0.001, 0.658),  // t=0.250
                (0.600, 0.088, 0.582),  // t=0.375
                (0.742, 0.215, 0.474),  // t=0.500
                (0.859, 0.345, 0.356),  // t=0.625
                (0.948, 0.497, 0.246),  // t=0.750
                (0.988, 0.683, 0.153),  // t=0.875
                (0.940, 0.975, 0.131),  // t=1.000
            ])
        }
        "inferno" => {
            lerp_colormap_uniform(t, &[
                (0.001, 0.000, 0.014),  // t=0.000
                (0.087, 0.044, 0.225),  // t=0.125
                (0.258, 0.039, 0.406),  // t=0.250
                (0.459, 0.082, 0.396),  // t=0.375
                (0.647, 0.165, 0.314),  // t=0.500
                (0.825, 0.281, 0.198),  // t=0.625
                (0.946, 0.449, 0.062),  // t=0.750
                (0.988, 0.668, 0.086),  // t=0.875
                (0.988, 0.998, 0.645),  // t=1.000
            ])
        }
        "magma" => {
            lerp_colormap_uniform(t, &[
                (0.001, 0.000, 0.014),  // t=0.000
                (0.078, 0.042, 0.206),  // t=0.125
                (0.232, 0.059, 0.437),  // t=0.250
                (0.432, 0.075, 0.524),  // t=0.375
                (0.647, 0.108, 0.511),  // t=0.500
                (0.847, 0.210, 0.415),  // t=0.625
                (0.960, 0.401, 0.344),  // t=0.750
                (0.993, 0.635, 0.432),  // t=0.875
                (0.987, 0.991, 0.750),  // t=1.000
            ])
        }
        "cividis" => {
            lerp_colormap_uniform(t, &[
                (0.000, 0.135, 0.305),  // t=0.000
                (0.107, 0.209, 0.370),  // t=0.125
                (0.209, 0.283, 0.404),  // t=0.250
                (0.326, 0.359, 0.424),  // t=0.375
                (0.471, 0.457, 0.420),  // t=0.500
                (0.596, 0.543, 0.384),  // t=0.625
                (0.734, 0.643, 0.329),  // t=0.750
                (0.877, 0.750, 0.252),  // t=0.875
                (0.995, 0.863, 0.196),  // t=1.000
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
        // --- Additional qualitative colormaps ---
        "tab10" => {
            let colors: [(f32, f32, f32); 10] = [
                (0.122, 0.467, 0.706), (1.000, 0.498, 0.055),
                (0.173, 0.627, 0.173), (0.839, 0.153, 0.157),
                (0.580, 0.404, 0.741), (0.549, 0.337, 0.294),
                (0.890, 0.467, 0.761), (0.498, 0.498, 0.498),
                (0.737, 0.741, 0.133), (0.090, 0.745, 0.812),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "tab20b" => {
            let colors: [(f32, f32, f32); 20] = [
                (0.227, 0.227, 0.467), (0.322, 0.322, 0.639),
                (0.420, 0.420, 0.812), (0.612, 0.612, 0.875),
                (0.255, 0.412, 0.310), (0.345, 0.565, 0.427),
                (0.459, 0.725, 0.557), (0.639, 0.820, 0.694),
                (0.498, 0.322, 0.157), (0.639, 0.463, 0.267),
                (0.792, 0.635, 0.388), (0.910, 0.788, 0.557),
                (0.482, 0.286, 0.337), (0.675, 0.376, 0.431),
                (0.839, 0.533, 0.565), (0.929, 0.706, 0.718),
                (0.459, 0.427, 0.357), (0.588, 0.545, 0.424),
                (0.718, 0.671, 0.541), (0.847, 0.820, 0.710),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "tab20c" => {
            let colors: [(f32, f32, f32); 16] = [
                (0.192, 0.510, 0.741), (0.420, 0.682, 0.839),
                (0.620, 0.792, 0.882), (0.776, 0.855, 0.922),
                (0.333, 0.659, 0.408), (0.569, 0.792, 0.557),
                (0.718, 0.867, 0.690), (0.820, 0.914, 0.796),
                (0.890, 0.435, 0.318), (0.965, 0.647, 0.463),
                (0.992, 0.788, 0.616), (0.992, 0.878, 0.757),
                (0.620, 0.298, 0.631), (0.761, 0.494, 0.761),
                (0.859, 0.671, 0.851), (0.918, 0.800, 0.918),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Accent" => {
            let colors: [(f32, f32, f32); 8] = [
                (0.498, 0.788, 0.498), (0.745, 0.682, 0.831),
                (0.992, 0.753, 0.525), (1.000, 1.000, 0.600),
                (0.220, 0.424, 0.690), (0.941, 0.008, 0.498),
                (0.749, 0.357, 0.090), (0.400, 0.400, 0.400),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Dark2" => {
            let colors: [(f32, f32, f32); 8] = [
                (0.106, 0.620, 0.467), (0.851, 0.373, 0.008),
                (0.459, 0.439, 0.702), (0.906, 0.161, 0.541),
                (0.400, 0.651, 0.118), (0.902, 0.671, 0.008),
                (0.651, 0.463, 0.114), (0.400, 0.400, 0.400),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        "Paired" => {
            let colors: [(f32, f32, f32); 12] = [
                (0.651, 0.808, 0.890), (0.122, 0.471, 0.706),
                (0.698, 0.875, 0.541), (0.200, 0.627, 0.173),
                (0.984, 0.604, 0.600), (0.890, 0.102, 0.110),
                (0.992, 0.749, 0.435), (1.000, 0.498, 0.000),
                (0.792, 0.698, 0.839), (0.416, 0.239, 0.604),
                (1.000, 1.000, 0.600), (0.694, 0.349, 0.157),
            ];
            let idx = ((t * (colors.len() as f32 - 0.001)) as usize).min(colors.len() - 1);
            Color::from_f32(colors[idx].0, colors[idx].1, colors[idx].2, 1.0)
        }
        // --- Additional sequential colormaps (ColorBrewer) ---
        "Oranges" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 0.961, 0.922),
                (0.25, 0.992, 0.816, 0.635),
                (0.5, 0.992, 0.553, 0.235),
                (0.75, 0.851, 0.282, 0.004),
                (1.0, 0.498, 0.153, 0.016),
            ])
        }
        "Purples" => {
            lerp_colormap(t, &[
                (0.0, 0.988, 0.984, 0.992),
                (0.25, 0.816, 0.749, 0.859),
                (0.5, 0.612, 0.518, 0.769),
                (0.75, 0.471, 0.318, 0.663),
                (1.0, 0.247, 0.004, 0.490),
            ])
        }
        "YlOrBr" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 1.000, 0.898),
                (0.25, 0.996, 0.890, 0.569),
                (0.5, 0.996, 0.769, 0.310),
                (0.75, 0.820, 0.518, 0.082),
                (1.0, 0.400, 0.145, 0.024),
            ])
        }
        "YlGn" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 1.000, 0.898),
                (0.25, 0.780, 0.914, 0.706),
                (0.5, 0.502, 0.804, 0.522),
                (0.75, 0.133, 0.663, 0.396),
                (1.0, 0.000, 0.408, 0.216),
            ])
        }
        "GnBu" => {
            lerp_colormap(t, &[
                (0.0, 0.969, 0.988, 0.941),
                (0.25, 0.780, 0.914, 0.745),
                (0.5, 0.502, 0.804, 0.733),
                (0.75, 0.188, 0.639, 0.792),
                (1.0, 0.031, 0.259, 0.561),
            ])
        }
        "PuBu" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 0.969, 0.984),
                (0.25, 0.816, 0.875, 0.957),
                (0.5, 0.529, 0.773, 0.882),
                (0.75, 0.208, 0.553, 0.765),
                (1.0, 0.016, 0.243, 0.494),
            ])
        }
        "PuRd" => {
            lerp_colormap(t, &[
                (0.0, 0.969, 0.957, 0.976),
                (0.25, 0.878, 0.773, 0.882),
                (0.5, 0.827, 0.506, 0.702),
                (0.75, 0.784, 0.141, 0.443),
                (1.0, 0.404, 0.000, 0.122),
            ])
        }
        "OrRd" => {
            lerp_colormap(t, &[
                (0.0, 1.000, 0.969, 0.925),
                (0.25, 0.996, 0.855, 0.678),
                (0.5, 0.992, 0.643, 0.420),
                (0.75, 0.925, 0.314, 0.153),
                (1.0, 0.502, 0.000, 0.000),
            ])
        }
        "BuGn" => {
            lerp_colormap(t, &[
                (0.0, 0.969, 0.988, 0.992),
                (0.25, 0.780, 0.914, 0.898),
                (0.5, 0.502, 0.804, 0.749),
                (0.75, 0.169, 0.643, 0.467),
                (1.0, 0.000, 0.396, 0.212),
            ])
        }
        "BuPu" => {
            lerp_colormap(t, &[
                (0.0, 0.969, 0.988, 0.992),
                (0.25, 0.749, 0.827, 0.902),
                (0.5, 0.549, 0.588, 0.780),
                (0.75, 0.549, 0.302, 0.651),
                (1.0, 0.302, 0.000, 0.294),
            ])
        }
        // --- Additional sequential colormaps (misc) ---
        "Wistia" => {
            lerp_colormap(t, &[
                (0.0, 0.894, 1.000, 0.502),
                (0.5, 1.000, 0.878, 0.000),
                (1.0, 1.000, 0.663, 0.000),
            ])
        }
        "afmhot" => {
            lerp_colormap(t, &[
                (0.0, 0.000, 0.000, 0.000),
                (0.25, 0.502, 0.000, 0.000),
                (0.5, 1.000, 0.502, 0.000),
                (0.75, 1.000, 1.000, 0.502),
                (1.0, 1.000, 1.000, 1.000),
            ])
        }
        "rainbow" => {
            lerp_colormap(t, &[
                (0.0, 0.500, 0.000, 1.000),
                (0.2, 0.000, 0.000, 1.000),
                (0.4, 0.000, 1.000, 0.000),
                (0.6, 1.000, 1.000, 0.000),
                (0.8, 1.000, 0.500, 0.000),
                (1.0, 1.000, 0.000, 0.000),
            ])
        }
        "gist_rainbow" => {
            lerp_colormap(t, &[
                (0.0,   1.000, 0.000, 0.160),
                (0.125, 1.000, 0.753, 0.000),
                (0.25,  0.286, 1.000, 0.000),
                (0.375, 0.000, 1.000, 0.722),
                (0.5,   0.000, 0.671, 1.000),
                (0.625, 0.000, 0.000, 1.000),
                (0.75,  0.643, 0.000, 1.000),
                (0.875, 1.000, 0.000, 0.741),
                (1.0,   1.000, 0.000, 0.000),
            ])
        }
        "gnuplot" => {
            // gnuplot: r=(t^0.5), g=(t^2), b=(sin(pi*t))
            let r = t.sqrt();
            let g = t * t;
            let b = (std::f32::consts::PI * t).sin().clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "gnuplot2" => {
            // gnuplot2: similar but shifted
            let r = (t * 4.0 / 3.0 - 1.0 / 3.0).clamp(0.0, 1.0);
            let g = (t * 4.0 / 3.0 - 2.0 / 3.0).clamp(0.0, 1.0);
            let b = if t < 0.25 { t * 4.0 } else if t < 0.92 { 1.0 } else { (1.0 - t) * 12.5 };
            Color::from_f32(r, g, b.clamp(0.0, 1.0), 1.0)
        }
        "CMRmap" => {
            lerp_colormap(t, &[
                (0.0,   0.000, 0.000, 0.000),
                (0.125, 0.150, 0.150, 0.500),
                (0.25,  0.300, 0.150, 0.750),
                (0.375, 0.600, 0.200, 0.500),
                (0.5,   1.000, 0.300, 0.150),
                (0.625, 1.000, 0.600, 0.000),
                (0.75,  1.000, 0.900, 0.200),
                (0.875, 1.000, 1.000, 0.600),
                (1.0,   1.000, 1.000, 1.000),
            ])
        }
        "cubehelix" => {
            // Default cubehelix (start=0.5, rotations=-1.5, hue=1.0, gamma=1.0)
            let angle = 2.0 * std::f32::consts::PI * (0.5 + (-1.5) * t);
            let amp = 0.5 * t * (1.0 - t); // hue=1.0
            let r = t + amp * (-0.14861 * angle.cos() + 1.78277 * angle.sin());
            let g = t + amp * (-0.29227 * angle.cos() - 0.90649 * angle.sin());
            let b = t + amp * (1.97294 * angle.cos());
            Color::from_f32(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0)
        }
        "brg" => {
            // brg: blue -> red -> green
            let r = (2.0 * t - 0.5).clamp(0.0, 1.0).min((3.0 - 2.0 * t).clamp(0.0, 1.0));
            let g = if t < 0.5 { 0.0 } else { (2.0 * t - 1.0).clamp(0.0, 1.0) };
            let b = (1.0 - 2.0 * t).clamp(0.0, 1.0);
            Color::from_f32(r, g, b, 1.0)
        }
        "gist_earth" => {
            lerp_colormap(t, &[
                (0.0,   0.000, 0.000, 0.000),
                (0.1,   0.063, 0.125, 0.502),
                (0.2,   0.251, 0.502, 0.631),
                (0.3,   0.125, 0.502, 0.251),
                (0.4,   0.224, 0.502, 0.125),
                (0.5,   0.467, 0.588, 0.192),
                (0.6,   0.635, 0.659, 0.361),
                (0.7,   0.741, 0.718, 0.502),
                (0.8,   0.875, 0.831, 0.706),
                (0.9,   0.949, 0.925, 0.875),
                (1.0,   1.000, 1.000, 1.000),
            ])
        }
        "gist_stern" => {
            lerp_colormap(t, &[
                (0.0,  0.000, 0.000, 0.000),
                (0.09, 1.000, 0.000, 0.000),
                (0.10, 0.000, 0.000, 0.000),
                (0.49, 0.000, 0.000, 0.000),
                (0.5,  1.000, 0.000, 0.000),
                (0.51, 0.012, 0.000, 0.000),
                (1.0,  1.000, 0.000, 1.000),
            ])
        }
        "gist_ncar" => {
            lerp_colormap(t, &[
                (0.0,   0.043, 0.000, 0.357),
                (0.1,   0.000, 0.133, 1.000),
                (0.2,   0.000, 0.714, 1.000),
                (0.3,   0.020, 0.969, 0.533),
                (0.4,   0.145, 1.000, 0.051),
                (0.5,   0.698, 1.000, 0.000),
                (0.6,   1.000, 0.835, 0.000),
                (0.7,   1.000, 0.365, 0.000),
                (0.8,   1.000, 0.004, 0.416),
                (0.9,   0.890, 0.004, 0.969),
                (1.0,   1.000, 0.988, 0.988),
            ])
        }
        // --- Cyclic colormaps ---
        "twilight_shifted" => {
            lerp_colormap(t, &[
                (0.0,  0.141, 0.125, 0.259),
                (0.25, 0.584, 0.310, 0.397),
                (0.5,  0.886, 0.851, 0.887),
                (0.75, 0.332, 0.384, 0.683),
                (1.0,  0.141, 0.125, 0.259),
            ])
        }
        "hsv" => {
            // HSV colormap: cycles through hues at full saturation and value
            let h = t * 6.0;
            let i = h.floor() as u32 % 6;
            let f = h - h.floor();
            let (r, g, b) = match i {
                0 => (1.0, f, 0.0),
                1 => (1.0 - f, 1.0, 0.0),
                2 => (0.0, 1.0, f),
                3 => (0.0, 1.0 - f, 1.0),
                4 => (f, 0.0, 1.0),
                _ => (1.0, 0.0, 1.0 - f),
            };
            Color::from_f32(r, g, b, 1.0)
        }
        "coolwarm" => {
            lerp_colormap_uniform(t, &[
                (0.230, 0.299, 0.754),
                (0.411, 0.467, 0.851),
                (0.600, 0.635, 0.925),
                (0.765, 0.781, 0.963),
                (0.878, 0.878, 0.878),
                (0.958, 0.768, 0.678),
                (0.906, 0.588, 0.478),
                (0.808, 0.377, 0.306),
                (0.706, 0.016, 0.150),
            ])
        }
        "bwr" => {
            if t < 0.5 {
                let s = t * 2.0;
                Color::from_f32(s, s, 1.0, 1.0)
            } else {
                let s = (t - 0.5) * 2.0;
                Color::from_f32(1.0, 1.0 - s, 1.0 - s, 1.0)
            }
        }
        "seismic" => {
            lerp_colormap_uniform(t, &[
                (0.0, 0.0, 0.30),
                (0.0, 0.0, 1.0),
                (1.0, 1.0, 1.0),
                (1.0, 0.0, 0.0),
                (0.50, 0.0, 0.0),
            ])
        }
        "PuOr" => {
            lerp_colormap_uniform(t, &[
                (0.498, 0.231, 0.031),
                (0.702, 0.345, 0.024),
                (0.878, 0.510, 0.078),
                (0.969, 0.718, 0.376),
                (0.996, 0.878, 0.714),
                (0.847, 0.855, 0.922),
                (0.698, 0.671, 0.824),
                (0.502, 0.451, 0.675),
                (0.329, 0.153, 0.533),
            ])
        }
        "RdGy" => {
            lerp_colormap_uniform(t, &[
                (0.404, 0.000, 0.122),
                (0.698, 0.094, 0.169),
                (0.890, 0.290, 0.200),
                (0.984, 0.604, 0.471),
                (1.000, 1.000, 1.000),
                (0.878, 0.878, 0.878),
                (0.729, 0.729, 0.729),
                (0.529, 0.529, 0.529),
                (0.302, 0.302, 0.302),
            ])
        }
        "RdYlGn" => {
            lerp_colormap_uniform(t, &[
                (0.647, 0.000, 0.149),
                (0.843, 0.188, 0.153),
                (0.957, 0.427, 0.263),
                (0.992, 0.682, 0.380),
                (1.000, 1.000, 0.749),
                (0.851, 0.937, 0.545),
                (0.651, 0.851, 0.416),
                (0.400, 0.741, 0.388),
                (0.102, 0.596, 0.314),
            ])
        }
        "Greys" => {
            Color::from_f32(1.0 - t, 1.0 - t, 1.0 - t, 1.0)
        }
        "PuBuGn" => {
            lerp_colormap_uniform(t, &[
                (1.000, 0.969, 0.984),
                (0.925, 0.886, 0.941),
                (0.816, 0.776, 0.886),
                (0.686, 0.647, 0.831),
                (0.478, 0.533, 0.733),
                (0.271, 0.459, 0.635),
                (0.137, 0.388, 0.502),
                (0.004, 0.353, 0.357),
                (0.004, 0.275, 0.212),
            ])
        }
        "RdPu" => {
            lerp_colormap_uniform(t, &[
                (1.000, 0.969, 0.953),
                (0.992, 0.878, 0.867),
                (0.988, 0.773, 0.753),
                (0.980, 0.624, 0.710),
                (0.969, 0.408, 0.631),
                (0.867, 0.204, 0.592),
                (0.682, 0.102, 0.494),
                (0.478, 0.004, 0.467),
                (0.286, 0.000, 0.416),
            ])
        }
        "gist_yarg" => {
            // Reverse of gist_gray: white to black
            Color::from_f32(1.0 - t, 1.0 - t, 1.0 - t, 1.0)
        }
        "flag" => {
            // Repeating red/white/blue/black pattern
            let phase = (t * 4.0).fract();
            let idx = ((t * 4.0) as u32) % 4;
            match idx {
                0 => Color::from_f32(1.0, 0.0, 0.0, 1.0), // red
                1 => Color::from_f32(1.0, 1.0, 1.0, 1.0), // white
                2 => Color::from_f32(0.0, 0.0, 1.0, 1.0), // blue
                _ => Color::from_f32(0.0, 0.0, 0.0, 1.0), // black
            }
        }
        "prism" => {
            // Repeating color cycle
            let phase = (t * 6.0).fract();
            let idx = ((t * 6.0) as u32) % 6;
            match idx {
                0 => Color::from_f32(1.0, 0.0, 0.0, 1.0), // red
                1 => Color::from_f32(1.0, 0.5, 0.0, 1.0), // orange
                2 => Color::from_f32(1.0, 1.0, 0.0, 1.0), // yellow
                3 => Color::from_f32(0.0, 1.0, 0.0, 1.0), // green
                4 => Color::from_f32(0.0, 0.0, 1.0, 1.0), // blue
                _ => Color::from_f32(0.5, 0.0, 1.0, 1.0), // violet
            }
        }
        // "viridis" and default (9 control points from matplotlib source data)
        "viridis" | _ => {
            lerp_colormap_uniform(t, &[
                (0.267, 0.005, 0.329),  // t=0.000 (dark purple)
                (0.283, 0.141, 0.458),  // t=0.125
                (0.254, 0.265, 0.530),  // t=0.250
                (0.207, 0.372, 0.553),  // t=0.375
                (0.164, 0.471, 0.558),  // t=0.500
                (0.128, 0.567, 0.551),  // t=0.625
                (0.135, 0.659, 0.518),  // t=0.750
                (0.360, 0.786, 0.387),  // t=0.875
                (0.993, 0.906, 0.144),  // t=1.000 (yellow)
            ])
        }
    }
}

/// Format a value for heatmap annotation based on a format spec.
fn format_annotation_value(val: f64, fmt: &str) -> String {
    match fmt {
        ".0f" => format!("{:.0}", val),
        ".1f" => format!("{:.1}", val),
        ".2f" => format!("{:.2}", val),
        ".3f" => format!("{:.3}", val),
        ".0g" | ".0" => {
            if val == val.floor() && val.abs() < 1e15 {
                format!("{:.0}", val)
            } else {
                format!("{:.0e}", val)
            }
        }
        ".1g" => format!("{:.1}", val),
        ".3g" => format!("{:.3}", val),
        "d" => format!("{}", val as i64),
        // Default: ".2g" — show 2 significant figures, trim trailing zeros
        _ => {
            let s = format!("{:.2}", val);
            // Trim trailing zeros after decimal point
            if s.contains('.') {
                let trimmed = s.trim_end_matches('0').trim_end_matches('.');
                trimmed.to_string()
            } else {
                s
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ImageInterpolation {
    Nearest,    // current behavior — no interpolation
    Bilinear,   // smooth blending between adjacent pixels
    Bicubic,    // smoother interpolation using cubic kernel
    Lanczos,    // sharp interpolation using sinc-based Lanczos kernel (a=3)
}

/// Image data variants: scalar (uses colormap) or direct RGB/RGBA.
pub enum ImageData {
    Scalar(Vec<Vec<f64>>),
    RGB(Vec<Vec<(f64, f64, f64)>>),
    RGBA(Vec<Vec<(f64, f64, f64, f64)>>),
}

pub struct Image {
    pub data: ImageData,
    pub rows: usize,
    pub cols: usize,
    pub cmap: String,
    pub vmin: f64,
    pub vmax: f64,
    pub annotate: bool,
    pub fmt: String,
    pub interpolation: ImageInterpolation,
    pub extent: Option<(f64, f64, f64, f64)>,  // (left, right, bottom, top)
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

        Image { data: ImageData::Scalar(data), rows, cols, cmap, vmin, vmax, annotate: false, fmt: ".2g".to_string(), interpolation: ImageInterpolation::Nearest, extent: None }
    }

    /// Create an Image from RGB data (H x W x 3).
    pub fn new_rgb(data: Vec<Vec<(f64, f64, f64)>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Image {
            data: ImageData::RGB(data),
            rows,
            cols,
            cmap: String::new(),
            vmin: 0.0,
            vmax: 1.0,
            annotate: false,
            fmt: ".2g".to_string(),
            interpolation: ImageInterpolation::Nearest,
            extent: None,
        }
    }

    /// Create an Image from RGBA data (H x W x 4).
    pub fn new_rgba(data: Vec<Vec<(f64, f64, f64, f64)>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Image {
            data: ImageData::RGBA(data),
            rows,
            cols,
            cmap: String::new(),
            vmin: 0.0,
            vmax: 1.0,
            annotate: false,
            fmt: ".2g".to_string(),
            interpolation: ImageInterpolation::Nearest,
            extent: None,
        }
    }

    /// Get the color for a cell, regardless of data type.
    fn cell_color(&self, r: usize, c: usize) -> Color {
        match &self.data {
            ImageData::Scalar(data) => {
                let val = data[r][c];
                let t = (val - self.vmin) / (self.vmax - self.vmin);
                colormap_lookup(&self.cmap, t)
            }
            ImageData::RGB(data) => {
                let (rv, gv, bv) = data[r][c];
                Color::from_f32(rv as f32, gv as f32, bv as f32, 1.0)
            }
            ImageData::RGBA(data) => {
                let (rv, gv, bv, av) = data[r][c];
                Color::from_f32(rv as f32, gv as f32, bv as f32, av as f32)
            }
        }
    }

    /// Get scalar value for annotation. Returns None for non-scalar data.
    fn cell_scalar(&self, r: usize, c: usize) -> Option<f64> {
        match &self.data {
            ImageData::Scalar(data) => Some(data[r][c]),
            _ => None,
        }
    }

    /// Get the data coordinates for a given cell using extent if set.
    fn cell_bounds(&self, r: usize, c: usize) -> (f64, f64, f64, f64) {
        if let Some((ext_left, ext_right, ext_bottom, ext_top)) = self.extent {
            // Map cell indices to extent coordinates
            let cell_w = (ext_right - ext_left) / self.cols as f64;
            let cell_h = (ext_top - ext_bottom) / self.rows as f64;
            let x0 = ext_left + c as f64 * cell_w;
            let x1 = x0 + cell_w;
            // Row 0 is top of image, so y goes from ext_top downward
            let y0 = ext_top - r as f64 * cell_h;
            let y1 = y0 - cell_h;
            (x0, x1, y1, y0) // (x0, x1, y_bottom, y_top)
        } else {
            let x0 = c as f64 - 0.5;
            let x1 = c as f64 + 0.5;
            let y0 = r as f64 - 0.5;
            let y1 = r as f64 + 0.5;
            (x0, x1, y0, y1)
        }
    }
}

impl Artist for Image {
    fn draw(&self, pixmap: &mut Pixmap, transform: &Transform) {
        if self.rows == 0 || self.cols == 0 {
            return;
        }

        let ts = tiny_skia::Transform::identity();

        match self.interpolation {
            ImageInterpolation::Nearest => {
                // Nearest-neighbor rendering: one rect per data cell
                for r in 0..self.rows {
                    for c in 0..self.cols {
                        let color = self.cell_color(r, c);

                        let mut paint = Paint::default();
                        paint.set_color(color.to_tiny_skia());
                        paint.anti_alias = false;

                        let (x0, x1, y0, y1) = self.cell_bounds(r, c);

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
            ImageInterpolation::Bilinear => {
                // Bilinear interpolation only supported for scalar data; fall back to nearest for RGB/RGBA
                if !matches!(&self.data, ImageData::Scalar(_)) {
                    // Fall back to nearest for RGB/RGBA
                    for r in 0..self.rows {
                        for c in 0..self.cols {
                            let color = self.cell_color(r, c);
                            let mut paint = Paint::default();
                            paint.set_color(color.to_tiny_skia());
                            paint.anti_alias = false;
                            let (x0, x1, y0, y1) = self.cell_bounds(r, c);
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
                } else if let ImageData::Scalar(ref scalar_data) = self.data {
                    // Bilinear interpolation for scalar data
                    let (data_x_min, data_x_max, data_y_min, data_y_max) = if let Some((el, er, eb, et)) = self.extent {
                        (el, er, eb, et)
                    } else {
                        (-0.5_f64, self.cols as f64 - 0.5, -0.5_f64, self.rows as f64 - 0.5)
                    };

                    let (px_left, py_top) = transform.transform_xy(data_x_min, data_y_min);
                    let (px_right, py_bottom) = transform.transform_xy(data_x_max, data_y_max);
                    let px_min = px_left.min(px_right) as i32;
                    let px_max = px_left.max(px_right).ceil() as i32;
                    let py_min = py_top.min(py_bottom) as i32;
                    let py_max = py_top.max(py_bottom).ceil() as i32;

                    let pw = pixmap.width() as i32;
                    let ph = pixmap.height() as i32;
                    let start_x = px_min.max(0);
                    let end_x = px_max.min(pw);
                    let start_y = py_min.max(0);
                    let end_y = py_max.min(ph);

                    let (p0x_f32, p0y_f32) = transform.transform_xy(data_x_min, data_y_min);
                    let (p1x_f32, p1y_f32) = transform.transform_xy(data_x_max, data_y_max);
                    let p0x = p0x_f32 as f64;
                    let p0y = p0y_f32 as f64;
                    let p1x = p1x_f32 as f64;
                    let p1y = p1y_f32 as f64;

                    let inv_sx = if (p1x - p0x).abs() > 1e-10 { (data_x_max - data_x_min) / (p1x - p0x) } else { 0.0 };
                    let inv_sy = if (p1y - p0y).abs() > 1e-10 { (data_y_max - data_y_min) / (p1y - p0y) } else { 0.0 };

                    let range = self.vmax - self.vmin;
                    let pm_width = pixmap.width();

                    for py in start_y..end_y {
                        for px in start_x..end_x {
                            let pxf = px as f64 + 0.5;
                            let pyf = py as f64 + 0.5;

                            let dx = data_x_min + (pxf - p0x) * inv_sx;
                            let dy = data_y_min + (pyf - p0y) * inv_sy;

                            let fx = dx.clamp(0.0, (self.cols - 1) as f64);
                            let fy = dy.clamp(0.0, (self.rows - 1) as f64);

                            let ix = fx.floor() as usize;
                            let iy = fy.floor() as usize;
                            let ix1 = (ix + 1).min(self.cols - 1);
                            let iy1 = (iy + 1).min(self.rows - 1);

                            let frac_x = fx - ix as f64;
                            let frac_y = fy - iy as f64;

                            let v00 = scalar_data[iy][ix];
                            let v10 = scalar_data[iy][ix1];
                            let v01 = scalar_data[iy1][ix];
                            let v11 = scalar_data[iy1][ix1];

                            let val = v00 * (1.0 - frac_x) * (1.0 - frac_y)
                                    + v10 * frac_x * (1.0 - frac_y)
                                    + v01 * (1.0 - frac_x) * frac_y
                                    + v11 * frac_x * frac_y;

                            let t = (val - self.vmin) / range;
                            let color = colormap_lookup(&self.cmap, t);

                            let pixel = tiny_skia::PremultipliedColorU8::from_rgba(
                                color.r, color.g, color.b, color.a,
                            );
                            if let Some(pixel_ref) = pixmap.pixels_mut().get_mut((py as u32 * pm_width + px as u32) as usize) {
                                if let Some(p) = pixel {
                                    *pixel_ref = p;
                                }
                            }
                        }
                    }
                }
            }
            ImageInterpolation::Bicubic => {
                // Bicubic interpolation using Keys cubic kernel (a = -0.5)
                // Works for scalar data; falls back to bilinear for RGB/RGBA

                fn cubic_kernel(x: f64) -> f64 {
                    let a = -0.5_f64; // Keys' cubic parameter
                    let ax = x.abs();
                    if ax <= 1.0 {
                        (a + 2.0) * ax * ax * ax - (a + 3.0) * ax * ax + 1.0
                    } else if ax < 2.0 {
                        a * ax * ax * ax - 5.0 * a * ax * ax + 8.0 * a * ax - 4.0 * a
                    } else {
                        0.0
                    }
                }

                if let ImageData::Scalar(ref scalar_data) = self.data {
                    let (data_x_min, data_x_max, data_y_min, data_y_max) = if let Some((el, er, eb, et)) = self.extent {
                        (el, er, eb, et)
                    } else {
                        (-0.5_f64, self.cols as f64 - 0.5, -0.5_f64, self.rows as f64 - 0.5)
                    };

                    let (px_left, py_top) = transform.transform_xy(data_x_min, data_y_min);
                    let (px_right, py_bottom) = transform.transform_xy(data_x_max, data_y_max);
                    let px_min = px_left.min(px_right) as i32;
                    let px_max = px_left.max(px_right).ceil() as i32;
                    let py_min = py_top.min(py_bottom) as i32;
                    let py_max = py_top.max(py_bottom).ceil() as i32;

                    let pw = pixmap.width() as i32;
                    let ph = pixmap.height() as i32;
                    let start_x = px_min.max(0);
                    let end_x = px_max.min(pw);
                    let start_y = py_min.max(0);
                    let end_y = py_max.min(ph);

                    let (p0x_f32, p0y_f32) = transform.transform_xy(data_x_min, data_y_min);
                    let (p1x_f32, p1y_f32) = transform.transform_xy(data_x_max, data_y_max);
                    let p0x = p0x_f32 as f64;
                    let p0y = p0y_f32 as f64;
                    let p1x = p1x_f32 as f64;
                    let p1y = p1y_f32 as f64;

                    let inv_sx = if (p1x - p0x).abs() > 1e-10 { (data_x_max - data_x_min) / (p1x - p0x) } else { 0.0 };
                    let inv_sy = if (p1y - p0y).abs() > 1e-10 { (data_y_max - data_y_min) / (p1y - p0y) } else { 0.0 };

                    let range = self.vmax - self.vmin;
                    let pm_width = pixmap.width();
                    let rows = self.rows as i32;
                    let cols = self.cols as i32;

                    for py in start_y..end_y {
                        for px in start_x..end_x {
                            let pxf = px as f64 + 0.5;
                            let pyf = py as f64 + 0.5;

                            let dx = data_x_min + (pxf - p0x) * inv_sx;
                            let dy = data_y_min + (pyf - p0y) * inv_sy;

                            let fx = dx.clamp(0.0, (self.cols - 1) as f64);
                            let fy = dy.clamp(0.0, (self.rows - 1) as f64);

                            let ix = fx.floor() as i32;
                            let iy = fy.floor() as i32;
                            let frac_x = fx - ix as f64;
                            let frac_y = fy - iy as f64;

                            // Sample 4x4 neighborhood
                            let mut val = 0.0;
                            let mut weight_sum = 0.0;
                            for m in -1..=2_i32 {
                                let wy = cubic_kernel(frac_y - m as f64);
                                let sy = (iy + m).clamp(0, rows - 1) as usize;
                                for n in -1..=2_i32 {
                                    let wx = cubic_kernel(frac_x - n as f64);
                                    let sx = (ix + n).clamp(0, cols - 1) as usize;
                                    let w = wx * wy;
                                    val += scalar_data[sy][sx] * w;
                                    weight_sum += w;
                                }
                            }
                            if weight_sum.abs() > 1e-15 {
                                val /= weight_sum;
                            }

                            let t = ((val - self.vmin) / range).clamp(0.0, 1.0);
                            let color = colormap_lookup(&self.cmap, t);

                            let pixel = tiny_skia::PremultipliedColorU8::from_rgba(
                                color.r, color.g, color.b, color.a,
                            );
                            if let Some(pixel_ref) = pixmap.pixels_mut().get_mut((py as u32 * pm_width + px as u32) as usize) {
                                if let Some(p) = pixel {
                                    *pixel_ref = p;
                                }
                            }
                        }
                    }
                } else {
                    // RGB/RGBA: use bilinear-style fallback (per-pixel channel interpolation)
                    for r in 0..self.rows {
                        for c in 0..self.cols {
                            let color = self.cell_color(r, c);
                            let mut paint = Paint::default();
                            paint.set_color(color.to_tiny_skia());
                            paint.anti_alias = false;
                            let (x0, x1, y0, y1) = self.cell_bounds(r, c);
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
            }
            ImageInterpolation::Lanczos => {
                // Lanczos interpolation (a=3) using sinc-based kernel
                // Works for scalar data; falls back to nearest for RGB/RGBA

                fn sinc(x: f64) -> f64 {
                    if x.abs() < 1e-15 { 1.0 } else {
                        let px = std::f64::consts::PI * x;
                        px.sin() / px
                    }
                }

                fn lanczos_kernel(x: f64, a: f64) -> f64 {
                    let ax = x.abs();
                    if ax < a { sinc(x) * sinc(x / a) } else { 0.0 }
                }

                if let ImageData::Scalar(ref scalar_data) = self.data {
                    let a = 3.0_f64; // Lanczos window size
                    let (data_x_min, data_x_max, data_y_min, data_y_max) = if let Some((el, er, eb, et)) = self.extent {
                        (el, er, eb, et)
                    } else {
                        (-0.5_f64, self.cols as f64 - 0.5, -0.5_f64, self.rows as f64 - 0.5)
                    };

                    let (px_left, py_top) = transform.transform_xy(data_x_min, data_y_min);
                    let (px_right, py_bottom) = transform.transform_xy(data_x_max, data_y_max);
                    let px_min = px_left.min(px_right) as i32;
                    let px_max = px_left.max(px_right).ceil() as i32;
                    let py_min = py_top.min(py_bottom) as i32;
                    let py_max = py_top.max(py_bottom).ceil() as i32;

                    let pw = pixmap.width() as i32;
                    let ph = pixmap.height() as i32;
                    let start_x = px_min.max(0);
                    let end_x = px_max.min(pw);
                    let start_y = py_min.max(0);
                    let end_y = py_max.min(ph);

                    let (p0x_f32, p0y_f32) = transform.transform_xy(data_x_min, data_y_min);
                    let (p1x_f32, p1y_f32) = transform.transform_xy(data_x_max, data_y_max);
                    let p0x = p0x_f32 as f64;
                    let p0y = p0y_f32 as f64;
                    let p1x = p1x_f32 as f64;
                    let p1y = p1y_f32 as f64;

                    let inv_sx = if (p1x - p0x).abs() > 1e-10 { (data_x_max - data_x_min) / (p1x - p0x) } else { 0.0 };
                    let inv_sy = if (p1y - p0y).abs() > 1e-10 { (data_y_max - data_y_min) / (p1y - p0y) } else { 0.0 };

                    let range = self.vmax - self.vmin;
                    let pm_width = pixmap.width();
                    let rows = self.rows as i32;
                    let cols = self.cols as i32;
                    let a_int = a as i32;

                    for py in start_y..end_y {
                        for px in start_x..end_x {
                            let pxf = px as f64 + 0.5;
                            let pyf = py as f64 + 0.5;

                            let dx = data_x_min + (pxf - p0x) * inv_sx;
                            let dy = data_y_min + (pyf - p0y) * inv_sy;

                            let fx = dx.clamp(0.0, (self.cols - 1) as f64);
                            let fy = dy.clamp(0.0, (self.rows - 1) as f64);

                            let ix = fx.floor() as i32;
                            let iy = fy.floor() as i32;
                            let frac_x = fx - ix as f64;
                            let frac_y = fy - iy as f64;

                            // Sample (2a)x(2a) neighborhood
                            let mut val = 0.0;
                            let mut weight_sum = 0.0;
                            for m in (1 - a_int)..=a_int {
                                let wy = lanczos_kernel(frac_y - m as f64, a);
                                let sy = (iy + m).clamp(0, rows - 1) as usize;
                                for n in (1 - a_int)..=a_int {
                                    let wx = lanczos_kernel(frac_x - n as f64, a);
                                    let sx = (ix + n).clamp(0, cols - 1) as usize;
                                    let w = wx * wy;
                                    val += scalar_data[sy][sx] * w;
                                    weight_sum += w;
                                }
                            }
                            if weight_sum.abs() > 1e-15 {
                                val /= weight_sum;
                            }

                            let t = ((val - self.vmin) / range).clamp(0.0, 1.0);
                            let color = colormap_lookup(&self.cmap, t);

                            let pixel = tiny_skia::PremultipliedColorU8::from_rgba(
                                color.r, color.g, color.b, color.a,
                            );
                            if let Some(pixel_ref) = pixmap.pixels_mut().get_mut((py as u32 * pm_width + px as u32) as usize) {
                                if let Some(p) = pixel {
                                    *pixel_ref = p;
                                }
                            }
                        }
                    }
                } else {
                    // RGB/RGBA: fallback to nearest
                    for r in 0..self.rows {
                        for c in 0..self.cols {
                            let color = self.cell_color(r, c);
                            let mut paint = Paint::default();
                            paint.set_color(color.to_tiny_skia());
                            paint.anti_alias = false;
                            let (x0, x1, y0, y1) = self.cell_bounds(r, c);
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
            }
        }

        // Draw annotation text if enabled (only for scalar data)
        if self.annotate {
            for r in 0..self.rows {
                for c in 0..self.cols {
                    let color = self.cell_color(r, c);

                    let (x0, x1, y0, y1) = self.cell_bounds(r, c);

                    let (px0, py0) = transform.transform_xy(x0, y0);
                    let (px1, py1) = transform.transform_xy(x1, y1);

                    let cx_px = (px0 + px1) / 2.0;
                    let cy_px = (py0 + py1) / 2.0;
                    let rh = (py1 - py0).abs().max(1.0);

                    let text = if let Some(val) = self.cell_scalar(r, c) {
                        format_annotation_value(val, &self.fmt)
                    } else {
                        continue; // skip annotation for non-scalar data
                    };

                    let brightness = 0.299 * (color.r as f32 / 255.0)
                        + 0.587 * (color.g as f32 / 255.0)
                        + 0.114 * (color.b as f32 / 255.0);
                    let text_color = if brightness > 0.5 {
                        Color::new(0, 0, 0, 255)
                    } else {
                        Color::new(255, 255, 255, 255)
                    };

                    let fontsize = (rh * 0.4).clamp(6.0, 16.0);

                    draw_text(
                        pixmap,
                        &text,
                        cx_px,
                        cy_px,
                        fontsize,
                        text_color,
                        TextAnchorX::Center,
                        TextAnchorY::Center,
                        0.0,
                    );
                }
            }
        }
    }

    fn data_bounds(&self) -> (f64, f64, f64, f64) {
        if let Some((left, right, bottom, top)) = self.extent {
            (left, right, bottom, top)
        } else {
            (
                -0.5,
                self.cols as f64 - 0.5,
                -0.5,
                self.rows as f64 - 0.5,
            )
        }
    }

    fn legend_label(&self) -> Option<&str> {
        None
    }

    fn legend_color(&self) -> Color {
        Color::new(0, 0, 0, 255)
    }
}
