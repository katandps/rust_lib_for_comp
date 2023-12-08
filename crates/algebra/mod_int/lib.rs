//! # 剰余体
//!
//! $2^{32}$を$R$とするモンゴメリ乗算を使用して実装
use algebra::*;
use fxhasher::HashMap;
use prelude::*;

pub type Mi = ModInt<998_244_353>;

#[snippet(name = "mod-int", doc_hidden)]
pub use mod_int_impl::ModInt;
#[snippet(name = "mod-int", doc_hidden)]
mod mod_int_impl {
    use std::num::ParseIntError;

    use super::{
        Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, FromStr, Mul, MulAssign, Neg,
        One, Pow, PrimitiveRoot, Sub, SubAssign, Sum, Zero,
    };
    #[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
    pub struct ModInt<const MOD: u32 = 998_244_353>(u32);

    impl<const MOD: u32> ModInt<MOD> {
        /// # 法$N$
        pub const MOD: u32 = MOD;
        /// # $NN^{-1}$ \equiv 1 \pmod{2^32}}$ となる$N^{-1}$
        pub const MOD_INV: u32 = {
            let (mut n_inv, mut i) = (Self::MOD, 0);
            while i < 5 {
                n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(Self::MOD.wrapping_mul(n_inv)));
                i += 1;
            }
            n_inv
        };
        /// # $2^{64} \pmod N$
        /// すなわち、$1$のモンゴメリ表現
        pub const R: u32 = Self::MOD.wrapping_neg() % Self::MOD;
        /// # $(2^{64})^2 \pmod N$
        pub const R_POW2: u32 = ((Self::MOD as u64).wrapping_neg() % Self::MOD as u64) as u32;

        #[inline]
        pub const fn new(mut n: u32) -> Self {
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

        /// # 組み合わせnCr
        /// 前計算なし
        /// ## 計算量
        /// $M$を法として $O(r + \log M)$
        pub const fn comb(n: i64, mut r: i64) -> Self {
            assert!(0 <= r && r <= n);
            if r > n - r {
                r = n - r;
            }
            let (mut ret, mut rev) = (Self::one(), Self::one());
            let mut i = 0;
            while i < r {
                ret = Self::mul(&ret, Self::new((n - i) as u32));
                rev = Self::mul(&rev, Self::new((r - i) as u32));
                i += 1;
            }
            Self::div(&ret, rev)
        }

        /// # モンゴメリ表現同士の積
        /// # $mul(ar, br) == (a * b) * r \mod N$
        #[inline]
        pub const fn mrmul(ar: u32, br: u32) -> u32 {
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
        pub const fn reduce(self) -> u32 {
            let (t, f) = (((((self.0.wrapping_mul(Self::MOD_INV)) as u64) * (Self::MOD as u64))
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
    impl<const M: u32> Pow for ModInt<M> {
        #[inline]
        fn pow(self, e: i64) -> Self {
            Self::pow(self, e)
        }
    }
    impl<Rhs: Into<Self>, const M: u32> Add<Rhs> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Rhs) -> Self {
            Self::add(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const M: u32> AddAssign<Rhs> for ModInt<M> {
        #[inline]
        fn add_assign(&mut self, rhs: Rhs) {
            self.0 = Self::add(self, rhs.into()).0
        }
    }
    impl<const M: u32> Neg for ModInt<M> {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::zero() - self
        }
    }
    impl<Rhs: Into<Self>, const M: u32> Sub<Rhs> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Rhs) -> Self {
            Self::sub(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const M: u32> SubAssign<Rhs> for ModInt<M> {
        #[inline]
        fn sub_assign(&mut self, rhs: Rhs) {
            self.0 = Self::sub(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>, const M: u32> Mul<Rhs> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Rhs) -> Self {
            Self::mul(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const M: u32> MulAssign<Rhs> for ModInt<M> {
        #[inline]
        fn mul_assign(&mut self, rhs: Rhs) {
            self.0 = Self::mul(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>, const M: u32> Div<Rhs> for ModInt<M> {
        type Output = Self;
        #[inline]
        fn div(self, rhs: Rhs) -> Self {
            Self::div(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>, const M: u32> DivAssign<Rhs> for ModInt<M> {
        #[inline]
        fn div_assign(&mut self, rhs: Rhs) {
            self.0 = Self::div(self, rhs.into()).0
        }
    }
    impl<const M: u32> Display for ModInt<M> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<const M: u32> Debug for ModInt<M> {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl<const M: u32> Sum for ModInt<M> {
        #[inline]
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |x, a| x + a)
        }
    }
    impl<const M: u32> FromStr for ModInt<M> {
        type Err = ParseIntError;
        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse::<u32>()?))
        }
    }
    macro_rules! impl_integral {
        ($($ty:ty),*) => {
            $(
                impl<const M: u32> From<$ty> for ModInt<M> {
                    #[inline]
                    fn from(i: $ty) -> Self {
                        Self::new(i.rem_euclid(Self::MOD as $ty) as u32)
                    }
                }
            )*
        };
    }
    impl_integral!(i32, i64, i128, isize, u32, u64, u128, usize);
    impl<const M: u32> From<ModInt<M>> for i64 {
        #[inline]
        fn from(m: ModInt<M>) -> Self {
            m.reduce() as i64
        }
    }
    impl<const M: u32> Zero for ModInt<M> {
        #[inline]
        fn zero() -> Self {
            Self::zero()
        }
    }
    impl<const M: u32> One for ModInt<M> {
        #[inline]
        fn one() -> Self {
            Self::one()
        }
    }

    /// # $\mod 167_772_161$
    /// $167772161=2^{25}*5+1$
    impl PrimitiveRoot for ModInt<167_772_161> {
        const DIVIDE_LIMIT: usize = 25;
        #[inline]
        fn primitive_root() -> Self {
            let exp = (Self::zero() - 1) / Self::new(2).pow(Self::DIVIDE_LIMIT as i64);
            Self::pow(Self::new(3), exp.into())
        }
    }

    /// # $\mod 469_762_049$
    /// $469762049=2^{26}*7+1$
    impl PrimitiveRoot for ModInt<469_762_049> {
        const DIVIDE_LIMIT: usize = 26;
        #[inline]
        fn primitive_root() -> Self {
            let exp = (Self::zero() - 1) / Self::new(2).pow(Self::DIVIDE_LIMIT as i64);
            Self::pow(Self::new(3), exp.into())
        }
    }

    /// # $\mod 998_244_353$
    /// $1224736769=2^{23}*7*17$
    impl PrimitiveRoot for ModInt<998_244_353> {
        const DIVIDE_LIMIT: usize = 23;
        #[inline]
        fn primitive_root() -> Self {
            let exp = (Self::zero() - 1) / Self::new(2).pow(Self::DIVIDE_LIMIT as i64);
            Self::pow(Self::new(3), exp.into())
        }
    }

    /// # $\mod 1_224_736_769$
    /// $1224736769=2^{24}*73$
    impl PrimitiveRoot for ModInt<1_224_736_769> {
        const DIVIDE_LIMIT: usize = 24;
        #[inline]
        fn primitive_root() -> Self {
            let exp = (Self::zero() - 1) / Self::new(2).pow(Self::DIVIDE_LIMIT as i64);
            Self::pow(Self::new(3), exp.into())
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
        *self.0.entry(e).or_insert_with(|| Mi::new(2).pow(e))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    type Mi = ModInt<1_000_000_007>;
    const MOD: i64 = Mi::MOD as i64;

    #[test]
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let (mx, my) = (Mi::new(x as u32), Mi::new(y as u32));

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

            let (mx, my) = (Mi::new(x as u32), Mi::new(y as u32));

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
        assert_eq!(Mi::new(1_000_000_000) * std::i64::MAX, Mi::new(961796000));
        assert_eq!(Mi::new(1_000_000_000) + std::i64::MAX, Mi::new(291171996));
        assert_eq!(Mi::new(1_000_000_000) - std::i64::MAX, Mi::new(708827997));
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
        assert_eq!(Mi::new(10), Mi::comb(5, 2));
        assert_eq!(Mi::new(10), Mi::comb(5, 3));
        assert_eq!(
            Mi::new(1) * 1000000007 * 1000000008 * 1000000009 / 6,
            Mi::comb(MOD + 2, 3)
        );
    }

    #[test]
    fn display() {
        assert_eq!("1", &format!("{}", Mi::new(1)));
    }

    #[test]
    fn from_str() {
        assert_eq!(Ok(Mi::new(5)), Mi::from_str("5"));
        assert_eq!(Ok(Mi::new(1)), Mi::from_str("1000000008"));
        assert!(Mi::from_str("5a").is_err());
    }

    #[test]
    fn const_test() {
        type Mi = ModInt<998_244_353>;
        assert_eq!(Mi::MOD.wrapping_mul(Mi::MOD_INV), 1);
        assert_eq!(((1u64 << 32) % Mi::MOD as u64) as u32, Mi::R);
        assert_eq!(
            ((Mi::MOD as u64).wrapping_neg() % Mi::MOD as u64) as u32,
            Mi::R_POW2
        );
    }

    #[test]
    fn primitive_root() {
        primitive_root_inner_test(
            ModInt::<167_772_161>::DIVIDE_LIMIT,
            ModInt::<167_772_161>::primitive_root(),
        );
        primitive_root_inner_test(
            ModInt::<469_762_049>::DIVIDE_LIMIT,
            ModInt::<469_762_049>::primitive_root(),
        );
        primitive_root_inner_test(
            ModInt::<998_244_353>::DIVIDE_LIMIT,
            ModInt::<998_244_353>::primitive_root(),
        );
        primitive_root_inner_test(
            ModInt::<1_224_736_769>::DIVIDE_LIMIT,
            ModInt::<1_224_736_769>::primitive_root(),
        );
    }

    fn primitive_root_inner_test<const M: u32>(divide_limit: usize, primitive_root: ModInt<M>) {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for i in 0..divide_limit {
            let n = primitive_root.pow((1 << i).into());
            set.insert(n);
            assert_eq!(n.pow(1 << (divide_limit - i)), ModInt::<M>::one());
        }
        assert_eq!(set.len(), divide_limit);
    }
}
