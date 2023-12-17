//! # [Chopsticks](https://onlinejudge.u-aizu.ac.jp/problems/2659)

use rust_lib_for_comp::algebra::chinese_remainder_theorem::CRT;
use rust_lib_for_comp::util::io_util::*;

#[verify::aizu_online_judge("2659")]
pub fn p2659(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let (mut n, m, d) = reader.v3::<u64, usize, usize>();
    let a = reader.vec::<i64>(m);
    let r = reader.matrix::<i64>(d, m);
    let mut ok = true;
    for i in 0..d {
        let mut b = Vec::new();
        let mut mo = Vec::new();
        for (j, &aj) in a.iter().enumerate() {
            if r[i][j] != -1 {
                b.push(r[i][j] as u64);
                mo.push(aj as u64)
            }
        }
        if b.is_empty() {
            continue;
        }
        if let Some((r, m)) = CRT::crt_slice(&b, &mo) {
            if n < r {
                ok = false
            } else {
                n = n - (n - r) % m;
            }
        } else {
            ok = false;
        }
    }
    if ok {
        writeln!(write, "{n}").ok();
    } else {
        writeln!(write, "-1").ok();
    }
    write.flush().ok();
}
