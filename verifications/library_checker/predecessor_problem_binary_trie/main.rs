// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}
use binary_trie::BinaryTrie;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let mut trie = BinaryTrie::new(32);
    let (_n, q) = io.v2::<usize, usize>();
    let t = io.digits();
    for (i, ti) in t.iter().enumerate() {
        if *ti == 1 {
            trie.insert(i as u64);
        }
    }
    for _ in 0..q {
        let (c, k) = io.v2::<usize, u64>();
        if c == 0 {
            if !trie.contains(k) {
                trie.insert(k);
            }
        } else if c == 1 {
            if trie.contains(k) {
                trie.erase(k);
            }
        } else if c == 2 {
            io.out(usize::from(trie.contains(k)).line())
        } else if c == 3 {
            if let Some(i) = trie.lower_bound(k) {
                io.out(trie.nth(i).line())
            } else {
                io.out((-1).line())
            }
        } else if c == 4 {
            if let Some(i) = trie.upper_bound(k) {
                if i > 0 {
                    io.out(trie.nth(i - 1).line())
                } else {
                    io.out((-1).line())
                }
            } else if trie.is_empty() {
                io.out((-1).line())
            } else {
                io.out(trie.max_element().line())
            }
        }
    }
    io.flush();
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6 9
        010101
        3 3
        4 3
        4 0
        0 4
        1 3
        2 4
        2 3
        3 3
        4 3",
        "3
        3
        -1
        1
        0
        4
        1",
    ));
}
