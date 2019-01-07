use super::Scattering;
use math::{Ray, Vec3};

///
/// HitRecords store information about a ray intersection with a Hitable surface or volume.
///
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Scattering,
}

impl<'a> HitRecord<'a> {
    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn position(&self) -> &Vec3 {
        &self.p
    }
}

/// Hitable types can reflect rays for tracing
pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
