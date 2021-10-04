//! 累乗のMOD

/// 累乗 `$p^{e} % m$`
/// ### 計算量
/// `$log(e)$`
pub fn pow(p: usize, e: usize, m: usize) -> usize {
    let mut result = 1;
    let mut cur = p;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result = result * cur % m;
        }
        e >>= 1;
        cur = cur * cur % m;
    }
    result
}
