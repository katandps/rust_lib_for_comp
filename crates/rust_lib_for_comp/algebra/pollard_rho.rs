//! # ポラード・ロー法
//! 素因数分解を行う
//!
//! ## 計算量
//! $O(N^{\frac{1}{4}})$
//!
use crate::algebra::binary_operation::greatest_common_divisor::Gcd;
use crate::algebra::*;
use miller_rabin::MillerRabin;
use montgomery_multiplication_64::MontgomeryReduction;

#[codesnip::entry("pollard-rho")]
pub trait PollardRho {
    fn prime_factorize(&self) -> Vec<u64>;
}

#[codesnip::entry(
    "pollard-rho",
    include("algebra, gcd, miller-rabin, montgomery-multiplication-64")
)]
#[allow(clippy::many_single_char_names)]
impl PollardRho for u64 {
    fn prime_factorize(&self) -> Vec<u64> {
        if self <= &1 {
            return Vec::new();
        }
        fn find_cycle_by_brent(n: u64) -> u64 {
            if n == 1 {
                return 1;
            }
            if n % 2 == 0 {
                return 2;
            }
            if n.is_prime() {
                return n;
            }
            let mul = MontgomeryReduction::new(n);
            const LIMIT: u64 = 256;
            for epoch in 1..LIMIT {
                let prng_next = |x| mul.add(mul.mrmul(x, x), epoch);
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
                            q = mul.mrmul(q, max(x, y) - min(x, y));
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
        let mut ret = Vec::new();
        let mut t = *self;
        for &p in &[2, 3, 5, 7, 11, 13, 17] {
            if t.is_prime() || t <= p {
                break;
            }
            while t % p == 0 {
                ret.push(p);
                t /= p;
            }
        }
        let p = find_cycle_by_brent(t);
        if t == 1 || p == 1 {
            ret
        } else if p == t {
            ret.push(p);
            ret
        } else {
            ret.append(&mut p.prime_factorize());
            ret.append(&mut (t / p).prime_factorize());
            ret.sort_unstable();
            ret
        }
    }
}
#[test]
fn test() {
    assert_eq!(1.prime_factorize(), Vec::<u64>::new());
    assert_eq!(2023.prime_factorize(), vec![7, 17, 17]);
    assert_eq!(
        9999999999999.prime_factorize(),
        vec![3, 3, 53, 79, 265371653]
    );
    assert_eq!(10023859281455311421.prime_factorize().len(), 2);
}
