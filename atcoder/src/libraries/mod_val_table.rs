#[allow(unused_imports)]
use mod_val_table::ModValTable;

#[allow(dead_code)]
mod mod_val_table {
    use super::mod_int::*;

    #[derive(std::fmt::Debug)]
    pub struct ModValTable<M> {
        fact: Vec<M>,
        fact_inv: Vec<M>,
        inv: Vec<M>,
    }

    impl<M: Mod> ModValTable<ModInt<M>> {
        pub fn new(n: usize) -> Self {
            let mut fact = vec![ModInt::<M>::new(1); n + 1];
            let mut fact_inv = vec![ModInt::<M>::new(1); n + 1];
            let mut inv = vec![ModInt::<M>::new(1); n + 1];
            for i in 2..=n {
                fact[i] = fact[i - 1] * i as i64;
                inv[i] = inv[0] / i as i64;
                fact_inv[i] = fact_inv[i - 1] * inv[i];
            }
            Self {
                fact,
                fact_inv,
                inv,
            }
        }

        /// Factorial 階乗 n!
        pub fn factorial(&self, n: i64) -> ModInt<M> {
            self.fact[n as usize]
        }

        /// Permutation 順列 nPr = n! / (n - r)!
        pub fn permutation(&self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                0.into()
            } else {
                self.fact[n as usize] * self.fact_inv[(n - r) as usize]
            }
        }

        /// Combination 組合せ nCr = n! / (n - r)! r! = nPr / r!
        /// Binomial Coefficient
        pub fn combination(&self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                0.into()
            } else {
                self.permutation(n, r) * self.fact_inv[r as usize]
            }
        }

        /// Combinations with Replacement 重複組み合わせ nHr = (n+r)! / k!(n-1)!
        pub fn combinations_with_replacement(&self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                0.into()
            } else {
                self.fact[(n + r) as usize]
                    * self.fact_inv[r as usize]
                    * self.fact_inv[n as usize - 1]
            }
        }
    }
}

////////////////////////////////////////////////////////

use crate::libraries::mod_int::mod_int;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libraries::mod_int::mod_int::Mi;

    type MVT = ModValTable<Mi>;

    #[test]
    fn test() {
        let five = MVT::new(5);

        assert_eq!(1, five.factorial(0).get());
        assert_eq!(1, five.factorial(1).get());
        assert_eq!(2, five.factorial(2).get());
        assert_eq!(6, five.factorial(3).get());
        assert_eq!(24, five.factorial(4).get());
        assert_eq!(120, five.factorial(5).get());

        assert_eq!(1, five.combination(5, 0).get());
        assert_eq!(5, five.combination(5, 1).get());
        assert_eq!(10, five.combination(5, 2).get());
        assert_eq!(10, five.combination(5, 3).get());
        assert_eq!(5, five.combination(5, 4).get());
        assert_eq!(1, five.combination(5, 5).get());

        assert_eq!(1, five.permutation(5, 0).get());
        assert_eq!(5, five.permutation(5, 1).get());
        assert_eq!(20, five.permutation(5, 2).get());
        assert_eq!(60, five.permutation(5, 3).get());
        assert_eq!(120, five.permutation(5, 4).get());
        assert_eq!(120, five.permutation(5, 5).get());
    }

    #[test]
    #[should_panic]
    fn greater_than_number() {
        let fact = MVT::new(10);
        fact.combination(11, 11);
    }
}
