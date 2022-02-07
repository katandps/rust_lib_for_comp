//! ModInt上での階乗や組み合わせの数え上げ
//! 剰余類Modについて、組み合わせや順列を数え上げる
use crate::algebra::mod_int::{Mod, ModInt};
use crate::prelude::*;

#[snippet(name = "mod-val-table", doc_hidden)]
#[derive(Debug)]
pub struct ModValTable<M> {
    fact: Vec<M>,
    fact_inv: Vec<M>,
}

#[snippet(name = "mod-val-table", doc_hidden)]
impl<M: Mod> ModValTable<ModInt<M>> {
    ///
    /// あるnについてModValTableを初期化する
    ///
    /// nを超える値を呼び出したとき、panicする
    /// ```rust, should_panic
    /// # use rust_lib_for_comp::algebra::mod_int::Mi;
    /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let fact = ModValTable::<Mi>::new(10);
    /// fact.combination(11, 11);
    /// ```
    pub fn new(n: usize) -> Self {
        let mut fact = vec![ModInt::<M>::new(1); n + 1];
        let mut fact_inv = vec![ModInt::<M>::new(1); n + 1];
        let mut inv = vec![ModInt::<M>::new(1); n + 1];
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as i64;
            inv[i] = inv[0] / i as i64;
            fact_inv[i] = fact_inv[i - 1] * inv[i];
        }
        Self { fact, fact_inv }
    }

    /// Factorial 階乗 n!
    /// ```
    /// # use rust_lib_for_comp::algebra::mod_int::Mi;
    /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let five = ModValTable::<Mi>::new(5);
    /// let res = vec![1, 1, 2, 6, 24, 120];
    /// for i in 0..=5 {
    ///     assert_eq!(res[i], five.factorial(i as i64).get());
    /// }
    /// ```

    pub fn factorial(&self, n: i64) -> ModInt<M> {
        self.fact[n as usize]
    }

    /// Permutation 順列 nPr = n! / (n - r)!
    /// ```
    /// # use rust_lib_for_comp::algebra::mod_int::Mi;
    /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let five = ModValTable::<Mi>::new(5);
    /// assert_eq!(1, five.permutation(5, 0).get());
    /// assert_eq!(5, five.permutation(5, 1).get());
    /// assert_eq!(20, five.permutation(5, 2).get());
    /// assert_eq!(60, five.permutation(5, 3).get());
    /// assert_eq!(120, five.permutation(5, 4).get());
    /// assert_eq!(120, five.permutation(5, 5).get());
    /// ```
    pub fn permutation(&self, n: i64, r: i64) -> ModInt<M> {
        if n < r {
            0.into()
        } else {
            self.fact[n as usize] * self.fact_inv[(n - r) as usize]
        }
    }

    /// Combination 組合せ nCr = n! / (n - r)! r! = nPr / r!
    /// Binomial Coefficient
    /// ```
    /// use rust_lib_for_comp::algebra::mod_int::Mi;
    /// use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let five = ModValTable::<Mi>::new(5);
    /// assert_eq!(1, five.combination(5, 0).get());
    /// assert_eq!(5, five.combination(5, 1).get());
    /// assert_eq!(10, five.combination(5, 2).get());
    /// assert_eq!(10, five.combination(5, 3).get());
    /// assert_eq!(5, five.combination(5, 4).get());
    /// assert_eq!(1, five.combination(5, 5).get());
    /// ```
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
            self.fact[(n + r) as usize] * self.fact_inv[r as usize] * self.fact_inv[n as usize - 1]
        }
    }
}
