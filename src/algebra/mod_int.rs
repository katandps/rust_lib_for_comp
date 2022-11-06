//! # 剰余体
//!
//! $2^{32}$を$R$とするモンゴメリ乗算を使用して実装
use crate::prelude::*;
pub use mod1000000007::{mi, Mi};

pub mod mod1000000007;
pub mod mod998244353;

#[snippet(name = "mod-int", doc_hidden)]
pub trait Mod: Copy + Clone + Debug {
    /// # 法$N$
    const MOD: u32;
    /// # $NN^{-1}$ \equiv 1 \pmod{2^32}}$ となる$N^{-1}$
    const MOD_INV: u32;
    /// # $2^{64} \pmod N$
    /// すなわち、$1$のモンゴメリ表現
    const R: u32;
    /// # $(2^{64})^2 \pmod N$
    const R_POW2: u32;
}

#[snippet(name = "mod-int", doc_hidden)]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct ModInt<M: Mod>(u32, PhantomData<fn() -> M>);

#[snippet(name = "mod-int", doc_hidden)]
mod mod_int_impl {
    use std::num::ParseIntError;

    use super::{
        Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, FromStr, Mod, ModInt, Mul,
        MulAssign, Neg, One, PhantomData, Pow, Sub, SubAssign, Sum, Zero,
    };

    impl<M: Mod> Mod for ModInt<M> {
        const MOD: u32 = M::MOD;
        const MOD_INV: u32 = M::MOD_INV;
        const R_POW2: u32 = M::R_POW2;
        const R: u32 = M::R;
    }

    impl<M: Mod> ModInt<M> {
        #[inline]
        pub fn new(mut n: u32) -> Self {
            if n >= Self::MOD {
                n = n.rem_euclid(Self::MOD);
            }
            // # モンゴメリ表現への変換
            Self(Self::mrmul(n, Self::R_POW2), PhantomData)
        }

        /// # 組み合わせnCr
        /// 前計算なし
        /// ## 計算量
        /// $M$を法として $O(r + \log M)$
        pub fn comb(n: i64, mut r: i64) -> Self {
            assert!(0 <= r && r <= n);
            if r > n - r {
                r = n - r;
            }
            let (mut ret, mut rev) = (Self::one(), Self::one());
            for k in 0..r {
                ret *= n - k;
                rev *= r - k;
            }
            ret / rev
        }

        /// # モンゴメリ表現同士の積
        /// # $mul(ar, br) == (a * b) * r \mod N$
        #[inline]
        pub fn mrmul(ar: u32, br: u32) -> u32 {
            let t: u64 = (ar as u64) * (br as u64);
            let (t, f) = ((t >> 32) as u32).overflowing_sub(
                ((((t as u32).wrapping_mul(Self::MOD_INV) as u128) * Self::MOD as u128) >> 32)
                    as u32,
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
        pub fn reduce(self) -> u32 {
            let (t, f) = (((((self.0.wrapping_mul(Self::MOD_INV)) as u128) * (Self::MOD as u128))
                >> 32) as u32)
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
    impl<M: Mod> Pow for ModInt<M> {
        #[inline]
        fn pow(mut self, mut e: i64) -> Self {
            debug_assert!(e > 0);
            let mut t = if e & 1 == 0 { M::R } else { self.0 };
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
    }
    impl<M: Mod> Add<i64> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn add(self, rhs: i64) -> Self {
            self + ModInt::from(rhs)
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
            self.0 = self.0 + rhs.0;
            if self.0 >= Self::MOD {
                self.0 -= Self::MOD
            }
        }
    }
    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::zero() - self
        }
    }
    impl<M: Mod> Sub<i64> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: i64) -> Self {
            self - ModInt::from(rhs)
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
            *self -= Self::from(rhs)
        }
    }
    impl<M: Mod> SubAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: Self) {
            self.0 = if self.0 >= rhs.0 {
                self.0 - rhs.0
            } else {
                self.0 + Self::MOD - rhs.0
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
        fn mul(mut self, rhs: Self) -> Self {
            self *= rhs;
            self
        }
    }
    impl<M: Mod> MulAssign<i64> for ModInt<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: i64) {
            *self *= Self::from(rhs);
        }
    }
    impl<M: Mod> MulAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = Self::mrmul(self.0, rhs.0)
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
            *self /= Self::from(rhs)
        }
    }
    impl<M: Mod> DivAssign<ModInt<M>> for ModInt<M> {
        #[inline]
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.pow((Self::MOD - 2) as i64)
        }
    }
    impl<M: Mod> Display for ModInt<M> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<M: Mod> Debug for ModInt<M> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<M: Mod> Sum for ModInt<M> {
        #[inline]
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |x, a| x + a)
        }
    }
    impl<M: Mod> FromStr for ModInt<M> {
        type Err = ParseIntError;
        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse::<u32>()?))
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        #[inline]
        fn from(i: i64) -> Self {
            Self::new(i.rem_euclid(Self::MOD as i64) as u32)
        }
    }
    impl<M: Mod> From<ModInt<M>> for i64 {
        #[inline]
        fn from(m: ModInt<M>) -> Self {
            m.reduce() as i64
        }
    }
    impl<M: Mod> Zero for ModInt<M> {
        #[inline]
        fn zero() -> Self {
            Self(0, PhantomData)
        }
    }
    impl<M: Mod> One for ModInt<M> {
        #[inline]
        fn one() -> Self {
            Self(M::R, PhantomData)
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
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let (mx, my) = (mi(x), mi(y));

            assert_eq!((x + y) % MOD, (mx + my).into());
            assert_eq!((x + y) % MOD, (mx + y).into());
            assert_eq!((x + MOD - y) % MOD, (mx - my).into());
            assert_eq!((x + MOD - y) % MOD, (mx - y).into());

            let (mut x, mut mx) = (x, mx);
            x += y;
            mx += my;
            assert_eq!(x % MOD, mx.into());

            mx += y;
            x += y;
            assert_eq!(x % MOD, mx.into());

            mx -= my;
            x = (x + MOD as i64 - y % MOD) % MOD;
            assert_eq!(x, mx.into());

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(x, mx.into());
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

            assert_eq!((x * y) % MOD, (mx * my).into());
            assert_eq!((x * y) % MOD, (mx * y).into());
        }
    }

    #[test]
    fn zero_test() {
        let a = Mi::new(1_000_000_000);
        let b = Mi::new(7);
        let c = a + b;
        assert_eq!(c.reduce(), 0);
    }

    #[test]
    fn pow_test() {
        let a = Mi::new(3);
        let a = a.pow(4);
        assert_eq!(a.reduce(), 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = Mi::one();
            a /= i;
            a *= i;
            assert_eq!(a.reduce(), 1);
        }
    }

    #[test]
    fn neg_test() {
        for i in 1..=100000 {
            let a = Mi::new(i);
            assert_eq!(Mi::zero(), -a + a);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(1, (Mi::from(MOD + 1)).reduce());
        assert_eq!(291172004, (Mi::from(std::i64::MAX) + 1).reduce(),);
        assert_eq!(Mi::new(1_000_000_000) * std::i64::MAX, mi(961796000));
        assert_eq!(Mi::new(1_000_000_000) + std::i64::MAX, mi(291171996));
        assert_eq!(Mi::new(1_000_000_000) - std::i64::MAX, mi(708827997));
        assert_eq!(
            (Mi::new(1_000_000_000) / std::i64::MAX * std::i64::MAX).reduce(),
            1_000_000_000
        );

        let mut a = Mi::new(1_000_000_000);
        a *= std::i64::MAX;
        assert_eq!(a.reduce(), 961796000);

        let mut a = Mi::new(1_000_000_000);
        a += std::i64::MAX;
        assert_eq!(a.reduce(), 291171996);

        let mut a = Mi::new(1_000_000_000);
        a -= std::i64::MAX;
        assert_eq!(a.reduce(), 708827997);

        let mut a = Mi::new(1_000_000_000);
        a /= std::i64::MAX;
        assert_eq!((a * std::i64::MAX).reduce(), 1_000_000_000);
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
