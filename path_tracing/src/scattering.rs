use super::HitRecord;
use super::{dot, normalize, rand, random_in_unit_sphere, reflect, refract, schlick, Ray, Vec3};

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
    pub roughness: f64,
}

impl Scattering for Metallic {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(&normalize(ray.direction()), &hit.normal);
        let scattered = Ray::new(hit.p, reflected + random_in_unit_sphere() * self.roughness);

        if dot(scattered.direction(), &hit.normal) > 0.0 {
            return Some(ScatteredRay {
                attenuation: self.albedo,
                ray: scattered,
            });
        }

        None
    }
}

/// Dielectric materials refract light, like glass.
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Scattering for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let outward_normal;
        let ni_over_nt;
        let cosine;

        if dot(ray.direction(), hit.normal()) > 0.0 {
            outward_normal = -hit.normal();
            ni_over_nt = self.refractive_index;
            let c = dot(ray.direction(), hit.normal()) / ray.direction().length();
            cosine = f64::sqrt(1.0 - self.refractive_index * self.refractive_index * (1.0 - c * c));
        } else {
            outward_normal = *hit.normal();
            ni_over_nt = 1.0 / self.refractive_index;
            cosine = -dot(ray.direction(), hit.normal()) / ray.direction().length();
        }

        if let Some(refracted) = refract(ray.direction(), &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            if rand() > reflect_prob {
                // refract
                return Some(ScatteredRay {
                    ray: Ray::new(*hit.position(), refracted),
                    attenuation,
                });
            }
        }

        let reflected = reflect(ray.direction(), hit.normal());
        Some(ScatteredRay {
            ray: Ray::new(*hit.position(), reflected),
            attenuation,
        })
    }
}

pub struct NaiveDielectric {
    pub refractive_index: f64,
}

impl Scattering for NaiveDielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let outward_normal: Vec3;
        let ni_over_nt: f64;
        if dot(ray.direction(), hit.normal()) > 0.0 {
            outward_normal = &Vec3::new(0.0, 0.0, 0.0) - hit.normal();
            ni_over_nt = self.refractive_index;
        } else {
            outward_normal = *hit.normal();
            ni_over_nt = 1.0 / self.refractive_index;
        }

        if let Some(refracted) = refract(ray.direction(), &outward_normal, ni_over_nt) {
            let scattered = Ray::new(*hit.position(), refracted);
            return Some(ScatteredRay {
                ray: scattered,
                attenuation,
            });
        } else {
            let reflected = reflect(ray.direction(), hit.normal());
            let scattered = Ray::new(*hit.position(), reflected);
            return Some(ScatteredRay {
                ray: scattered,
                attenuation,
            });
        }
    }
}
