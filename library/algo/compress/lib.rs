//! # 座標圧縮 $O( N \log N)$

use prelude::*;

#[snippet(name = "compress", doc_hidden)]
pub use compress_impl::Compress;
#[snippet(name = "compress", doc_hidden)]
mod compress_impl {
    pub trait Compress {
        /// # 座圧
        fn compress(self, initial_value: usize) -> Vec<usize>;
        /// # 逆変換付き座圧
        fn compress_with_reverse(self) -> (Vec<usize>, Vec<usize>);
    }
    impl<T: Ord + Clone> Compress for &[T] {
        fn compress(self, initial_value: usize) -> Vec<usize> {
            let n: usize = self.len();
            let mut idx: Vec<usize> = (0..n).collect();
            idx.sort_unstable_by_key(|&i| &self[i]);
            let mut ret = vec![0; n];
            let mut cur = initial_value;
            for i in 0..n {
                if i > 0 && self[idx[i - 1]] != self[idx[i]] {
                    cur += 1;
                }
                ret[idx[i]] = cur;
            }
            ret
        }
        fn compress_with_reverse(self) -> (Vec<usize>, Vec<usize>) {
            let n = self.len();
            let mut idx: Vec<usize> = (0..n).collect();
            idx.sort_unstable_by_key(|&i| &self[i]);
            let mut ret = vec![0; n];
            let mut cur = 0;
            for i in 0..n {
                if i > 0 && self[idx[i - 1]] != self[idx[i]] {
                    cur += 1;
                }
                ret[idx[i]] = cur;
            }
            let mut rev = vec![0; cur + 1];
            for i in 0..n {
                rev[ret[i]] = i;
            }
            (ret, rev)
        }
    }
}

#[test]
fn compress_test() {
    let s = vec![0, 10, 100, 50, 5, 2];
    let r = s[..].compress(0);
    assert_eq!(r, vec![0, 3, 5, 4, 2, 1]);
}
