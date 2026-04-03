/// Camera for 3D to 2D projection using orthographic projection.
pub struct Camera {
    pub elevation: f64, // degrees, vertical angle (default 30)
    pub azimuth: f64,   // degrees, horizontal angle (default -60)
    pub distance: f64,  // distance from origin (default 10)
}

impl Camera {
    pub fn new() -> Self {
        Self {
            elevation: 30.0,
            azimuth: -60.0,
            distance: 10.0,
        }
    }

    /// Project a 3D point (x, y, z) to 2D screen coordinates (sx, sy, depth).
    pub fn project(&self, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
        let elev = self.elevation.to_radians();
        let azim = self.azimuth.to_radians();

        // Rotate around z-axis by azimuth
        let x1 = x * azim.cos() + y * azim.sin();
        let y1 = -x * azim.sin() + y * azim.cos();
        let z1 = z;

        // Rotate around x-axis by elevation
        let x2 = x1;
        let y2 = y1 * elev.cos() - z1 * elev.sin();
        let z2 = y1 * elev.sin() + z1 * elev.cos();

        // Orthographic projection: screen_x = x2, screen_y = z2, depth = y2
        (x2, z2, y2)
    }
}
