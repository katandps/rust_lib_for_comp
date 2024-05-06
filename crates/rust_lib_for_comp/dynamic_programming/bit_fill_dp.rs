//! #
//! on off を表す部分bit列を移動させながらすべて使ってbit列を埋めるとき、i番目まででどのbit列が実現するかを判定するDP
//!
//! $ n := bit列の長さ $
//! $ k := 部分bit列の個数 $
//! $ src := 部分bit列 $
//!
//! ## 制約
//! 任意の2つの部分bit列について、bit積が0
//!
//! ## 計算量
//! $ n2^n $
//!

use crate::enumerator::bit_combination::bit_combination;

#[codesnip::entry("bit-fill-dp")]
pub struct BitFillDP;
#[codesnip::entry("bit-fill-dp", include = "bit-combination")]
impl BitFillDP {
    pub fn calc<I: IntoIterator<Item = u64>>(n: usize, k: usize, src: I) -> Vec<Vec<bool>> {
        let mut dp = vec![vec![false; 1 << n]; k + 1];
        dp[0][0] = true;
        let mut total_bits = 0;
        for (i, pi) in src.into_iter().enumerate() {
            for from in bit_combination(n, total_bits) {
                if !dp[i][from as usize] {
                    continue;
                }
                let mut pi = pi;
                pi >>= pi.trailing_zeros();
                while pi < 1 << n {
                    if from & pi == 0 {
                        dp[i + 1][(from + pi) as usize] = true;
                    }
                    pi <<= 1;
                }
            }
            total_bits += pi.count_ones() as usize;
        }
        dp
    }
}

#[test]
fn test() {
    let bits = vec![1, 2, 12, 16];
    let result = BitFillDP::calc(5, 4, bits);
    assert_eq!(
        result[0],
        (0u64..1 << 5)
            .map(|i| i.count_ones() == 0)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result[1],
        (0u64..1 << 5)
            .map(|i| i.count_ones() == 1)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        result[2],
        (0u64..1 << 5)
            .map(|i| i.count_ones() == 2)
            .collect::<Vec<_>>()
    );

    assert_eq!(
        result[3],
        (0u64..1 << 5)
            .map(|i| i.count_ones() == 4)
            .collect::<Vec<_>>()
    );

    assert_eq!(
        result[4],
        (0u64..1 << 5)
            .map(|i| i.count_ones() == 5)
            .collect::<Vec<_>>()
    );
}
