// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#[cfg_attr(coverage_nightly, coverage(off))]
fn main() {
    solve(io_util::IO::default());
}

use binary_trie::BinaryTrie;
use io_util::*;
use string_util::*;

pub fn solve<IO: ReaderTrait + WriterTrait>(mut io: IO) {
    let mut trie = BinaryTrie::new(30);
    for _ in 0..io.v() {
        let (c, x) = io.v2::<usize, u64>();
        if c == 0 {
            if !trie.contains(x) {
                trie.insert(x);
            }
        } else if c == 1 {
            if trie.contains(x) {
                trie.erase(x);
            }
        } else {
            trie.set_xor_val(x);
            io.out((trie.min_element().unwrap() ^ x).line());
        }
    }
    io.flush()
}

#[test]
fn test() {
    solve(io_debug::IODebug::static_assert(
        "6
        0 6
        0 7
        2 5
        1 7
        1 10
        2 7",
        "2
        1
        ",
    ))
}
