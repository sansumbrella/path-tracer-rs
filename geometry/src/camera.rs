use math::{cross, normalize, Ray, Vec3};

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, target: Vec3, up: Vec3, fov: f64, aspect: f64) -> Camera {
        let theta = fov * std::f64::consts::PI / 180.0;
        let half_height = f64::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = normalize(&(origin - target));
        let u = normalize(&cross(&up, &w));
        let v = cross(&w, &u);
        let lower_left_corner = origin - u * half_width - v * half_height - w;
        let horizontal = u * 2.0 * half_width;
        let vertical = v * 2.0 * half_height;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn make_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
