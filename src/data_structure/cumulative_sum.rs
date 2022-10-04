//! # 累積和
//!
//! イテレータに初期値を与え、累積和を生成する
//!
//! ## 計算量
//! $O(N)$
//!
//! ## 使い方
//!
//! ```
//! # use rust_lib_for_comp::data_structure::cumulative_sum::*;
//! let v = vec![2, 3, 4];
//! assert_eq!(vec![1, 3, 6, 10], v.cumsum(1));
//! ```
use crate::prelude::*;
#[snippet(name = "cumulative-sum", doc_hidden)]
pub trait CumulativeSum {
    type Item;
    fn cumsum(self, initial: Self::Item) -> Vec<Self::Item>;
}

#[snippet(name = "cumulative-sum", doc_hidden)]
impl<T: Clone + Add<Output = T>, I: IntoIterator<Item = T>> CumulativeSum for I {
    type Item = I::Item;
    fn cumsum(self, initial: T) -> Vec<T> {
        let mut ret = vec![initial];
        for t in self {
            ret.push(ret[ret.len() - 1].clone() + t);
        }
        ret
    }
}

#[test]
fn test() {
    let v = vec![2, 3, 4];
    assert_eq!(vec![1, 3, 6, 10], v.cumsum(1));
}
