//! # 座標圧縮 $O( N \log N)$

use prelude::*;

#[snippet(name = "compress", doc_hidden)]
pub fn compress<T: Ord>(source: &[T]) -> Vec<usize> {
    let n = source.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by_key(|&i| &source[i]);
    let mut ret = vec![0; n];
    let mut cur = 0;
    for i in 0..n {
        if i > 0 && source[idx[i - 1]] != source[idx[i]] {
            cur += 1;
        }
        ret[idx[i]] = cur;
    }
    ret
}

#[snippet(name = "compress", doc_hidden)]
pub fn compress_with_reverse<T: Ord + Clone>(source: &[T]) -> (Vec<usize>, Vec<T>) {
    let n = source.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_unstable_by_key(|&i| &source[i]);
    let mut ret = vec![0; n];
    let mut cur = 0;
    for i in 0..n {
        if i > 0 && source[idx[i - 1]] != source[idx[i]] {
            cur += 1;
        }
        ret[idx[i]] = cur;
    }
    let mut rev = vec![source[0].clone(); cur + 1];
    for i in 0..n {
        rev[ret[i]] = source[i].clone();
    }
    (ret, rev)
}

#[test]
fn compress_test() {
    let s = vec![0, 10, 100, 50, 5, 2];
    let r = compress(&s);
    assert_eq!(r, vec![0, 3, 5, 4, 2, 1]);
}
