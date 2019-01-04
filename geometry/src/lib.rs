use math::{dot, normalize, reflect, Ray, Vec3};
use rand::distributions::{Distribution, UnitSphereSurface};
use std::rc::Rc;

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let sphere = UnitSphereSurface::new();
    Vec3(sphere.sample(&mut rng))
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<Scattering>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Scattering>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
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
                    material: Rc::clone(&self.material),
                });
            }
        }
        None
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
