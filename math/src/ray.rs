use super::vector::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        &self.origin + (&self.direction * t)
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_along_ray() {
        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.point_at_parameter(0.5), Vec3::new(1.5, 1.0, 1.0));
    }
}
