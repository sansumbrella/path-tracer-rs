use super::{cross, normalize, random_in_unit_disk, Ray, Vec3};

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
}

impl Camera {
    pub fn new(
        origin: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = fov * std::f64::consts::PI / 180.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = normalize(origin - target);
        let u = normalize(cross(up, w));
        let v = cross(w, u);

        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * 2.0 * focus_dist * half_width;
        let vertical = v * 2.0 * focus_dist * half_height;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
        }
    }

    pub fn make_ray(&self, u: f64, v: f64) -> Ray {
        let rd = random_in_unit_disk();
        let offset = (self.u * rd[0] + self.v * rd[1]) * self.lens_radius;
        let origin = self.origin + offset;
        Ray::new(
            origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - origin,
        )
    }
}
