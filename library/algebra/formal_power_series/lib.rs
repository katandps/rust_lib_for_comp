//! # 形式的冪級数
use mod_int::ModInt;
use prelude::*;

mod formal_power_series_impl {
    use super::{Add, ModInt, Neg, Sub};

    #[derive(Clone, Debug, Eq)]
    pub struct FormalPowerSeries(Vec<ModInt>);

    impl FormalPowerSeries {
        fn len(&self) -> usize {
            self.0.len()
        }
    }

    impl PartialEq for FormalPowerSeries {
        fn eq(&self, other: &Self) -> bool {
            if self.len() < other.len() {
                for i in 0..self.len() {
                    if self.0[i] != other.0[i] {
                        return false;
                    }
                }
                for i in self.len()..other.len() {
                    if other.0[i] != ModInt::zero() {
                        return false;
                    }
                }
            } else {
                for i in 0..other.len() {
                    if self.0[i] != other.0[i] {
                        return false;
                    }
                }
                for i in other.len()..self.len() {
                    if self.0[i] != ModInt::zero() {
                        return false;
                    }
                }
            }
            true
        }
    }

    impl Add<Self> for FormalPowerSeries {
        type Output = Self;
        fn add(mut self, mut rhs: Self) -> Self {
            if self.0.len() < rhs.0.len() {
                for i in 0..self.0.len() {
                    rhs.0[i] += self.0[i];
                }
                rhs
            } else {
                for i in 0..rhs.0.len() {
                    self.0[i] += rhs.0[i]
                }
                self
            }
        }
    }
    impl Neg for FormalPowerSeries {
        type Output = Self;
        fn neg(mut self) -> Self {
            for i in 0..self.len() {
                self.0[i] = -self.0[i]
            }
            self
        }
    }
    impl Sub<Self> for FormalPowerSeries {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self + (-rhs)
        }
    }
}
