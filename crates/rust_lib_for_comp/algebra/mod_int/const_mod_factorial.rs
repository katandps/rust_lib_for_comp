//! # ModIntの逆元/階乗の前計算(const版)
//! 逆元と階乗を$O(N)$で前計算する。
//!
//! 少し多めに前計算すること
use super::mod_int::ModInt;

#[codesnip::entry("const-mod-factorial", include("mod-int"))]
pub use mod_val_table_impl::Factorial;
#[codesnip::entry("const-mod-factorial", include("mod-int"))]
pub mod mod_val_table_impl {
    use super::ModInt;
    #[derive(Debug, Clone)]
    pub struct Factorial<const M: u32, const N: usize> {
        /// # inv$\[i] = i$の逆元
        // inv: [M; N],
        /// # fact$\[i] = i!$
        pub fact: [ModInt<M>; N],
        /// # fact_inv$\[i] = i!$の逆元
        pub fact_inv: [ModInt<M>; N],
    }
    impl<const M: u32, const N: usize> Factorial<M, N> {
        /// # 初期化
        /// あるnについてModValTableを初期化する
        pub const fn new() -> Self {
            let mut fact = [ModInt::one(); N];
            let mut fact_inv = [ModInt::one(); N];
            let mut inv = [ModInt::one(); N];
            inv[0] = ModInt::zero();
            let mut i = 2;
            while i < N {
                inv[i] = ModInt::zero().sub(
                    inv[ModInt::<M>::MOD as usize % i]
                        .mul(ModInt::new(ModInt::<M>::MOD / i as u32)),
                );
                i += 1;
            }
            let mut i = 2;
            while i < N {
                fact[i] = fact[i - 1].mul(ModInt::new(i as u32));
                fact_inv[i] = fact_inv[i - 1].mul(inv[i]);
                i += 1;
            }
            Self {
                // inv,
                fact,
                fact_inv,
            }
        }

        /// # Factorial 階乗
        /// $ n! $
        pub const fn factorial(&self, n: i64) -> ModInt<M> {
            self.fact[n as usize]
        }

        /// # Permutation 順列
        /// $nPr = n! / (n - r)!$
        pub const fn permutation(&self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                ModInt::zero()
            } else {
                self.fact[n as usize].mul(self.fact_inv[(n - r) as usize])
            }
        }

        /// # Combination 組合せ
        /// $nCr = n! / (n - r)! r! = nPr / r!$
        /// Binomial Coefficient 二項係数 とも呼ぶ
        pub const fn combination(&self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                ModInt::zero()
            } else {
                self.permutation(n, r).mul(self.fact_inv[r as usize])
            }
        }

        /// # Combinations with Replacement 重複組み合わせ
        /// $nHr = (n+r)! / k!(n-1)!$
        pub const fn combinations_with_replacement(&self, n: i64, r: i64) -> ModInt<M> {
            if n < r || n == 0 {
                ModInt::zero()
            } else {
                let (n, r) = (n as usize, r as usize);
                self.fact[n + r]
                    .mul(self.fact_inv[r])
                    .mul(self.fact_inv[n - 1])
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
        pub const fn catalan_number(&self, n: i64) -> ModInt<M> {
            if n < 0 {
                ModInt::zero()
            } else {
                let n = n as usize;
                self.fact[n * 2]
                    .mul(self.fact_inv[n + 1])
                    .mul(self.fact_inv[n])
            }
        }

        /// # Montmort number モンモール数
        /// 完全順列の個数
        ///
        /// ## 計算量
        /// $O(N)$
        /// N番目まですべて求めても $O(N)$ なので、必要な時は改造する
        pub const fn montmort_number(&self, n: usize) -> ModInt<M> {
            if n <= 1 {
                ModInt::zero()
            } else {
                let mut ret = ModInt::zero();
                let mut k = 2;
                while k <= n {
                    if k & 1 == 0 {
                        ret = ret.add(self.fact_inv[k]);
                    } else {
                        ret = ret.sub(self.fact_inv[k]);
                    }
                    k += 1;
                }
                ret.mul(self.fact[n])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mvt: Factorial<998_244_353, 100> = Factorial::new();
        assert_eq!(ModInt::new(1), mvt.factorial(1));
        assert_eq!(ModInt::new(2), mvt.factorial(2));
        assert_eq!(ModInt::new(6), mvt.factorial(3));
        assert_eq!(ModInt::new(24), mvt.factorial(4));
        assert_eq!(ModInt::new(120), mvt.factorial(5));
    }

    #[test]
    fn catalan_test() {
        let mvt: Factorial<998_244_353, 100> = Factorial::new();
        assert_eq!(ModInt::new(1), mvt.catalan_number(0));
        assert_eq!(ModInt::new(1), mvt.catalan_number(1));
        assert_eq!(ModInt::new(2), mvt.catalan_number(2));
        assert_eq!(ModInt::new(5), mvt.catalan_number(3));
        assert_eq!(ModInt::new(14), mvt.catalan_number(4));
        assert_eq!(ModInt::new(42), mvt.catalan_number(5));
        assert_eq!(ModInt::new(132), mvt.catalan_number(6));
        assert_eq!(ModInt::new(429), mvt.catalan_number(7));
        assert_eq!(ModInt::new(1430), mvt.catalan_number(8));
        assert_eq!(ModInt::new(4862), mvt.catalan_number(9));
    }

    #[test]
    fn montmort_test() {
        let mvt: Factorial<998_244_353, 100> = Factorial::new();
        assert_eq!(ModInt::new(0), mvt.montmort_number(1));
        assert_eq!(ModInt::new(1), mvt.montmort_number(2));
        assert_eq!(ModInt::new(2), mvt.montmort_number(3));
        assert_eq!(ModInt::new(9), mvt.montmort_number(4));
        assert_eq!(ModInt::new(44), mvt.montmort_number(5));
        assert_eq!(ModInt::new(265), mvt.montmort_number(6));
        assert_eq!(ModInt::new(1854), mvt.montmort_number(7));
        assert_eq!(ModInt::new(14833), mvt.montmort_number(8));
        assert_eq!(ModInt::new(133496), mvt.montmort_number(9));
    }
}

#[test]
fn permutation_test() {
    let five: Factorial<998_244_353, 100> = Factorial::new();
    assert_eq!(1, five.permutation(5, 0).reduce());
    assert_eq!(5, five.permutation(5, 1).reduce());
    assert_eq!(20, five.permutation(5, 2).reduce());
    assert_eq!(60, five.permutation(5, 3).reduce());
    assert_eq!(120, five.permutation(5, 4).reduce());
    assert_eq!(120, five.permutation(5, 5).reduce());
    assert_eq!(0, five.permutation(5, 6).reduce());
}

#[test]
fn factorial_test() {
    let five: Factorial<998_244_353, 100> = Factorial::new();
    let res = [1, 1, 2, 6, 24, 120];
    for i in 0..=5 {
        assert_eq!(res[i], five.factorial(i as i64).reduce());
    }
}

#[test]
fn combination_test() {
    let five: Factorial<998_244_353, 100> = Factorial::new();
    assert_eq!(1, five.combination(5, 0).reduce());
    assert_eq!(5, five.combination(5, 1).reduce());
    assert_eq!(10, five.combination(5, 2).reduce());
    assert_eq!(10, five.combination(5, 3).reduce());
    assert_eq!(5, five.combination(5, 4).reduce());
    assert_eq!(1, five.combination(5, 5).reduce());
    assert_eq!(0, five.combination(5, 6).reduce());
}
