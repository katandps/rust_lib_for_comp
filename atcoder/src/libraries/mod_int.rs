#[allow(unused_imports)]
use mod_int::*;

#[allow(dead_code)]
pub mod mod_int {
    use std::fmt;
    use std::ops::*;

    type Num = i64;

    const MOD: Num = 1_000_000_007;

    #[derive(Copy, Clone)]
    pub struct ModInt<T: Clone + Copy>(T);

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: Num) -> ModInt<Num> {
            ModInt::new(self.get() + rhs.rem_euclid(MOD))
        }
    }
    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.get()
        }
    }
    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other
        }
    }
    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            ModInt::new(self.get() - rhs.rem_euclid(MOD))
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.get()
        }
    }

    impl Neg for ModInt<Num> {
        type Output = ModInt<Num>;
        fn neg(self) -> ModInt<Num> {
            Self::new(-self.get())
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other
        }
    }
    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: Num) -> ModInt<Num> {
            ModInt::new(self.get() * (rhs % MOD))
        }
    }
    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.get()
        }
    }
    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs
        }
    }
    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs
        }
    }
    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: Num) -> ModInt<Num> {
            self * ModInt::new(rhs).pow(MOD - 2)
        }
    }
    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.get()
        }
    }
    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }
    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl fmt::Display for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.get())
        }
    }

    impl fmt::Debug for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.get())
        }
    }

    impl Deref for ModInt<Num> {
        type Target = i64;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for ModInt<Num> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl ModInt<Num> {
        pub fn pow(mut self, mut e: Num) -> ModInt<Num> {
            let mut result = Self::new(1);
            while e > 0 {
                if e & 1 == 1 {
                    result *= self.0;
                }
                e >>= 1;
                self *= self.0;
            }
            result
        }
        pub fn new(v: Num) -> ModInt<Num> {
            ModInt(v.rem_euclid(MOD))
        }
        pub fn get(&self) -> Num {
            self.0
        }
    }

    impl From<Num> for ModInt<Num> {
        fn from(i: i64) -> Self {
            Self::new(i)
        }
    }

    impl From<ModInt<Num>> for Num {
        fn from(m: ModInt<i64>) -> Self {
            m.0
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_int::*;
    use rand::distributions::{Distribution, Uniform};

    const MOD: i64 = 1_000_000_007;

    #[test]
    fn neg_test() {
        assert_eq!(
            (ModInt::new(0) - 1_000_000).get(),
            (ModInt::new(-1_000_000)).get()
        );
    }

    #[test]
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];
            let y: i64 = between.sample_iter(&mut rng).take(1).collect::<Vec<i64>>()[0];

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

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

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

            assert_eq!((mx * my).get(), (x * y) % MOD);
            assert_eq!((mx * y).get(), (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;
        assert_eq!(c.get(), 0);
    }

    #[test]
    fn pow_test() {
        let a = ModInt::new(3);
        let a = a.pow(4);
        assert_eq!(a.get(), 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = ModInt::new(1);
            a /= i;
            a *= i;
            assert_eq!(a.get(), 1);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!((ModInt::new(MOD + 1)).get(), 1);
        assert_eq!((ModInt::new(std::i64::MAX) + 1).get(), 291172004);
        assert_eq!(
            (ModInt::new(1_000_000_000) * std::i64::MAX).get(),
            961796000
        );
        assert_eq!(
            (ModInt::new(1_000_000_000) + std::i64::MAX).get(),
            291171996
        );
        assert_eq!(
            (ModInt::new(1_000_000_000) - std::i64::MAX).get(),
            708827997
        );
        assert_eq!(
            (ModInt::new(1_000_000_000) / std::i64::MAX * std::i64::MAX).get(),
            1_000_000_000
        );

        let mut a = ModInt::new(1_000_000_000);
        a *= std::i64::MAX;
        assert_eq!(a.get(), 961796000);

        let mut a = ModInt::new(1_000_000_000);
        a += std::i64::MAX;
        assert_eq!(a.get(), 291171996);

        let mut a = ModInt::new(1_000_000_000);
        a -= std::i64::MAX;
        assert_eq!(a.get(), 708827997);

        let mut a = ModInt::new(1_000_000_000);
        a /= std::i64::MAX;
        assert_eq!((a * std::i64::MAX).get(), 1_000_000_000);
    }
}
