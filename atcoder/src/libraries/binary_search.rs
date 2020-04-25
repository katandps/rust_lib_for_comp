#[allow(dead_code)]
mod binary_search {
    fn search() -> i32 {
        let mut ng: i32 = -1;
        let mut ok: i32 = 100;
        while (ok - ng).abs() > 1 {
            let mid = (ok + ng) / 2;
            if is_ok(mid) {
                ok = mid
            } else {
                ng = mid
            }
        }
        ok
    }

    fn is_ok(key: i32) -> bool {
        return true;
    }
}
