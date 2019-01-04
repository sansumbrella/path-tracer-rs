use super::HitRecord;
use math::{dot, normalize, random_in_unit_sphere, reflect, Ray, Vec3};

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Scattering {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay>;
}

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
