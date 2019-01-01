#[derive(Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    /// Component-wise multiplication of two vectors.
    ///
    /// # Examples
    /// ```rust
    /// use math::Vec3;
    /// let a = Vec3::new(0.0, 1.0, 2.0);
    /// let b = Vec3::new(10.0, 10.0, 10.0);
    /// assert_eq!(a * b, Vec3::new(0.0, 10.0, 20.0))
    /// ```
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    // Component-wise addition of two vectors.
    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::cmp::Eq for Vec3 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_two_vectors() {
        let a = Vec3::new(-1.0, 1.0, 2.0);
        let b = Vec3::new(10.0, 10.0, 10.0);
        assert_eq!(a * b, Vec3::new(-10.0, 10.0, 20.0));
    }

    #[test]
    fn add_two_vectors() {
        let a = Vec3::new(-1.0, 1.0, 2.0);
        let b = Vec3::new(10.0, 10.0, 10.0);
        assert_eq!(a + b, Vec3::new(9.0, 11.0, 12.0));
    }
}
