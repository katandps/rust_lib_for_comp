#[allow(dead_code)]
fn main() {
    let n = i::u();
    let mut t = i::uv(n);
    let f = Factorial::new(n);
    t.sort();
    let mut before = t[0];
    let mut count = 1;
    let mut ans = ModInt::new(1);
    let mut cur_sum = t[0];
    let mut penalty = t[0];
    for i in 1..n {
        cur_sum += t[i];
        penalty += cur_sum;
        if t[i] != before {
            ans = ans * f.get(count);
            count = 1;
            before = t[i];
        } else {
            count += 1;
        }
    }
    ans = ans * f.get(count);
    println!("{}", penalty);
    println!("{}", ans);
}

#[allow(unused_imports)]
use factorial::Factorial;

#[allow(dead_code)]
mod factorial {
    use super::mod_int::ModInt;

    pub struct Factorial {
        stack: Vec<ModInt<usize>>,
    }

    impl Factorial {
        pub fn new(number: usize) -> Self {
            let mut stack: Vec<ModInt<usize>> = Vec::new();
            stack.push(ModInt::new(1));
            for i in 1..(number + 1) as usize {
                let k = stack[i - 1] * i;
                stack.push(k);
            }

            Factorial { stack: stack }
        }

        pub fn get(&self, number: usize) -> ModInt<usize> {
            self.stack[number]
        }
    }
}

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

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::*;
#[allow(unused_imports)]
use std::num::*;
#[allow(unused_imports)]
use std::str::*;

#[allow(dead_code)]
mod i {
    use super::*;

    pub fn read<T: FromStr>() -> T {
        stdin()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<T>()
            .ok()
            .unwrap()
    }

    pub fn str() -> String {
        read()
    }

    pub fn s() -> Vec<char> {
        str().chars().collect()
    }

    pub fn i() -> i64 {
        read()
    }

    pub fn u() -> usize {
        read()
    }

    pub fn f() -> f64 {
        read()
    }

    pub fn c() -> char {
        read::<String>().pop().unwrap()
    }

    pub fn iv(n: usize) -> Vec<i64> {
        (0..n).map(|_| i()).collect()
    }

    pub fn iv2(n: usize) -> Vec<(i64, i64)> {
        (0..n).map(|_| iv(2)).map(|a| (a[0], a[1])).collect()
    }

    pub fn uv(n: usize) -> Vec<usize> {
        (0..n).map(|_| u()).collect()
    }

    pub fn uv2(n: usize) -> Vec<(usize, usize)> {
        (0..n).map(|_| uv(2)).map(|a| (a[0], a[1])).collect()
    }

    pub fn fv(n: usize) -> Vec<f64> {
        (0..n).map(|_| f()).collect()
    }

    pub fn cmap(h: usize) -> Vec<Vec<char>> {
        (0..h).map(|_| s()).collect()
    }
}
