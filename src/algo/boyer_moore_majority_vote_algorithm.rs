//! # Boyer-Moore多数決アルゴリズム
//! スライスの多数決要素を探す
//! 生成されるのは多数決要素が *ある* ときの多数決要素があるINDEX ない場合の値は不定
//!
use crate::prelude::*;

#[snippet(name = "boyer-moore-vote-algorithm", doc_hidden)]
#[derive(Clone, Debug)]
pub struct BoyerMooreVoteAlgorithm(Vec<usize>);

#[snippet(name = "boyer-moore-vote-algorithm", doc_hidden)]
impl BoyerMooreVoteAlgorithm {
    pub fn run<T: PartialEq>(src: &[T]) -> Self {
        let mut cnt = 0;
        let mut result = vec![0; src.len()];
        let mut major = 0;
        for i in 0..src.len() {
            if cnt == 0 {
                major = i;
                cnt += 1;
            } else {
                cnt += if src[major] == src[i] { 1 } else { -1 };
            }
            if cnt > 0 {
                result[i] = i;
            }
        }
        Self(result)
    }
}

#[test]
fn test() {
    // 1番目、5番目、13番目が多数決要素
    let src = vec![1, 8, 7, 1, 1, 2, 4, 2, 2, 2, 2, 2, 2];
    let byva = BoyerMooreVoteAlgorithm::run(&src);
    assert_eq!(src[byva.0[0]], 1);
    assert_eq!(src[byva.0[4]], 1);
    assert_eq!(src[byva.0[12]], 2);
}
