use super::HitRecord;
use math::{dot, normalize, random_in_unit_sphere, reflect, Ray, Vec3};

/// A Ray after scattering off a Hitable
pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vec3,
}

/// Scattering determines how a ray behaves after hitting a Hitable
pub trait Scattering {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay>;
}

/// Lambertian scattering is perfectly diffuse
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scattering for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let ray = Ray::new(hit.p, target - hit.p);
        Some(ScatteredRay {
            ray,
            attenuation: self.albedo,
        })
    }
}

/// Metallic scattering reflects rays at a consistent angle
pub struct Metallic {
    pub albedo: Vec3,
}

impl Scattering for Metallic {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(&normalize(ray.direction()), &hit.normal);
        let scattered = Ray::new(hit.p, reflected);

        if dot(scattered.direction(), &hit.normal) > 0.0 {
            return Some(ScatteredRay {
                attenuation: self.albedo,
                ray: scattered,
            });
        }

        None
    }
}
