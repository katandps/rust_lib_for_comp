// verification-helper: PROBLEM https://judge.yosupo.jp/problem/factorize

use io_util::*;
use pollard_rho::PollardRho;
use string_util::*;

fn main() {
    let mut io = IO::default();
    let n = io.v::<usize>();
    for _ in 0..n {
        let p = io.v::<u64>().prime_factorize();
        io.out(format!("{} {}\n", p.len(), p.join(" ")));
    }
    io.flush();
}
