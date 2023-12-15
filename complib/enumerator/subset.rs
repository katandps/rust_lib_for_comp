//! # 集合の部分集合の全列挙
//! 集合を列挙し、その集合の部分集合を辞書順に大きい順から列挙する
//! ## 計算量
//! $O(N^3)$

#[codesnip::entry("subset")]
pub fn subset(n: usize) {
    // i: 集合
    for i in 1..1 << n {
        //j: iの部分集合
        let mut j = i;
        while j > 0 {
            // implement
            j = (j - 1) & i;
        }
    }
}
