//! 整数範囲での平方根(切り捨て)

#[allow(dead_code)]
fn sqrt(a: i64) -> i64 {
    let mut ng = std::i32::MAX as i64 / 8;
    let mut ok = -1;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if mid * mid <= a {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[test]
fn test() {
    assert_eq!(10, sqrt(100));
    assert_eq!(9, sqrt(81));
    assert_eq!(9, sqrt(99));
}
