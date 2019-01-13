use super::{dot, Ray, Vec3};
use super::{HitRecord, Hitable, Scattering};

/// A Sphere at a given position
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Scattering>, // for testability, would be easier not to complect with materials
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = dot(ray.direction(), ray.direction());
        let b = dot(oc, ray.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - f64::sqrt(discriminant)) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (&p - &self.center) / self.radius,
                    material: &*self.material,
                });
            }

            let t = (-b + f64::sqrt(discriminant)) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (&p - &self.center) / self.radius,
                    material: &*self.material,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::super::Lambertian;
    use super::{Hitable, Ray, Sphere, Vec3};

    #[test]
    fn sphere_at_origin() {
        let sphere = Sphere {
            center: Vec3::fill(0.0),
            radius: 1.0,
            material: Box::new(Lambertian {
                albedo: Vec3::fill(1.0),
            }),
        };

        let ray = Ray::new(Vec3::new(0.0, 0.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.0, std::f64::MAX).unwrap();
        assert_eq!(hit.p, Vec3::new(0.0, 0.0, -1.0));
    }
}
