use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Simple vector implementation in 3 dimensions
/// Using the newtype idiom since arrays can be structs directly
#[derive(Debug, PartialEq, Copy)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3([x, y, z])
    }
    pub fn fill(v: f64) -> Vec3 {
        Vec3([v, v, v])
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
        f64::sqrt(self.length_squared())
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Add<T> for Vec3
where
    T: Borrow<Vec3>,
{
    type Output = Self;

    // Component-wise addition of two vectors.
    fn add(self, rhs: T) -> Self {
        let &[x, y, z] = &rhs.borrow().0;
        Vec3::new(self.0[0] + x, self.0[1] + y, self.0[2] + z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        let &[x, y, z] = &self.0;
        Vec3::new(x + rhs, y + rhs, z + rhs)
    }
}

impl<T> Add<T> for &Vec3
where
    T: Borrow<Vec3>,
{
    type Output = Vec3;

    fn add(self, rhs: T) -> Vec3 {
        let rhs = rhs.borrow();
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

impl<T> Div<T> for Vec3
where
    T: Borrow<Vec3>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let &[x, y, z] = &rhs.borrow().0;
        Vec3::new(self.0[0] / x, self.0[1] / y, self.0[2] / z)
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

impl<T> Mul<T> for Vec3
where
    T: Borrow<Vec3>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let &[x, y, z] = &rhs.borrow().0;
        Vec3::new(self.0[0] * x, self.0[1] * y, self.0[2] * z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        let &[x, y, z] = &self.0;
        Vec3::new(x * rhs, y * rhs, z * rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        let &[x, y, z] = &self.0;
        Vec3::new(x * rhs, y * rhs, z * rhs)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        let &[x, y, z] = &self.0;
        Vec3::new(-x, -y, -z)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        let &[x, y, z] = &self.0;
        Vec3::new(-x, -y, -z)
    }
}

impl<T> Sub<T> for Vec3
where
    T: Borrow<Vec3>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        let &[x, y, z] = &rhs.borrow().0;
        Vec3::new(self.0[0] - x, self.0[1] - y, self.0[2] - z)
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
    fn multiplication() {
        let a = Vec3::new(-1.0, 1.0, 2.0);
        let b = Vec3::new(10.0, 10.0, 10.0);
        assert_eq!(
            a * b,
            Vec3::new(-10.0, 10.0, 20.0),
            "Vector multiplication is component-wise"
        );
        assert_eq!(a * b, a * &b);
    }

    #[test]
    fn division() {
        let a = Vec3::new(1.0, 2.0, 4.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(
            a / b,
            Vec3::new(0.5, 1.0, 2.0),
            "Vector division is component-wise"
        );
        assert_eq!(a / b, a / &b);
    }

    #[test]
    fn addition() {
        let a = Vec3::new(-1.0, 1.0, 2.0);
        let b = Vec3::new(10.0, 10.0, 10.0);
        assert_eq!(
            a + b,
            Vec3::new(9.0, 11.0, 12.0),
            "Vectors can be added together"
        );

        assert_eq!(a + b, a + &b);
        assert_eq!(a + b, b + a);
    }

    #[test]
    fn subtraction() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(b - a, Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(b - a, b - &a);
        assert_eq!(a - b, a - &b);
        assert_ne!(a - b, b - a);

        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
        assert_eq!(-b, Vec3::new(-4.0, -5.0, -6.0));
    }
}
