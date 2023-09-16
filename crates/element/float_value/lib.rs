//! # カスタム浮動小数点数
//! EPSの設定を自分でやる
use prelude::*;

#[snippet(name = "float_value", doc_hidden)]
pub use float_value_impl::{FValue, EPS};
#[snippet(name = "float_value", doc_hidden)]
#[rustfmt::skip]
mod float_value_impl {
    use super::{Add, Debug, Display, Div, Formatter, Mul, Neg, Ordering, Sub};
    pub const EPS: f64 = std::f64::EPSILON;

    /// # 浮動小数点数
    /// 誤差判定をうまく行うための構造体
    #[derive(Copy, Clone, Default)]
    pub struct FValue(pub f64);
    impl FValue {
        pub fn sqrt(&self) -> Self {
            self.0.sqrt().into()
        }

        pub fn abs(&self) -> Self {
            self.0.abs().into()
        }

        pub const fn eps() -> Self {
            Self(EPS)
        }
    }
    impl PartialEq for FValue {
        fn eq(&self, other: &Self) -> bool {
            (self.0 - other.0).abs() < EPS
        }
    }
    impl Eq for FValue {}
    impl PartialOrd for FValue {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl Ord for FValue {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.partial_cmp(&other.0).expect("something went wrong")
        }
    }

    impl From<f64> for FValue {
        fn from(value: f64) -> Self {
            if value.is_nan() {
                panic!("Detected NaN.");
            }
            FValue(value)
        }
    }
    impl From<FValue> for f64 {
        fn from(value: FValue) -> Self {
            value.0
        }
    }
    impl Display for FValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Debug for FValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Add<FValue> for f64 {
        type Output = FValue;
        fn add(self, rhs: FValue) -> Self::Output {
            (self + rhs.0).into()
        }
    }
    impl<T: Into<f64>> Add<T> for FValue {
        type Output = Self;
        fn add(self, rhs: T) -> Self::Output {
            (self.0 + rhs.into()).into()
        }
    }
    impl Sub<FValue> for f64 {
        type Output = FValue;
        fn sub(self, rhs: FValue) -> Self::Output {
            (self - rhs.0).into()
        }
    }
    impl<T: Into<f64>> Sub<T> for FValue {
        type Output = Self;
        fn sub(self, rhs: T) -> Self::Output {
            (self.0 - rhs.into()).into()
        }
    }
    impl Mul<FValue> for f64 {
        type Output = FValue;
        fn mul(self, rhs: FValue) -> Self::Output {
            (self * rhs.0).into()
        }
    }
    impl<T: Into<f64>> Mul<T> for FValue {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            (self.0 * rhs.into()).into()
        }
    }
    impl Div<FValue> for f64 {
        type Output = FValue;
        fn div(self, rhs: FValue) -> Self::Output {
            (self / rhs.0).into()
        }
    }
    impl<T: Into<f64>> Div<T> for FValue {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            (self.0 / rhs.into()).into()
        }
    }
    impl Neg for FValue {
        type Output = Self;
        fn neg(self) -> Self::Output {
            (-self.0).into()
        }
    }
}

#[cfg(test)]
mod test {
    use super::FValue;
    #[test]
    fn arith() {
        let f = FValue::from(1.0);
        assert_eq!(FValue::from(3.0), f + 2.0);
        assert_eq!(FValue::from(3.0), 2.0 + f);
        assert_eq!(FValue::from(3.0), FValue::from(2.0) + f);
    }
}