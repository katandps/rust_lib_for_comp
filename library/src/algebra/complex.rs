//! # 複素数
//! 四則演算をサポート
//!
//! 誤差の対策は十分に行うこと
use crate::algebra::{One, Zero};
use crate::*;

#[derive(Clone, Copy)]
pub struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    pub fn real(&self) -> f64 {
        self.real
    }
    pub fn imag(&self) -> f64 {
        self.imaginary
    }
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
    /// 複素数を極形式で指定して作る
    pub fn polar(rho: f64, theta: f64) -> Self {
        Self::new(rho * theta.cos(), rho * theta.sin())
    }
}
impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        let c = *self - *other;
        c.real.abs() < 1e-12 && c.imaginary.abs() < 1e-12
    }
}
impl From<f64> for Complex {
    fn from(r: f64) -> Self {
        Complex::new(r, 0.0)
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex::new(self.real + rhs.real, self.imaginary + rhs.imaginary)
    }
}
impl AddAssign<Self> for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Complex::new(self.real - rhs.real, self.imaginary - rhs.imaginary)
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imaginary * rhs.imaginary,
            self.real * rhs.imaginary + self.imaginary * rhs.real,
        )
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
        if d.abs() < std::f64::EPSILON {
            panic!("divide by 0");
        }
        Self::new(
            (self.real * rhs.real + self.imaginary * rhs.imaginary) / d,
            (self.imaginary * rhs.real - self.real * rhs.imaginary) / d,
        )
    }
}
impl DivAssign<usize> for Complex {
    fn div_assign(&mut self, rhs: usize) {
        *self = *self / Self::new(rhs as f64, 0.0);
    }
}
impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.real, -self.imaginary)
    }
}
impl Zero for Complex {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}
impl One for Complex {
    fn one() -> Self {
        Self::new(1.0, 1.0)
    }
}
impl Debug for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}i)", self.real, self.imaginary)
    }
}
