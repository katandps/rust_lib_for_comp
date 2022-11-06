//! # ModIntの逆元/階乗の前計算
//! 逆元と階乗を$O(N)$で前計算する。
//!
use crate::algebra::mod_int::Mod;
use crate::prelude::*;

#[snippet(name = "mod-val-table", doc_hidden)]
#[derive(Debug, Clone)]
pub struct ModValTable<M> {
    /// # inv$\[i] = i$の逆元
    pub inv: Vec<M>,
    /// # fact$\[i] = i!$
    pub fact: Vec<M>,
    /// # fact_inv$\[i] = i!$の逆元
    pub fact_inv: Vec<M>,
    /// 初期化済みのindex
    limit: usize,
}

#[snippet(name = "mod-val-table", doc_hidden)]
pub mod mod_val_table_impl {
    use super::{AddAssign, Mod, ModValTable, Mul, Neg, One, Sub, Zero};
    impl<M: Zero + One> Default for ModValTable<M> {
        fn default() -> Self {
            let inv = vec![M::zero(), M::one()];
            let fact = vec![M::one(), M::one()];
            let fact_inv = vec![M::one(), M::one()];

            Self {
                inv,
                fact,
                fact_inv,
                limit: 1,
            }
        }
    }
    pub trait ModInt:
        Zero
        + One
        + Mod
        + Mul<i64, Output = Self>
        + Mul<Output = Self>
        + Sub<Output = Self>
        + Neg<Output = Self>
        + AddAssign
    {
    }
    impl<
            M: Zero
                + One
                + Mod
                + Mul<i64, Output = Self>
                + Mul<Output = Self>
                + Sub<Output = Self>
                + Neg<Output = Self>
                + AddAssign,
        > ModInt for M
    {
    }
    impl<M: ModInt> ModValTable<M> {
        /// # 初期化
        /// あるnについてModValTableを初期化する
        pub fn new(n: usize) -> Self {
            let mut fact = vec![M::one(); n + 1];
            let mut fact_inv = vec![M::one(); n + 1];
            let mut inv = vec![M::one(); n + 1];
            inv[0] = M::zero();
            for i in 2..=n {
                inv[i] = M::zero() - inv[M::MOD as usize % i] * (M::MOD / i as u32) as i64;
            }
            for i in 2..=n {
                fact[i] = fact[i - 1] * i as i64;
                fact_inv[i] = fact_inv[i - 1] * inv[i];
            }
            Self {
                inv,
                fact,
                fact_inv,
                limit: 1,
            }
        }

        fn init(&mut self, n: usize) {
            if n <= self.limit {
                return;
            }
            self.inv.resize(n + 1, M::one());
            self.fact.resize(n + 1, M::one());
            self.fact_inv.resize(n + 1, M::one());
            for i in self.limit + 1..=n {
                self.inv[i] =
                    M::zero() - self.inv[M::MOD as usize % i] * (M::MOD / i as u32) as i64;
            }
            for i in self.limit + 1..=n {
                self.fact[i] = self.fact[i - 1] * i as i64;
                self.fact_inv[i] = self.fact_inv[i - 1] * self.inv[i];
            }
        }

        /// # Factorial 階乗
        /// $ n! $
        /// ```
        /// # use rust_lib_for_comp::algebra::mod_int::Mi;
        /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
        /// let mut five = ModValTable::<Mi>::new(5);
        /// let res = vec![1, 1, 2, 6, 24, 120];
        /// for i in 0..=5 {
        ///     assert_eq!(res[i], five.factorial(i as i64).reduce());
        /// }
        /// ```

        pub fn factorial(&mut self, n: i64) -> M {
            self.init(n as usize);
            self.fact[n as usize]
        }

        /// # Permutation 順列
        /// $nPr = n! / (n - r)!$
        /// ```
        /// # use rust_lib_for_comp::algebra::mod_int::Mi;
        /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
        /// let mut five = ModValTable::<Mi>::new(5);
        /// assert_eq!(1, five.permutation(5, 0).reduce());
        /// assert_eq!(5, five.permutation(5, 1).reduce());
        /// assert_eq!(20, five.permutation(5, 2).reduce());
        /// assert_eq!(60, five.permutation(5, 3).reduce());
        /// assert_eq!(120, five.permutation(5, 4).reduce());
        /// assert_eq!(120, five.permutation(5, 5).reduce());
        /// ```
        pub fn permutation(&mut self, n: i64, r: i64) -> M {
            if n < r {
                M::zero()
            } else {
                self.init(n as usize);
                self.fact[n as usize] * self.fact_inv[(n - r) as usize]
            }
        }

        /// # Combination 組合せ
        /// $nCr = n! / (n - r)! r! = nPr / r!$
        /// Binomial Coefficient 二項係数 とも呼ぶ
        /// ```
        /// use rust_lib_for_comp::algebra::mod_int::Mi;
        /// use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
        /// let mut five = ModValTable::<Mi>::new(5);
        /// assert_eq!(1, five.combination(5, 0).reduce());
        /// assert_eq!(5, five.combination(5, 1).reduce());
        /// assert_eq!(10, five.combination(5, 2).reduce());
        /// assert_eq!(10, five.combination(5, 3).reduce());
        /// assert_eq!(5, five.combination(5, 4).reduce());
        /// assert_eq!(1, five.combination(5, 5).reduce());
        /// ```
        pub fn combination(&mut self, n: i64, r: i64) -> M {
            if n < r {
                M::zero()
            } else {
                self.init(n as usize);
                self.permutation(n, r) * self.fact_inv[r as usize]
            }
        }

        /// # Combinations with Replacement 重複組み合わせ
        /// $nHr = (n+r)! / k!(n-1)!$
        pub fn combinations_with_replacement(&mut self, n: i64, r: i64) -> M {
            if n < r || n == 0 {
                M::zero()
            } else {
                let (n, r) = (n as usize, r as usize);
                self.init(n + r);
                self.fact[n + r] * self.fact_inv[r] * self.fact_inv[n - 1]
            }
        }

        /// # Catalan number カタラン数
        /// N番目のカタラン数を求める
        ///
        /// ## 特徴
        /// - validなかっこ列の個数
        /// - 二分木の個数
        /// - n * n の格子を対角線をまたがずに(0,0)から(n,n)に向かう道順の総数
        /// - N+2角形の三角形分割
        /// - 平面グラフの交差(2n人が円になって手を交差させないように握手する場合の数)
        ///
        pub fn catalan_number(&mut self, n: i64) -> M {
            if n < 0 {
                M::zero()
            } else {
                let n = n as usize;
                self.init(n * 2 + 1);
                self.fact[n * 2] * self.fact_inv[n + 1] * self.fact_inv[n]
            }
        }

        /// # Montmort number モンモール数
        /// 完全順列の個数
        ///
        /// ## 計算量
        /// $O(N)$
        /// N番目まですべて求めても $O(N)$ なので、必要な時は改造する
        pub fn montmort_number(&mut self, n: usize) -> M {
            if n <= 1 {
                M::zero()
            } else {
                self.init(n as usize);
                let mut ret = M::zero();
                for k in 2..=n {
                    ret += if k & 1 == 0 {
                        self.fact_inv[k]
                    } else {
                        -self.fact_inv[k]
                    };
                }
                ret * self.fact[n]
            }
        }
    }
}

#[test]
fn catalan_test() {
    use crate::algebra::mod_int::mod998244353::Mi;
    let mut mvt = ModValTable::default();
    assert_eq!(Mi::new(1), mvt.catalan_number(0));
    assert_eq!(Mi::new(1), mvt.catalan_number(1));
    assert_eq!(Mi::new(2), mvt.catalan_number(2));
    assert_eq!(Mi::new(5), mvt.catalan_number(3));
    assert_eq!(Mi::new(14), mvt.catalan_number(4));
    assert_eq!(Mi::new(42), mvt.catalan_number(5));
    assert_eq!(Mi::new(132), mvt.catalan_number(6));
    assert_eq!(Mi::new(429), mvt.catalan_number(7));
    assert_eq!(Mi::new(1430), mvt.catalan_number(8));
    assert_eq!(Mi::new(4862), mvt.catalan_number(9));
}

#[test]
fn montmort_test() {
    use crate::algebra::mod_int::mod998244353::Mi;

    let mut mvt = ModValTable::default();
    assert_eq!(Mi::new(0), mvt.montmort_number(1));
    assert_eq!(Mi::new(1), mvt.montmort_number(2));
    assert_eq!(Mi::new(2), mvt.montmort_number(3));
    assert_eq!(Mi::new(9), mvt.montmort_number(4));
    assert_eq!(Mi::new(44), mvt.montmort_number(5));
    assert_eq!(Mi::new(265), mvt.montmort_number(6));
    assert_eq!(Mi::new(1854), mvt.montmort_number(7));
    assert_eq!(Mi::new(14833), mvt.montmort_number(8));
    assert_eq!(Mi::new(133496), mvt.montmort_number(9));
}
