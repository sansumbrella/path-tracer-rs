/// Simple vector implementation in 3 dimensions
/// Using the newtype idiom since arrays can be structs directly
#[derive(Debug, PartialEq)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }
    pub fn g(&self) -> f32 {
        self.0[1]
    }
    pub fn b(&self) -> f32 {
        self.0[2]
    }
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    pub fn z(&self) -> f32 {
        self.0[2]
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
        Vec3::new(
            self.0[0] * rhs.0[0],
            self.0[1] * rhs.0[1],
            self.0[2] * rhs.0[2],
        )
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Vec3::new(
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        )
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0[0] /= rhs.0[0];
        self.0[1] /= rhs.0[1];
        self.0[2] /= rhs.0[2];
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    // Component-wise addition of two vectors.
    fn add(self, rhs: Self) -> Self {
        Vec3::new(
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        )
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3::new(
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        )
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
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
