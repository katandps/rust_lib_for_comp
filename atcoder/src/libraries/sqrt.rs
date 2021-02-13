fn sqrt(a: isize) -> isize {
    let mut ng = std::i32::MAX as isize / 2;
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
