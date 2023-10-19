//! # 剰余体
//!
//! $2^{32}$を$R$とするモンゴメリ乗算を使用して実装
use algebra::*;
use prelude::*;

#[snippet(name = "dynamic-mod-int", doc_hidden)]
pub use dynamic_mod_int_impl::ModInt;
#[snippet(name = "dynamic-mod-int", doc_hidden)]
// #[rustfmt::skip]
mod dynamic_mod_int_impl {
    use std::num::ParseIntError;
    use std::sync::OnceLock;

    use super::{
        Add, AddAssign, Debug, Display, Div, DivAssign, Formatter, FromStr, Mul, MulAssign, Neg,
        One, Pow, Sub, SubAssign, Sum, Zero,
    };

    static MOD: OnceLock<u32> = OnceLock::new();
    static MOD_INV: OnceLock<u32> = OnceLock::new();
    static R: OnceLock<u32> = OnceLock::new();
    static R_POW2: OnceLock<u32> = OnceLock::new();

    #[derive(Copy, Clone, Eq, PartialEq, Default, Hash)]
    pub struct ModInt(u32);

    impl ModInt {
        #[inline]
        fn get_mod() -> u32 {
            *MOD.get().expect("uninitialized mod int")
        }
        #[inline]
        fn get_mod_inv() -> u32 {
            *MOD_INV.get().expect("uninitialized mod int")
        }
        #[inline]
        fn get_r() -> u32 {
            *R.get().expect("uninitialized mod int")
        }
        #[inline]
        fn get_r_pow2() -> u32 {
            *R_POW2.get().expect("uninitialized mod int")
        }
        #[inline]
        pub fn set_mod(m: u32) {
            if let Err(e) = MOD.set(m) {
                dbg!("mod is already initialized", e);
            }
            let m = Self::get_mod();
            if let Err(e) = MOD_INV.set({
                let (mut n_inv, mut i) = (m, 0);
                while i < 5 {
                    n_inv = n_inv.wrapping_mul(2u32.wrapping_sub(m.wrapping_mul(n_inv)));
                    i += 1;
                }
                n_inv
            }) {
                dbg!("mod_inv is already initialized", e);
            }
            if let Err(e) = R.set(m.wrapping_neg() % m) {
                dbg!("r is already initialized", e);
            }
            if let Err(e) = R_POW2.set(((m as u64).wrapping_neg() % m as u64) as u32) {
                dbg!("r_pow2 is already initialized", e);
            }
        }

        #[inline]
        pub fn new(mut n: u32) -> Self {
            if n >= Self::get_mod() {
                n = n.rem_euclid(Self::get_mod());
            }
            // # モンゴメリ表現への変換
            Self(Self::mrmul(n, Self::get_r_pow2()))
        }

        pub fn one() -> Self {
            Self(Self::get_r())
        }

        pub fn zero() -> Self {
            Self(0)
        }
        pub fn add(&self, rhs: Self) -> Self {
            let mut x = self.0 + rhs.0;
            if x >= Self::get_mod() {
                x -= Self::get_mod()
            }
            Self(x)
        }
        pub fn sub(&self, rhs: Self) -> Self {
            let x = if self.0 >= rhs.0 {
                self.0 - rhs.0
            } else {
                self.0 + Self::get_mod() - rhs.0
            };
            Self(x)
        }

        pub fn mul(&self, rhs: Self) -> Self {
            Self(Self::mrmul(self.0, rhs.0))
        }
        pub fn div(&self, rhs: Self) -> Self {
            Self::mul(self, rhs.pow(Self::get_mod() as i64 - 2))
        }

        pub fn pow(mut self, mut e: i64) -> Self {
            debug_assert!(e > 0);
            let mut t = if e & 1 == 0 { Self::get_r() } else { self.0 };
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
        pub fn comb(n: i64, mut r: i64) -> Self {
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
        pub fn mrmul(ar: u32, br: u32) -> u32 {
            let t: u64 = (ar as u64) * (br as u64);
            let (t, f) = ((t >> 32) as u32).overflowing_sub(
                ((((t as u32).wrapping_mul(Self::get_mod_inv()) as u128) * Self::get_mod() as u128)
                    >> 32) as u32,
            );
            if f {
                t.wrapping_add(Self::get_mod())
            } else {
                t
            }
        }

        /// # モンゴメリ表現 $AR$ から $A$の復元
        /// return $a \frac R \mod N$
        #[inline]
        pub fn reduce(self) -> u32 {
            let (t, f) = (((((self.0.wrapping_mul(Self::get_mod_inv())) as u128)
                * (Self::get_mod() as u128))
                >> 32) as u32)
                .overflowing_neg();
            if f {
                t.wrapping_add(Self::get_mod())
            } else {
                t
            }
        }
    }

    /// # 累乗
    /// ## 計算量
    /// $M$を法として $ O(\log M) $
    impl Pow for ModInt {
        #[inline]
        fn pow(self, e: i64) -> Self {
            Self::pow(self, e)
        }
    }
    impl<Rhs: Into<Self>> Add<Rhs> for ModInt {
        type Output = Self;
        #[inline]
        fn add(self, rhs: Rhs) -> Self {
            Self::add(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>> AddAssign<Rhs> for ModInt {
        #[inline]
        fn add_assign(&mut self, rhs: Rhs) {
            self.0 = Self::add(self, rhs.into()).0
        }
    }
    impl Neg for ModInt {
        type Output = Self;
        #[inline]
        fn neg(self) -> Self {
            Self::zero() - self
        }
    }
    impl<Rhs: Into<Self>> Sub<Rhs> for ModInt {
        type Output = Self;
        #[inline]
        fn sub(self, rhs: Rhs) -> Self {
            Self::sub(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>> SubAssign<Rhs> for ModInt {
        #[inline]
        fn sub_assign(&mut self, rhs: Rhs) {
            self.0 = Self::sub(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>> Mul<Rhs> for ModInt {
        type Output = Self;
        #[inline]
        fn mul(self, rhs: Rhs) -> Self {
            Self::mul(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>> MulAssign<Rhs> for ModInt {
        #[inline]
        fn mul_assign(&mut self, rhs: Rhs) {
            self.0 = Self::mul(self, rhs.into()).0
        }
    }
    impl<Rhs: Into<Self>> Div<Rhs> for ModInt {
        type Output = Self;
        #[inline]
        fn div(self, rhs: Rhs) -> Self {
            Self::div(&self, rhs.into())
        }
    }
    impl<Rhs: Into<Self>> DivAssign<Rhs> for ModInt {
        #[inline]
        fn div_assign(&mut self, rhs: Rhs) {
            self.0 = Self::div(self, rhs.into()).0
        }
    }
    impl Display for ModInt {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl Debug for ModInt {
        #[inline]
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.reduce())
        }
    }
    impl Sum for ModInt {
        #[inline]
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(Self::zero(), |x, a| x + a)
        }
    }
    impl FromStr for ModInt {
        type Err = ParseIntError;
        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self::new(s.parse::<u32>()?))
        }
    }
    macro_rules! impl_integral {
        ($($ty:ty),*) => {
            $(
                impl From<$ty> for ModInt {
                    #[inline]
                    fn from(i: $ty) -> Self {
                        Self::new(i.rem_euclid(Self::get_mod() as $ty) as u32)
                    }
                }
            )*
        };
    }
    impl_integral!(i32, i64, i128, isize, u32, u64, u128, usize);
    impl From<ModInt> for i64 {
        #[inline]
        fn from(m: ModInt) -> Self {
            m.reduce() as i64
        }
    }
    impl Zero for ModInt {
        #[inline]
        fn zero() -> Self {
            Self::zero()
        }
    }
    impl One for ModInt {
        #[inline]
        fn one() -> Self {
            Self::one()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    const MOD: i64 = 1_000_000_007;

    #[test]
    fn random_add_sub() {
        ModInt::set_mod(MOD as u32);
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let (mx, my) = (ModInt::from(x), ModInt::from(y));

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
        ModInt::set_mod(MOD as u32);
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let (mx, my) = (ModInt::new(x as u32), ModInt::new(y as u32));

            assert_eq!((x * y) % MOD, (mx * my).into());
            assert_eq!((x * y) % MOD, (mx * y).into());
        }
    }

    #[test]
    fn zero_test() {
        ModInt::set_mod(MOD as u32);
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;
        assert_eq!(c.reduce(), 0);
    }

    #[test]
    fn pow_test() {
        ModInt::set_mod(MOD as u32);
        let a = ModInt::new(3);
        let a = a.pow(4);
        assert_eq!(a.reduce(), 81);
    }

    #[test]
    fn div_test() {
        ModInt::set_mod(MOD as u32);
        for i in 1..100000 {
            let mut a = ModInt::one();
            a /= i;
            a *= i;
            assert_eq!(a.reduce(), 1);
        }
    }

    #[test]
    fn neg_test() {
        ModInt::set_mod(MOD as u32);
        for i in 1..=100000 {
            let a = ModInt::new(i);
            assert_eq!(ModInt::zero(), -a + a);
        }
    }

    #[test]
    fn edge_cases() {
        ModInt::set_mod(MOD as u32);
        assert_eq!(1, (ModInt::from(MOD + 1)).reduce());
        assert_eq!(291172004, (ModInt::from(std::i64::MAX) + 1).reduce(),);
        assert_eq!(
            ModInt::new(1_000_000_000) * std::i64::MAX,
            ModInt::new(961796000)
        );
        assert_eq!(
            ModInt::new(1_000_000_000) + std::i64::MAX,
            ModInt::new(291171996)
        );
        assert_eq!(
            ModInt::new(1_000_000_000) - std::i64::MAX,
            ModInt::new(708827997)
        );
        assert_eq!(
            (ModInt::new(1_000_000_000) / std::i64::MAX * std::i64::MAX).reduce(),
            1_000_000_000
        );

        let mut a = ModInt::new(1_000_000_000);
        a *= std::i64::MAX;
        assert_eq!(a.reduce(), 961796000);

        let mut a = ModInt::new(1_000_000_000);
        a += std::i64::MAX;
        assert_eq!(a.reduce(), 291171996);

        let mut a = ModInt::new(1_000_000_000);
        a -= std::i64::MAX;
        assert_eq!(a.reduce(), 708827997);

        let mut a = ModInt::new(1_000_000_000);
        a /= std::i64::MAX;
        assert_eq!((a * std::i64::MAX).reduce(), 1_000_000_000);
    }

    #[test]
    fn comb() {
        ModInt::set_mod(MOD as u32);
        assert_eq!(ModInt::new(10), ModInt::comb(5, 2));
        assert_eq!(ModInt::new(10), ModInt::comb(5, 3));
        assert_eq!(
            ModInt::new(1) * 1000000007 * 1000000008 * 1000000009 / 6,
            ModInt::comb(MOD + 2, 3)
        );
    }

    #[test]
    fn display() {
        ModInt::set_mod(MOD as u32);
        assert_eq!("1", &format!("{}", ModInt::new(1)));
    }

    #[test]
    fn from_str() {
        ModInt::set_mod(MOD as u32);
        assert_eq!(Ok(ModInt::new(5)), ModInt::from_str("5"));
        assert_eq!(Ok(ModInt::new(1)), ModInt::from_str("1000000008"));
        assert!(ModInt::from_str("5a").is_err());
    }
}
