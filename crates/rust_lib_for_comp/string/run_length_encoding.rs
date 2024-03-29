//! # ランレングス圧縮
//!
//! ## 計算量
//! $O(N)$

#[codesnip::entry("run-length-encoding")]
pub fn encode(s: &[char]) -> Vec<char> {
    let mut cur = '-';
    let mut cnt = 0;
    let mut ret = String::new();
    for c in s {
        if &cur != c {
            if cnt != 0 {
                ret += &format!("{}{}", cur, cnt);
            }
            cur = *c;
            cnt = 0;
        }
        cnt += 1;
    }
    if cnt > 0 {
        ret += &format!("{}{}", cur, cnt);
    }
    ret.chars().collect()
}

#[test]
fn test() {
    let src = "aabbbaad".chars().collect::<Vec<_>>();
    let expect = "a2b3a2d1".chars().collect::<Vec<_>>();
    assert_eq!(encode(&src), expect)
}
