//! # ModIntの逆元/階乗の前計算
//! 逆元と階乗を$O(N)$で前計算する。
//!
use super::mod_int::ModInt;
use crate::algebra::*;

#[codesnip::entry("mod-factorial", include("mod-int"))]
pub use mod_factorial_impl::Factorial;
#[codesnip::entry("mod-factorial", include("mod-int"))]
pub mod mod_factorial_impl {
    use super::{ModInt, One, Zero};
    #[derive(Debug, Clone)]
    pub struct Factorial<M> {
        /// # inv$\[i] = i$の逆元
        inv: Vec<M>,
        /// # fact$\[i] = i!$
        fact: Vec<M>,
        /// # fact_inv$\[i] = i!$の逆元
        fact_inv: Vec<M>,
        /// 初期化済みのindex
        limit: usize,
    }
    impl<M: Zero + One> Default for Factorial<M> {
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
    impl<const M: u32> Factorial<ModInt<M>> {
        /// # 初期化
        /// あるnについてModValTableを初期化する
        pub fn new(n: usize) -> Self {
            let mut fact = vec![ModInt::one(); n + 1];
            let mut fact_inv = vec![ModInt::one(); n + 1];
            let mut inv = vec![ModInt::one(); n + 1];
            inv[0] = ModInt::zero();
            for i in 2..=n {
                inv[i] = ModInt::zero()
                    - inv[ModInt::<M>::MOD as usize % i] * (ModInt::<M>::MOD / i as u32);
            }
            for i in 2..=n {
                fact[i] = fact[i - 1] * i;
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
            self.inv.resize(n + 1, ModInt::one());
            self.fact.resize(n + 1, ModInt::one());
            self.fact_inv.resize(n + 1, ModInt::one());
            for i in self.limit + 1..=n {
                self.inv[i] = ModInt::zero()
                    - self.inv[ModInt::<M>::MOD as usize % i] * (ModInt::<M>::MOD / i as u32);
            }
            for i in self.limit + 1..=n {
                self.fact[i] = self.fact[i - 1] * i;
                self.fact_inv[i] = self.fact_inv[i - 1] * self.inv[i];
            }
        }

        /// # Factorial 階乗
        /// $ n! $
        pub fn factorial(&mut self, n: i64) -> ModInt<M> {
            self.init(n as usize);
            self.fact[n as usize]
        }

        /// # Permutation 順列
        /// $nPr = n! / (n - r)!$
        pub fn permutation(&mut self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                ModInt::zero()
            } else {
                self.init(n as usize);
                self.fact[n as usize] * self.fact_inv[(n - r) as usize]
            }
        }

        /// # Combination 組合せ
        /// $nCr = n! / (n - r)! r! = nPr / r!$
        /// Binomial Coefficient 二項係数 とも呼ぶ
        pub fn combination(&mut self, n: i64, r: i64) -> ModInt<M> {
            if n < r {
                ModInt::zero()
            } else {
                self.init(n as usize);
                self.permutation(n, r) * self.fact_inv[r as usize]
            }
        }

        /// # Combinations with Replacement 重複組み合わせ
        /// $nHr = (n+r)! / k!(n-1)!$
        pub fn combinations_with_replacement(&mut self, n: i64, r: i64) -> ModInt<M> {
            if n < r || n == 0 {
                ModInt::zero()
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
        pub fn catalan_number(&mut self, n: i64) -> ModInt<M> {
            if n < 0 {
                ModInt::zero()
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
        pub fn montmort_number(&mut self, n: usize) -> ModInt<M> {
            if n <= 1 {
                ModInt::zero()
            } else {
                self.init(n);
                let mut ret = ModInt::zero();
                for k in 2..=n {
                    ret += self.fact_inv[k] * if k & 1 == 0 { 1 } else { -1 };
                }
                ret * self.fact[n]
            }
        }
    }
}

#[test]
fn catalan_test() {
    let mut mvt: Factorial<ModInt<998_244_353>> = Factorial::default();
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
    let mut mvt: Factorial<ModInt<998_244_353>> = Factorial::default();
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

#[test]
fn permutation_test() {
    let mut five = Factorial::<ModInt>::new(5);
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
    let mut five = Factorial::<ModInt>::new(5);
    let res = [1, 1, 2, 6, 24, 120];
    for (i, &ri) in res.iter().enumerate() {
        assert_eq!(ri, five.factorial(i as i64).reduce());
    }
}

#[test]
fn combination_test() {
    let mut five = Factorial::<ModInt>::new(5);
    assert_eq!(1, five.combination(5, 0).reduce());
    assert_eq!(5, five.combination(5, 1).reduce());
    assert_eq!(10, five.combination(5, 2).reduce());
    assert_eq!(10, five.combination(5, 3).reduce());
    assert_eq!(5, five.combination(5, 4).reduce());
    assert_eq!(1, five.combination(5, 5).reduce());
    assert_eq!(0, five.combination(5, 6).reduce());
}
