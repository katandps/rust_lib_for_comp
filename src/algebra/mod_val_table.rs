//! # 階乗の前計算
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
    /// # 初期化
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
        inv[0] = ModInt::zero();
        for i in 2..=n {
            inv[i] = ModInt::zero() - inv[M::MOD as usize % i] * (M::MOD / i as u32) as i64;
        }
        for i in 2..=n {
            fact[i] = fact[i - 1] * i as i64;
            fact_inv[i] = fact_inv[i - 1] * inv[i];
        }
        Self { fact, fact_inv }
    }

    /// # Factorial 階乗
    /// $ n! $
    /// ```
    /// # use rust_lib_for_comp::algebra::mod_int::Mi;
    /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let five = ModValTable::<Mi>::new(5);
    /// let res = vec![1, 1, 2, 6, 24, 120];
    /// for i in 0..=5 {
    ///     assert_eq!(res[i], five.factorial(i as i64).reduce());
    /// }
    /// ```

    pub fn factorial(&self, n: i64) -> ModInt<M> {
        self.fact[n as usize]
    }

    /// # Permutation 順列
    /// $nPr = n! / (n - r)!$
    /// ```
    /// # use rust_lib_for_comp::algebra::mod_int::Mi;
    /// # use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let five = ModValTable::<Mi>::new(5);
    /// assert_eq!(1, five.permutation(5, 0).reduce());
    /// assert_eq!(5, five.permutation(5, 1).reduce());
    /// assert_eq!(20, five.permutation(5, 2).reduce());
    /// assert_eq!(60, five.permutation(5, 3).reduce());
    /// assert_eq!(120, five.permutation(5, 4).reduce());
    /// assert_eq!(120, five.permutation(5, 5).reduce());
    /// ```
    pub fn permutation(&self, n: i64, r: i64) -> ModInt<M> {
        if n < r {
            0.into()
        } else {
            self.fact[n as usize] * self.fact_inv[(n - r) as usize]
        }
    }

    /// # Combination 組合せ
    /// $nCr = n! / (n - r)! r! = nPr / r!$
    /// Binomial Coefficient 二項係数 とも呼ぶ
    /// ```
    /// use rust_lib_for_comp::algebra::mod_int::Mi;
    /// use rust_lib_for_comp::algebra::mod_val_table::ModValTable;
    /// let five = ModValTable::<Mi>::new(5);
    /// assert_eq!(1, five.combination(5, 0).reduce());
    /// assert_eq!(5, five.combination(5, 1).reduce());
    /// assert_eq!(10, five.combination(5, 2).reduce());
    /// assert_eq!(10, five.combination(5, 3).reduce());
    /// assert_eq!(5, five.combination(5, 4).reduce());
    /// assert_eq!(1, five.combination(5, 5).reduce());
    /// ```
    pub fn combination(&self, n: i64, r: i64) -> ModInt<M> {
        if n < r {
            0.into()
        } else {
            self.permutation(n, r) * self.fact_inv[r as usize]
        }
    }

    /// # Combinations with Replacement 重複組み合わせ
    /// $nHr = (n+r)! / k!(n-1)!$
    pub fn combinations_with_replacement(&self, n: i64, r: i64) -> ModInt<M> {
        if n < r {
            0.into()
        } else {
            self.fact[(n + r) as usize] * self.fact_inv[r as usize] * self.fact_inv[n as usize - 1]
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
    pub fn catalan_number(&self, n: i64) -> ModInt<M> {
        if n < 0 {
            0.into()
        } else {
            assert!(n as usize * 2 < self.fact.len());
            self.fact[n as usize * 2] * self.fact_inv[n as usize + 1] * self.fact_inv[n as usize]
        }
    }

    /// # Montmort number モンモール数
    /// 完全順列の個数
    ///
    /// ## 計算量
    /// $O(N)$
    /// N番目まですべて求めても $O(N)$ なので、必要な時は改造する
    pub fn montmort_number(&self, n: usize) -> ModInt<M> {
        if n <= 1 {
            ModInt::zero()
        } else {
            let mut ret = ModInt::zero();
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

#[test]
fn catalan_test() {
    use crate::algebra::mod_int::mod998244353::Mi;
    let mvt = ModValTable::<Mi>::new(20);
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

    let mvt = ModValTable::<Mi>::new(10);
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
