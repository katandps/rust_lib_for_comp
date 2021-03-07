#[allow(unused_imports)]
use combination::Combination;

#[allow(dead_code)]
mod combination {
    use super::mod_int::*;
    type Num = i64;

    /// Binomial Coefficient mod by mod_int::MOD
    pub struct Combination {
        n: Vec<ModInt<Num>>,
    }

    impl Combination {
        pub fn new(n: Num) -> Self {
            let mut stack = vec![ModInt::new(1)];
            for i in 0..n {
                let t = stack[i as usize] * (n - i);
                stack.push(t / (i + 1));
            }
            Combination { n: stack }
        }

        pub fn n_c_r(&self, r: Num) -> ModInt<Num> {
            self.n[r as usize]
        }
    }
}

use crate::libraries::mod_int::mod_int;
#[cfg(test)]
mod tests {
    use super::combination::*;

    #[test]
    fn can_calc() {
        let five = Combination::new(5);

        assert_eq!(1, five.n_c_r(0).get());
        assert_eq!(5, five.n_c_r(1).get());
        assert_eq!(10, five.n_c_r(2).get());
        assert_eq!(10, five.n_c_r(3).get());
        assert_eq!(5, five.n_c_r(4).get());
        assert_eq!(1, five.n_c_r(5).get());
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = Combination::new(10);
        fact.n_c_r(11);
    }
}
