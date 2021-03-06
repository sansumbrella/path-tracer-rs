use super::hitable::*;
use super::Ray;

pub struct World(Vec<Box<dyn Hitable + Sync>>);

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut found: Option<HitRecord> = None;
        for hitable in &self.0 {
            if let Some(hit) = hitable.hit(ray, t_min, closest) {
                closest = hit.t;
                found = Some(hit);
            }
        }
        found
    }
}

impl World {
    pub fn new() -> World {
        World(vec![])
    }

    pub fn push(&mut self, item: Box<dyn Hitable + Sync>) {
        self.0.push(item);
    }
}
