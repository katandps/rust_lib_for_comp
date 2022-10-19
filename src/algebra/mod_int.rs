//! # 剰余体
//!
//! ## dependency
//! [MontgomeryMultiplication](super::montgomery_multiplication)
use crate::prelude::*;
pub use mod1000000007::{mi, Mi};

use super::montgomery_multiplication::MontgomeryReduction;

pub mod mod1000000007;
pub mod mod998244353;

#[snippet(name = "mod-int", doc_hidden)]
pub trait Mod: Copy + Clone + Debug {
    fn get() -> mod_int_impl::InnerType;
    fn mont() -> MontgomeryReduction;
}

#[snippet(name = "mod-int", doc_hidden)]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct ModInt<M: Mod>(mod_int_impl::InnerType, PhantomData<fn() -> M>);

#[snippet(name = "mod-int", doc_hidden)]
mod mod_int_impl {
    use std::num::ParseIntError;

    use super::{
        Add, AddAssign, Debug, Deref, DerefMut, Display, Div, DivAssign, Formatter, FromStr, Mod,
        ModInt, Mul, MulAssign, Neg, One, PhantomData, Pow, Sub, SubAssign, Sum, Zero,
    };

    pub type InnerType = i64;
    impl<M: Mod> ModInt<M> {
        pub fn new(mut n: InnerType) -> Self {
            if n < 0 || n >= M::get() {
                n = n.rem_euclid(M::get());
            }
            Self(n, PhantomData)
        }

        /// # 組み合わせnCr
        /// 前計算なし
        /// ## 計算量
        /// $M$を法として $O(r + \log M)$
        pub fn comb(n: i64, mut r: i64) -> Self {
            if r > n - r {
                r = n - r;
            }
            if r == 0 {
                return Self::new(1);
            }
            let (mut ret, mut rev) = (Self::new(1), Self::new(1));
            for k in 0..r {
                ret *= n - k;
                rev *= r - k;
            }
            ret / rev
        }

        pub fn get(self) -> InnerType {
            self.0
        }
    }

    /// # 累乗
    /// ## 計算量
    /// $M$を法として $ O(\log M) $
    impl<M: Mod> Pow for ModInt<M> {
        fn pow(self, mut e: i64) -> Self {
            let m = e < 0;
            e = e.abs();
            let t = M::mont().reduce(M::mont().pow(self.0 as u64, e as u64));
            if m {
                Self::new(1) / t as i64
            } else {
                Self::new(t as i64)
            }
        }
    }
    impl<M: Mod> Add<i64> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn add(self, rhs: i64) -> Self {
            self + ModInt::new(rhs)
        }
    }
    impl<M: Mod> Add<ModInt<M>> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn add(mut self, rhs: Self) -> Self {
            self += rhs;
            self
        }
    }
    impl<M: Mod> AddAssign<i64> for ModInt<M> {
        #[inline]
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs
        }
    }
    impl<M: Mod> AddAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn add_assign(&mut self, rhs: Self) {
            self.0 = if self.0 + rhs.0 >= M::get() {
                self.0 + rhs.0 - M::get()
            } else {
                self.0 + rhs.0
            }
        }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::new(-self.0)
        }
    }
    impl<M: Mod> Sub<i64> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: i64) -> Self {
            self - ModInt::new(rhs)
        }
    }
    impl<M: Mod> Sub<ModInt<M>> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn sub(mut self, rhs: Self) -> Self {
            self -= rhs;
            self
        }
    }
    impl<M: Mod> SubAssign<i64> for ModInt<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: i64) {
            *self -= Self::new(rhs)
        }
    }
    impl<M: Mod> SubAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.0 = if self.0 >= rhs.0 {
                self.0 - rhs.0
            } else {
                self.0 + M::get() - rhs.0
            }
        }
    }
    impl<M: Mod> Mul<i64> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn mul(mut self, rhs: i64) -> Self {
            self *= rhs;
            self
        }
    }
    impl<M: Mod> Mul<ModInt<M>> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Self) -> Self {
            self * rhs.0
        }
    }
    impl<M: Mod> MulAssign<i64> for ModInt<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: i64) {
            *self *= Self::new(rhs);
        }
    }
    impl<M: Mod> MulAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = M::mont().mul_prim(self.0 as u64, rhs.0 as u64) as i64
        }
    }
    impl<M: Mod> Div<i64> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn div(mut self, rhs: i64) -> Self {
            self /= rhs;
            self
        }
    }
    impl<M: Mod> Div<ModInt<M>> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn div(mut self, rhs: Self) -> Self {
            self /= rhs;
            self
        }
    }
    impl<M: Mod> DivAssign<i64> for ModInt<M> {
        #[inline]
        fn div_assign(&mut self, rhs: i64) {
            *self /= Self::new(rhs)
        }
    }
    impl<M: Mod> DivAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.pow(M::get() - 2)
        }
    }
    impl<M: Mod> Display for ModInt<M> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<M: Mod> Debug for ModInt<M> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<M: Mod> Deref for ModInt<M> {
        type Target = i64;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: Mod> DerefMut for ModInt<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M: Mod> Sum for ModInt<M> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::new(0), |x, a| x + a)
        }
    }
    impl<M: Mod> FromStr for ModInt<M> {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse::<i64>()?))
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(i: i64) -> Self {
            Self::new(i)
        }
    }
    impl<M: Mod> From<ModInt<M>> for i64 {
        fn from(m: ModInt<M>) -> Self {
            m.0
        }
    }
    impl<M: Mod> Zero for ModInt<M> {
        fn zero() -> Self {
            Self::new(0)
        }
    }
    impl<M: Mod> One for ModInt<M> {
        fn one() -> Self {
            Self::new(1)
        }
    }
}

#[snippet(name = "pow-table", doc_hidden)]
#[derive(Clone, Debug, Default)]
/// # 2のべき乗を都度生成するDefaultDict
pub struct PowTable(HashMap<i64, Mi>);
#[snippet(name = "pow-table", doc_hidden)]
impl PowTable {
    pub fn pow(&mut self, e: i64) -> Mi {
        *self.0.entry(e).or_insert_with(|| mi(2).pow(e))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    const MOD: i64 = 1_000_000_007;

    #[test]
    fn neg_test() {
        assert_eq!(Mi::new(0) - 1_000_000, Mi::new(-1_000_000));
    }

    #[test]
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let (mx, my) = (mi(x), mi(y));

            assert_eq!((mx + my).get(), (x + y) % MOD);
            assert_eq!((mx + y).get(), (x + y) % MOD);
            assert_eq!((mx - my).get(), (x + MOD - y) % MOD);
            assert_eq!((mx - y).get(), (x + MOD - y) % MOD);

            let (mut x, mut mx) = (x, mx);
            x += y;
            mx += my;
            assert_eq!(mx.get(), x % MOD);

            mx += y;
            x += y;
            assert_eq!(mx.get(), x % MOD);

            mx -= my;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.get(), x);

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.get(), x);
        }
    }

    #[test]
    fn random_mul() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let (mx, my) = (mi(x), mi(y));

            assert_eq!((mx * my).get(), (x * y) % MOD);
            assert_eq!((mx * y).get(), (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = Mi::new(1_000_000_000);
        let b = Mi::new(7);
        let c = a + b;
        assert_eq!(c.get(), 0);
    }

    #[test]
    fn pow_test() {
        let a = Mi::new(3);
        let a = a.pow(4);
        assert_eq!(a.get(), 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = Mi::new(1);
            a /= i;
            a *= i;
            assert_eq!(a.get(), 1);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!((Mi::new(MOD + 1)).get(), 1);
        assert_eq!((Mi::new(std::i64::MAX) + 1).get(), 291172004);
        assert_eq!(Mi::new(1_000_000_000) * std::i64::MAX, mi(961796000));
        assert_eq!(Mi::new(1_000_000_000) + std::i64::MAX, mi(291171996));
        assert_eq!(Mi::new(1_000_000_000) - std::i64::MAX, mi(708827997));
        assert_eq!(
            (Mi::new(1_000_000_000) / std::i64::MAX * std::i64::MAX).get(),
            1_000_000_000
        );

        let mut a = Mi::new(1_000_000_000);
        a *= std::i64::MAX;
        assert_eq!(a.get(), 961796000);

        let mut a = Mi::new(1_000_000_000);
        a += std::i64::MAX;
        assert_eq!(a.get(), 291171996);

        let mut a = Mi::new(1_000_000_000);
        a -= std::i64::MAX;
        assert_eq!(a.get(), 708827997);

        let mut a = Mi::new(1_000_000_000);
        a /= std::i64::MAX;
        assert_eq!((a * std::i64::MAX).get(), 1_000_000_000);
    }

    #[test]
    fn comb() {
        assert_eq!(mi(10), Mi::comb(5, 2));
        assert_eq!(
            mi(1) * 1000000007 * 1000000008 * 1000000009 / 6,
            Mi::comb(MOD + 2, 3)
        );
    }
}
