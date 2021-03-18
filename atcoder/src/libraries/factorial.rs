#[allow(unused_imports)]
use factorial::Factorial;

#[allow(dead_code)]
mod factorial {
    use super::mod_int::ModInt;

    type Num = i64;

    pub struct Factorial {
        stack: Vec<ModInt<Num>>,
    }

    impl Factorial {
        pub fn new(number: Num) -> Self {
            let mut stack: Vec<ModInt<Num>> = Vec::new();
            stack.push(ModInt::new(1));
            for i in 1..(number + 1) {
                let k = stack[i as usize - 1] * i;
                stack.push(k);
            }
            Factorial { stack }
        }

        /// 階乗 n!
        pub fn fact(&self, number: Num) -> ModInt<Num> {
            self.stack[number as usize]
        }

        /// 順列 nPk
        pub fn npk(&self, n: Num, k: Num) -> ModInt<Num> {
            if k > n {
                ModInt::new(0)
            } else {
                self.stack[n as usize] / self.stack[(n - k) as usize]
            }
        }

        /// 組み合わせ nCk
        pub fn nck(&self, n: Num, k: Num) -> ModInt<Num> {
            if k > n {
                ModInt::new(0)
            } else {
                self.npk(n, k) / self.stack[k as usize]
            }
        }

        ///重複組合せ nHk
        pub fn nhk(&self, n: Num, k: Num) -> ModInt<Num> {
            self.stack[n as usize + k as usize - 1]
                / self.stack[k as usize]
                / self.stack[n as usize - 1]
        }
    }
}

use crate::libraries::mod_int::mod_int;

#[cfg(test)]
mod tests {
    use super::factorial::*;

    #[test]
    fn can_calc() {
        let fact = Factorial::new(5);

        assert_eq!(1, fact.fact(0).get());
        assert_eq!(1, fact.fact(1).get());
        assert_eq!(2, fact.fact(2).get());
        assert_eq!(6, fact.fact(3).get());
        assert_eq!(24, fact.fact(4).get());
        assert_eq!(120, fact.fact(5).get());
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Factorial::new(10);
        fact.fact(11);
    }
}
