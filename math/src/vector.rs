use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// Simple vector implementation in 3 dimensions
/// Using the newtype idiom since arrays can be structs directly
#[derive(Debug, PartialEq, Copy)]
pub struct Vec3(pub [f64; 3]);

/// Returns a unit vector with the same direction as the input vector.
/// TODO: make generic so at least Vec3 and &Vec3 parameters are accepted
pub fn normalize(vector: &Vec3) -> Vec3 {
    let &[x, y, z] = &vector.0;
    Vec3::new(x, y, z) / vector.length()
}

/// Reflect a vector about a normal.
pub fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    vector - &(normal * 2.0 * dot(vector, normal))
}

/// Calculates the dot product of two vectors.
pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    let &[ax, ay, az] = &a.0;
    let &[bx, by, bz] = &b.0;

    ax * bx + ay * by + az * bz
}

/// Calculates the cross product of two vectors.
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    let &[ax, ay, az] = &a.0;
    let &[bx, by, bz] = &b.0;

    Vec3::new(ay * bz - az * by, -(ax * bz - az * bx), ax * by - ay * bx)
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn r(&self) -> f64 {
        self.0[0]
    }
    pub fn g(&self) -> f64 {
        self.0[1]
    }
    pub fn b(&self) -> f64 {
        self.0[2]
    }
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }

    /// Returns the magnitude squared of the vector.
    /// Use to avoid the square root calculation needed for magnitude.
    pub fn length_squared(&self) -> f64 {
        let &[x, y, z] = &self.0;
        x * x + y * y + z * z
    }

    /// Returns the magnitude of the vector (Euclidian norm)
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl Mul<Vec3> for Vec3 {
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

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.0[0] * rhs, self.0[1] * rhs, self.0[2] * rhs)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Vec3::new(
            self.0[0] / rhs.0[0],
            self.0[1] / rhs.0[1],
            self.0[2] / rhs.0[2],
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Vec3::new(self.0[0] / rhs, self.0[1] / rhs, self.0[2] / rhs)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0[0] /= rhs.0[0];
        self.0[1] /= rhs.0[1];
        self.0[2] /= rhs.0[2];
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}

impl Add for Vec3 {
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

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let &[x, y, z] = &self.0;
        Vec3::new(x + rhs, y + rhs, z + rhs)
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3::new(
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        )
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        )
    }
}

impl SubAssign for Vec3 {
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
        assert_eq!(
            a + b,
            Vec3::new(9.0, 11.0, 12.0),
            "Vectors can be added together"
        );
    }

    #[test]
    fn dot_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.5, 0.5, 0.5);

        assert_eq!(
            dot(&a, &b),
            0.5,
            "Dot product returns a scalar measuring similarity of two vectors"
        );

        assert_eq!(
            dot(&Vec3::new(0.0, -1.0, 0.0), &Vec3::new(0.0, 1.0, 0.0)),
            -1.0,
            "Dot product returns a scalar measuring similarity of two vectors"
        );
    }

    #[test]
    fn cross_product() {
        let x = Vec3::new(1.0, 0.0, 0.0);
        let y = Vec3::new(0.0, 1.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(
            cross(&x, &y),
            z,
            "Cross product returns a vector orthogonal to both inputs"
        );

        assert_eq!(
            cross(&y, &x),
            Vec3::new(0.0, 0.0, -1.0),
            "Cross product is not commutative"
        );

        assert_eq!(
            cross(&y, &z),
            x,
            "Cross product returns a vector orthogonal to both inputs"
        );

        assert_eq!(
            cross(&z, &x),
            y,
            "Cross product returns a vector orthogonal to both inputs"
        );
    }
}
