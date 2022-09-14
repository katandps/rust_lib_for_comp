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
//! [ポラードのロー素因数分解法](https://algo-method.com/submissions/600248)

use crate::{
    algebra::{binary_operation::greatest_common_divisor::Gcd, miller_rabin::MillerRabin},
    prelude::*,
};

#[snippet(name = "pollard-rho", doc_hidden)]
pub trait PollardRho {
    fn prime_factorize(&self) -> Vec<u64>;
}

#[snippet(name = "pollard-rho", doc_hidden)]
impl PollardRho for u64 {
    fn prime_factorize(&self) -> Vec<u64> {
        if self <= &1 {
            return Vec::new();
        }
        // 素因数を一つ得る
        fn pollard(n: u64) -> u64 {
            if n % 2 == 0 {
                return 2;
            }
            if n.is_prime() {
                return n;
            }
            let sqplus1 = |x: u64| -> u64 { ((x as i128 * x as i128 + 1) % n as i128) as u64 };
            let mut step = 0;
            loop {
                step += 1;
                let (mut x, mut y) = (step, sqplus1(step));
                loop {
                    let p = Gcd::op(&((y as i128 - x as i128 + n as i128) as u64), &n);
                    if p == 0 || p == n {
                        break;
                    }
                    if p != 1 {
                        return p;
                    }
                    x = sqplus1(x);
                    y = sqplus1(sqplus1(y));
                }
            }
        }

        let p = pollard(*self);
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
