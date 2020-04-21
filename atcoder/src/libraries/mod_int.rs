#[allow(unused_imports)]
use mod_int::*;

#[allow(dead_code)]
pub mod mod_int {
    use std::fmt;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;

    const MOD: Num = 1_000_000_007;

    #[derive(Copy, Clone)]
    pub struct ModInt<T: Clone + Copy> {
        pub v: T,
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            let mut t = rhs + self.v;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt::new(t)
        }
    }

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.v
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
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.v < rhs { self.v + MOD } else { self.v };
            ModInt::new(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.v
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
        fn mul(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            ModInt::new((self.v * rhs) % MOD)
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.v
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
        fn div(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            self * ModInt::new(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.v
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
            write!(f, "{}", self.v)
        }
    }

    impl fmt::Debug for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.v)
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: Num) -> ModInt<Num> {
            let mut result = ModInt::new(1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }

        pub fn new(v: Num) -> ModInt<Num> {
            if v >= MOD {
                ModInt { v: v % MOD }
            } else {
                //                ModInt { v } // unstable in rust 1.15.1
                ModInt { v: v }
            }
        }

        pub fn get(&self) -> Num {
            self.v
        }
    }
}

#[cfg(test)]
mod test {
    use super::mod_int::*;
    use rand::distributions::{Distribution, Uniform};

    const MOD: usize = 1_000_000_007;

    #[test]
    fn random_add_sub() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: usize = between
                .sample_iter(&mut rng)
                .take(1)
                .collect::<Vec<usize>>()[0];
            let y: usize = between
                .sample_iter(&mut rng)
                .take(1)
                .collect::<Vec<usize>>()[0];

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

            assert_eq!((mx + my).v, (x + y) % MOD);
            assert_eq!((mx + y).v, (x + y) % MOD);
            assert_eq!((mx - my).v, (x + MOD - y) % MOD);
            assert_eq!((mx - y).v, (x + MOD - y) % MOD);

            let mut x = x;
            let mut mx = mx;
            x += y;
            mx += my;
            assert_eq!(mx.v, x % MOD);

            mx += y;
            x += y;
            assert_eq!(mx.v, x % MOD);

            mx -= my;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.v, x);

            mx -= y;
            x = (x + MOD - y % MOD) % MOD;
            assert_eq!(mx.v, x);
        }
    }

    #[test]
    fn random_mul() {
        let between = Uniform::new_inclusive(0, MOD);
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let x: usize = between
                .sample_iter(&mut rng)
                .take(1)
                .collect::<Vec<usize>>()[0];
            let y: usize = between
                .sample_iter(&mut rng)
                .take(1)
                .collect::<Vec<usize>>()[0];

            let mx = ModInt::new(x);
            let my = ModInt::new(y);

            assert_eq!((mx * my).v, (x * y) % MOD);
            assert_eq!((mx * y).v, (x * y) % MOD);
        }
    }

    #[test]
    fn zero_test() {
        let a = ModInt::new(1_000_000_000);
        let b = ModInt::new(7);
        let c = a + b;
        assert_eq!(c.v, 0);
    }

    #[test]
    fn pow_test() {
        let a = ModInt::new(3);
        let a = a.pow(4);
        assert_eq!(a.v, 81);
    }

    #[test]
    fn div_test() {
        for i in 1..100000 {
            let mut a = ModInt::new(1);
            a /= i;
            a *= i;
            assert_eq!(a.v, 1);
        }
    }

    #[test]
    fn edge_cases() {
        let a = ModInt::new(MOD + 1);
        assert_eq!(a.v, 1);

        let a = ModInt::new(std::usize::MAX) + 1;
        assert_eq!(a.v, 582344008);

        let a = ModInt::new(std::usize::MAX) * 2;
        assert_eq!(a.v, 164688007);

        let a = ModInt::new(1_000_000_000) * std::usize::MAX;
        assert_eq!(a.v, 923591986);

        let mut a = ModInt::new(1_000_000_000);
        a *= std::usize::MAX;
        assert_eq!(a.v, 923591986);

        let a = ModInt::new(1_000_000_000) + std::usize::MAX;
        assert_eq!(a.v, 582344000);

        let mut a = ModInt::new(1_000_000_000);
        a += std::usize::MAX;
        assert_eq!(a.v, 582344000);

        let a = ModInt::new(1_000_000_000) - std::usize::MAX;
        assert_eq!(a.v, 417655993);

        let mut a = ModInt::new(1_000_000_000);
        a -= std::usize::MAX;
        assert_eq!(a.v, 417655993);

        let a = ModInt::new(1_000_000_000) / std::usize::MAX;
        assert_eq!(a.v, 605455209);

        let mut a = ModInt::new(1_000_000_000);
        a /= std::usize::MAX;
        assert_eq!(a.v, 605455209);
    }
}
