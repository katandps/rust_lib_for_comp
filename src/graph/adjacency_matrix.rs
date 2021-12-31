//! # 隣接行列

use crate::graph::adjacency_list::Graph;
use crate::prelude::*;

#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
/// i64の隣接行列からグラフを生成する O(N^2)
impl From<&Vec<Vec<i64>>> for Graph<i64> {
    fn from(w: &Vec<Vec<i64>>) -> Self {
        let n = w.len();
        let mut ret = Self::new(n);
        for i in 0..n {
            assert_eq!(n, w[i].len());
            for j in i + 1..n {
                if w[i][j] == -1 {
                    continue;
                }
                ret.add_edge(i, j, w[i as usize][j as usize]);
                ret.add_edge(j, i, w[j as usize][i as usize]);
            }
        }
        ret
    }
}
