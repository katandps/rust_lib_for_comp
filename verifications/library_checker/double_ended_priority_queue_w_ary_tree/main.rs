// verification-helper: PROBLEM https://judge.yosupo.jp/problem/double_ended_priority_queue
//! # PQの問題を座圧とw-ary-treeで解く
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use complete_64_part_tree::Complete64PartTree;
use compress::compress_with_reverse;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let (n, q) = io.v2::<usize, usize>();
    let mut s = io.vec::<i64>(n);
    let mut queries = Vec::new();
    for _ in 0..q {
        let q = io.v::<usize>();
        if q == 0 {
            let x = io.v();
            s.push(x);
            queries.push(x);
        } else if q == 1 {
            queries.push(std::i64::MIN + 1);
        } else {
            queries.push(std::i64::MIN + 2);
        }
    }
    let (comp, rev) = compress_with_reverse(&s);
    let mut tree = Complete64PartTree::build(comp.len() as u64);
    let mut cnt = vec![0; comp.len()];
    for &si in comp.iter().take(n) {
        if cnt[si] == 0 {
            tree.insert(si as u64);
        }
        cnt[si] += 1;
    }
    let mut i = n;
    for q in queries {
        if q == std::i64::MIN + 1 {
            let p = tree.min().unwrap();
            cnt[p as usize] -= 1;
            if cnt[p as usize] == 0 {
                tree.remove(p);
            }
            io.out(rev[p as usize].line());
        } else if q == std::i64::MIN + 2 {
            let p = tree.max().unwrap();
            cnt[p as usize] -= 1;
            if cnt[p as usize] == 0 {
                tree.remove(p);
            }
            io.out(rev[p as usize].line());
        } else {
            dbg!(comp[i]);
            if cnt[comp[i]] == 0 {
                tree.insert(comp[i] as u64);
            }
            cnt[comp[i]] += 1;

            i += 1;
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "4 10
        -3 0 1 3
        0 3
        2
        2
        0 -2
        0 1
        1
        1
        2
        1
        2",
        "3
        3
        -3
        -2
        1
        0
        1
        ",
    ));
}
