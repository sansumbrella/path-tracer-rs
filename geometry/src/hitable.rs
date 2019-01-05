use super::Scattering;
use math::{dot, Ray, Vec3};

///
/// HitRecords store information about a ray intersection with a Hitable surface or volume.
/// 
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Scattering,
}

/// Hitable types can reflect rays for tracing
pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// A Sphere at a given position
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Scattering>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        // would like to try out domain distortions here
        // let oc = oc + Vec3::new((oc.x() * 3.0).sin(), (oc.y() * 2.0).sin(), (oc.z() * 5.0).sin());
        let a = dot(&ray.direction(), &ray.direction());
        let b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - (b * b - a * c).sqrt()) / a;
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
