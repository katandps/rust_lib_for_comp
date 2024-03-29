//! # 集合のBit表現による列挙
//!
//! ## 計算量
//! 各ステップ $O(1)$
//!
//! ## see
//! 蟻本 p.144

#[codesnip::entry("bit-combination")]
pub fn bit_combination(n: usize, k: usize) -> impl Iterator<Item = u64> {
    assert!(n < 64);
    assert!(k <= n);
    std::iter::successors(Some(!(!0u64 << k)), move |&v| {
        if k == 0 {
            return None;
        }
        let x = v & v.wrapping_neg();
        let y = v + x;
        let z = (v & !y) >> (x.trailing_zeros() + 1);
        Some(y | z)
    })
    .take_while(move |&v| v < (1 << n))
}

#[test]
fn test() {
    let comb = bit_combination(10, 4);
    let mut v = vec![false; 1 << 10];
    for c in comb {
        v[c as usize] = true;
    }
    for (i, &vi) in v.iter().enumerate() {
        assert!(vi ^ (i.count_ones() != 4))
    }
}
