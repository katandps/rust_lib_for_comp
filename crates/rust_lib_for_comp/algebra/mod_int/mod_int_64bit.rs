//! # mod_int(64bit version)
//!
//! モンゴメリ乗算を利用する
//! 64bit長のModIntが必要なときに
use crate::algebra::*;

#[codesnip::entry("mod-int-64", include("algebra", "prelude"))]
pub use mod_int_impl::ModInt64;
#[codesnip::entry("mod-int-64", include("algebra", "prelude"))]
mod mod_int_impl {
    use std::num::ParseIntError;

    use super::{
        Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, FromStr, Mul, MulAssign, Neg,
        One, Pow, Sub, SubAssign, Sum, Zero,
    };
    #[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
    pub struct ModInt64<const MOD: u64 = { (1 << 61) - 1 }>(u64);

    impl<const MOD: u64> ModInt64<MOD> {
        /// # 法$N$
        pub const MOD: u64 = MOD;
        /// # $NN^{-1}$ \equiv 1 \pmod{2^32}}$ となる$N^{-1}$
        pub const MOD_INV: u64 = {
            let (mut n_inv, mut i) = (Self::MOD, 0);
            while i < 5 {
                n_inv = n_inv.wrapping_mul(2u64.wrapping_sub(Self::MOD.wrapping_mul(n_inv)));
                i += 1;
            }
            n_inv
        };
        /// # $2^{64} \pmod N$
        /// すなわち、$1$のモンゴメリ表現
        pub const R: u64 = Self::MOD.wrapping_neg() % Self::MOD;
        /// # $(2^{64})^2 \pmod N$
        pub const R_POW2: u64 =
            ((Self::MOD as u128).wrapping_neg() % (Self::MOD as u128)) as u64 % Self::MOD;

        #[inline]
        pub const fn new(mut n: u64) -> Self {
            if n >= Self::MOD {
                n = n.rem_euclid(Self::MOD);
            }
            // # モンゴメリ表現への変換
            Self(Self::mrmul(n, Self::R_POW2))
        }

        pub const fn one() -> Self {
            Self(Self::R)
        }

        pub const fn zero() -> Self {
            Self(0)
        }
        pub const fn add(&self, rhs: Self) -> Self {
            let mut x = self.0 + rhs.0;
            if x >= Self::MOD {
                x -= Self::MOD
            }
            Self(x)
        }
        pub const fn sub(&self, rhs: Self) -> Self {
            let x = if self.0 >= rhs.0 {
                self.0 - rhs.0
            } else {
                self.0 + Self::MOD - rhs.0
            };
            Self(x)
        }

        pub const fn mul(&self, rhs: Self) -> Self {
            Self(Self::mrmul(self.0, rhs.0))
        }
        pub const fn div(&self, rhs: Self) -> Self {
            Self::mul(self, rhs.pow(Self::MOD as i64 - 2))
        }

        pub const fn pow(mut self, mut e: i64) -> Self {
            debug_assert!(e >= 0);
            if e == 0 {
                return Self::one();
            }
            let mut t = if e & 1 == 0 { Self::R } else { self.0 };
            e >>= 1;
            while e != 0 {
                self.0 = Self::mrmul(self.0, self.0);
                if e & 1 != 0 {
                    t = Self::mrmul(t, self.0);
                }
                e >>= 1;
            }
            self.0 = t;
            self
        }

        /// # モンゴメリ表現同士の積
        /// $mul(ar, br) == (a * b) * r \mod N$
        ///
        /// ## todo
        /// 128bit演算を使用しないようにする
        #[inline]
        pub const fn mrmul(ar: u64, br: u64) -> u64 {
            let t: u128 = (ar as u128) * (br as u128);
            let (t, f) = ((t >> 64) as u64).overflowing_sub(
                ((((t as u64).wrapping_mul(Self::MOD_INV) as u128) * Self::MOD as u128) >> 64)
                    as u64,
            );
            if f {
                t.wrapping_add(Self::MOD)
            } else {
                t
            }
        }

        /// # モンゴメリ表現 $AR$ から $A$の復元
        /// return $a \frac R \mod N$
        #[inline]
        pub const fn reduce(self) -> u64 {
            let (t, f) = (((((self.0.wrapping_mul(Self::MOD_INV)) as u128) * (Self::MOD as u128))
                >> 64) as u64)
                .overflowing_neg();
            if f {
                t.wrapping_add(Self::MOD)
            } else {
                t
            }
        }
    }

    /// # 累乗
    /// ## 計算量
    /// $M$を法として $ O(\log M) $
    impl<const MOD: u64> Pow for ModInt64<MOD> {
        #[inline]
        fn pow(self, e: i64) -> Self {
            Self::pow(self, e)
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Add<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Rhs) -> Self {
            Self::add(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> AddAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn add_assign(&mut self, rhs: Rhs) {
            self.0 = Self::add(self, rhs.into()).0
        }
    }
    impl<const MOD: u64> Neg for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::zero() - self
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Sub<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Rhs) -> Self {
            Self::sub(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> SubAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn sub_assign(&mut self, rhs: Rhs) {
            self.0 = Self::sub(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Mul<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Rhs) -> Self {
            Self::mul(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> MulAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn mul_assign(&mut self, rhs: Rhs) {
            self.0 = Self::mul(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> Div<Rhs> for ModInt64<MOD> {
        type Output = Self;
        #[inline]
        fn div(self, rhs: Rhs) -> Self {
            Self::div(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const MOD: u64> DivAssign<Rhs> for ModInt64<MOD> {
        #[inline]
        fn div_assign(&mut self, rhs: Rhs) {
            self.0 = Self::div(self, rhs.into()).0
        }
    }
    impl<const MOD: u64> Display for ModInt64<MOD> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<const MOD: u64> Debug for ModInt64<MOD> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<const MOD: u64> Sum for ModInt64<MOD> {
        #[inline]
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |x, a| x + a)
        }
    }
    impl<const MOD: u64> FromStr for ModInt64<MOD> {
        type Err = ParseIntError;
        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse::<u64>()?))
        }
    }
    macro_rules! impl_integral {
        ($($ty:ty),*) => {
            $(
                impl<const MOD: u64> From<$ty> for ModInt64<MOD> {
                    #[inline]
                    fn from(i: $ty) -> Self {
                        Self::new((i as i128).rem_euclid(Self::MOD as i128) as u64)
                    }
                }
            )*
        };
    }
    impl_integral!(i32, i64, i128, isize, u32, u64, u128, usize);
    impl<const MOD: u64> From<ModInt64<MOD>> for i64 {
        #[inline]
        fn from(m: ModInt64<MOD>) -> Self {
            m.reduce() as i64
        }
    }
    impl<const MOD: u64> Zero for ModInt64<MOD> {
        #[inline]
        fn zero() -> Self {
            Self::zero()
        }
    }
    impl<const MOD: u64> One for ModInt64<MOD> {
        #[inline]
        fn one() -> Self {
            Self::one()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ModInt64;

    type Mi = ModInt64<{ (1 << 61) - 1 }>;

    #[test]
    fn test() {
        let one = Mi::one();
        assert_eq!("1".to_string(), one.to_string());
        assert_eq!("100000".to_string(), Mi::from(100000).to_string());
    }
}
