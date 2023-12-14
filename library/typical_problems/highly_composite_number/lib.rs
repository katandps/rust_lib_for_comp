//! # 高度合成数
//! 自然数で、それ未満のどの自然数よりも約数の個数が多いもの
//!
//! ## x以下の最大の高度合成数と、その約数の個数
//! ### x = 1
//! 1(1個)
//! ### x = 10
//! 6(4個)

use prelude::*;

#[codesnip::entry("highly_composite_number", doc_hidden)]
pub struct HighlyCompositeNumber;

#[codesnip::entry("highly_composite_number", doc_hidden)]
impl HighlyCompositeNumber {
    /// 上限値
    const LIM: usize = 1_000_000_000_000_000_000;
    /// 使用されうる素数
    const PRIMES: [usize; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    /// returns Vec<(高度合成数, 約数の個数, vec<素因数の数>)>
    pub fn hcn() -> Vec<(usize, usize, Vec<usize>)> {
        let mut hcn = vec![(1, 1, Vec::new())];
        for i in 0..Self::PRIMES.len() {
            let mut new_hcn = Vec::new();
            for el in hcn {
                new_hcn.push(el.clone());
                if el.2.len() < i {
                    continue;
                }
                let e_max = if i >= 1 {
                    el.2[i - 1]
                } else {
                    (Self::LIM as f64).log2() as usize
                };
                let mut n: usize = el.0;
                for e in 1..=e_max {
                    n = n.saturating_mul(Self::PRIMES[i]);
                    if n > Self::LIM {
                        break;
                    }
                    let div = el.1 * (e + 1);
                    let mut exp = el.2.clone();
                    exp.push(e);
                    new_hcn.push((n, div, exp));
                }
            }
            new_hcn.sort();
            hcn = vec![(1, 1, Vec::new())];
            for el in new_hcn {
                if el.1 > hcn[hcn.len() - 1].1 {
                    hcn.push(el);
                }
            }
        }
        hcn
    }
}

#[test]
fn test() {
    let hc = HighlyCompositeNumber::hcn();
    assert_eq!(hc.len(), 156);
    assert_eq!(hc[0], (1, 1, Vec::new()));
    assert_eq!(hc[9], (120, 16, vec![3, 1, 1])); // 2*2*2*3*5
}
