//! # 累積和
//!
//! 初期値とスライスから累積和を生成する
//!
//! ## 計算量
//! $O(N)$
//!
//! ## 使い方
//!
//! ```
//! # use rust_lib_for_comp::data_structure::cumulative_sum::*;
//! let v = vec![2,3,4];
//! let res = cumsum(1, &v);
//! assert_eq!(vec![1,3,6,10], res);
//! ```
use crate::prelude::*;

#[snippet(name = "cumulative-sum", doc_hidden)]
pub fn cumsum<T: Clone + Add<Output = T>>(initial: T, src: &[T]) -> Vec<T> {
    let mut ret = vec![initial];
    for i in 0..src.len() {
        ret.push(ret[i].clone() + src[i].clone());
    }
    ret
}
