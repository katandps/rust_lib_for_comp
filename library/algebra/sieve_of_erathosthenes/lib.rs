//! # エラトステネスの篩
//! 前計算を施し、高速に素数判定及び素因数分解を行う
//!
//! ## 計算量
//! 前計算
//! $O(N\log \log N)$

use prelude::*;

#[snippet(name = "sieve-of-eratosthenes", doc_hidden)]
pub struct SieveOfEratosthenes {
    m: usize,
    // i番目を割り切る最小の素因数
    table: Vec<usize>,
}
#[snippet(name = "sieve-of-eratosthenes", doc_hidden)]
impl SieveOfEratosthenes {
    /// # 前計算
    ///
    /// ## 計算量
    /// 自然数列の長さをNとして、素数の逆数和 $O(\log \log N)$がかかり、$O(N\log\log N)$
    pub fn new(m: usize) -> Self {
        let mut table = vec![0; m + 1];
        table.iter_mut().enumerate().skip(2).for_each(|(i, x)| {
            if i % 2 == 0 {
                *x = 2;
            } else if i % 3 == 0 {
                *x = 3;
            }
        });
        if m <= 2 {
            return Self { m, table };
        }
        // 6で割ったあまりが5か1のときにのみ素数の可能性がある
        let mut i = 5;
        let mut f = 4;
        while i <= m {
            if table[i] == 0 {
                table[i] = i;
                for j in i..m / i + 1 {
                    if table[i * j] == 0 {
                        table[i * j] = i;
                    }
                }
            }
            f = 6 - f;
            i += f;
        }
        Self { m, table }
    }

    /// # 素数列を得る
    ///
    /// ## 計算量
    /// $O(N)$
    pub fn primes(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for i in 2..=self.m {
            if self.table[i] == i {
                ret.push(i);
            }
        }
        ret
    }

    /// # 素数判定
    ///
    /// ## 計算量
    /// $O(1)$
    pub fn is_prime(&self, n: usize) -> bool {
        self.table[n] == n
    }

    /// # 素因数分解
    /// 得られる素数列は昇順ソート済み
    /// ## 計算量
    /// $O(logN)$
    pub fn prime_factorize(&self, mut n: usize) -> Vec<usize> {
        let mut ret = Vec::new();
        while self.table[n] > 1 {
            ret.push(self.table[n]);
            n /= self.table[n];
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let sieve = SieveOfEratosthenes::new(200);
        let p = sieve.primes();
        assert_eq!(p.len(), 46);
        assert_eq!(p[0], 2);
        assert_eq!(p[1], 3);
        assert_eq!(p[2], 5);
        assert_eq!(p[3], 7);
        assert_eq!(p[4], 11);
        assert_eq!(p[5], 13);
        assert_eq!(p[6], 17);
        assert_eq!(p[7], 19);
        assert_eq!(
            vec![2, 2, 3, 3, 5],
            sieve.prime_factorize(2 * 2 * 3 * 3 * 5)
        )
    }
}
