//! 剰余体

////////////////////////////////////////////////////////

#[allow(unused_imports)]
use mod_int::*;

#[allow(dead_code)]
pub mod mod_int {
    use std::marker::PhantomData;
    use std::ops::*;

    pub fn mi(i: i64) -> Mi {
        Mi::new(i)
    }

    pub trait Mod: Copy + Clone + std::fmt::Debug {
        fn get() -> i64;
    }

    pub type Mi = ModInt<Mod1e9p7>;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Mod1e9p7;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Mod1e9p9;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Mod998244353;

    impl Mod for Mod1e9p7 {
        fn get() -> i64 {
            1_000_000_007
        }
    }

    impl Mod for Mod1e9p9 {
        fn get() -> i64 {
            1_000_000_009
        }
    }

    impl Mod for Mod998244353 {
        fn get() -> i64 {
            998_244_353
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModInt<M: Mod> {
        n: i64,
        _p: PhantomData<M>,
    }

    impl<M: Mod> ModInt<M> {
        pub fn new(n: i64) -> Self {
            Self {
                n: n.rem_euclid(M::get()),
                _p: PhantomData,
            }
        }

        pub fn pow(mut self, mut e: i64) -> ModInt<M> {
            let mut result = Self::new(1);
            while e > 0 {
                if e & 1 == 1 {
                    result *= self.n;
                }
                e >>= 1;
                self *= self.n;
            }
            result
        }

        pub fn get(&self) -> i64 {
            self.n
        }
    }

    impl<M: Mod> Add<i64> for ModInt<M> {
        type Output = Self;
        fn add(self, rhs: i64) -> Self {
            ModInt::new(self.n + rhs.rem_euclid(M::get()))
        }
    }

    impl<M: Mod> Add<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            self + rhs.n
        }
    }

    impl<M: Mod> AddAssign<i64> for ModInt<M> {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs
        }
    }

    impl<M: Mod> AddAssign<ModInt<M>> for ModInt<M> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }

    impl<M: Mod> Neg for ModInt<M> {
        type Output = Self;
        fn neg(self) -> Self {
            Self::new(-self.n)
        }
    }

    impl<M: Mod> Sub<i64> for ModInt<M> {
        type Output = Self;
        fn sub(self, rhs: i64) -> Self {
            ModInt::new(self.n - rhs.rem_euclid(M::get()))
        }
    }

    impl<M: Mod> Sub<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            self - rhs.n
        }
    }

    impl<M: Mod> SubAssign<i64> for ModInt<M> {
        fn sub_assign(&mut self, rhs: i64) {
            *self = *self - rhs
        }
    }

    impl<M: Mod> SubAssign<ModInt<M>> for ModInt<M> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }

    impl<M: Mod> Mul<i64> for ModInt<M> {
        type Output = Self;
        fn mul(self, rhs: i64) -> Self {
            ModInt::new(self.n * (rhs % M::get()))
        }
    }

    impl<M: Mod> Mul<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            self * rhs.n
        }
    }

    impl<M: Mod> MulAssign<i64> for ModInt<M> {
        fn mul_assign(&mut self, rhs: i64) {
            *self = *self * rhs
        }
    }

    impl<M: Mod> MulAssign<ModInt<M>> for ModInt<M> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs
        }
    }

    impl<M: Mod> Div<i64> for ModInt<M> {
        type Output = Self;
        fn div(self, rhs: i64) -> Self {
            self * ModInt::new(rhs).pow(M::get() - 2)
        }
    }

    impl<M: Mod> Div<ModInt<M>> for ModInt<M> {
        type Output = Self;
        fn div(self, rhs: Self) -> Self {
            self / rhs.n
        }
    }

    impl<M: Mod> DivAssign<i64> for ModInt<M> {
        fn div_assign(&mut self, rhs: i64) {
            *self = *self / rhs
        }
    }

    impl<M: Mod> DivAssign<ModInt<M>> for ModInt<M> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs
        }
    }

    impl<M: Mod> std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.n)
        }
    }

    impl<M: Mod> std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.n)
        }
    }

    impl<M: Mod> Deref for ModInt<M> {
        type Target = i64;
        fn deref(&self) -> &Self::Target {
            &self.n
        }
    }

    impl<M: Mod> DerefMut for ModInt<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.n
        }
    }

    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(i: i64) -> Self {
            Self::new(i)
        }
    }

    impl<M: Mod> From<ModInt<M>> for i64 {
        fn from(m: ModInt<M>) -> Self {
            m.n
        }
    }
}

////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use super::mod_int::*;
    use rand::distributions::{Distribution, Uniform};

    const MOD: i64 = 1_000_000_007;

    #[test]
    fn neg_test() {
        assert_eq!((Mi::new(0) - 1_000_000).get(), (Mi::new(-1_000_000)).get());
    }

    #[test]
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let mx = Mi::new(x);
            let my = Mi::new(y);

            assert_eq!((mx + my).get(), (x + y) % MOD);
            assert_eq!((mx + y).get(), (x + y) % MOD);
            assert_eq!((mx - my).get(), (x + MOD - y) % MOD);
            assert_eq!((mx - y).get(), (x + MOD - y) % MOD);

            let mut x = x;
            let mut mx = mx;
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

            let mx = Mi::new(x);
            let my = Mi::new(y);

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
        assert_eq!((Mi::new(1_000_000_000) * std::i64::MAX).get(), 961796000);
        assert_eq!((Mi::new(1_000_000_000) + std::i64::MAX).get(), 291171996);
        assert_eq!((Mi::new(1_000_000_000) - std::i64::MAX).get(), 708827997);
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
}
