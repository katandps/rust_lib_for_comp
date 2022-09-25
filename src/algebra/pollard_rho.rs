//! # ポラード・ロー法
//! 素因数分解を行う
//!
//! ## 計算量
//! $O(N^{\frac{1}{4}})$
//!
//! ## dependency
//! [mod_pow](crate::algebra::mod_pow)
//! [gcd](crate::algebra::binary_operation::greatest_common_divisor)
//! [miller_rabin](crate::algebra::miller_rabin)
//!
//! ## verify
//! [ポラードのロー素因数分解法](https://algo-method.com/submissions/611854)

use crate::{
    algebra::{
        binary_operation::greatest_common_divisor::Gcd, miller_rabin::MillerRabin,
        montgomery_multiplication::MontgomeryU64,
    },
    prelude::*,
};

#[snippet(name = "pollard-rho", doc_hidden)]
pub trait PollardRho {
    fn prime_factorize(&self) -> Vec<u64>;
}

#[snippet(name = "pollard-rho", doc_hidden)]
#[allow(clippy::many_single_char_names)]
impl PollardRho for u64 {
    fn prime_factorize(&self) -> Vec<u64> {
        if self <= &1 {
            return Vec::new();
        }
        fn find_cycle_by_brent(n: u64) -> u64 {
            if n % 2 == 0 {
                return 2;
            }
            if n.is_prime() {
                return n;
            }
            let mul = MontgomeryU64::new(n);
            const LIMIT: u64 = 256;
            for epoch in 1..LIMIT {
                let prng_next = |x| mul.add(mul.mul(x, x), epoch);
                let m = 1 << ((0u64.leading_zeros() - n.leading_zeros()) >> 3);
                let (mut y, mut r, mut q, mut g) = (2, 1, 1, 1);
                let (mut x, mut ys) = (0, 0);
                while g == 1 {
                    x = y;
                    for _ in 0..r {
                        y = prng_next(y);
                    }
                    let mut k = 0;
                    while k < r && g == 1 {
                        ys = y;
                        for _ in 0..min(m, r - k) {
                            y = prng_next(y);
                            q = mul.mul(q, max(x, y) - min(x, y));
                        }
                        g = Gcd::op(&q, &n);
                        k += m;
                    }
                    r <<= 1;
                }
                if g == n {
                    g = 1;
                    while g == 1 {
                        ys = prng_next(ys);
                        g = Gcd::op(&(max(x, ys) - min(x, ys)), &n);
                    }
                }
                if g < n {
                    return g;
                }
            }
            panic!("not found cycle.")
        }
        let p = find_cycle_by_brent(*self);
        if &p == self {
            return vec![p];
        }
        let mut ret = p.prime_factorize();
        ret.append(&mut (*self / p).prime_factorize());
        ret.sort_unstable();
        ret
    }
}
#[test]
fn test() {
    assert_eq!(
        vec![3, 3, 53, 79, 265371653],
        9999999999999.prime_factorize()
    );
    assert!(10023859281455311421.prime_factorize().len() == 2);
}