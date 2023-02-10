//! # 隣接行列

use graph::GraphTrait;
use prelude::*;

#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
pub use adjacency_matrix_impl::GraphMatrix;
#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
mod adjacency_matrix_impl {
    use super::{Debug, Formatter, GraphTrait};
    pub struct GraphMatrix<W> {
        pub n: usize,
        pub matrix: Vec<Vec<W>>,
    }

    impl<W: Clone> GraphTrait for GraphMatrix<W> {
        type Weight = W;
        fn size(&self) -> usize {
            self.n
        }
        fn edges(&self, src: usize) -> Vec<(usize, W)> {
            (0..self.n)
                .map(|dst| (dst, self.matrix[src][dst].clone()))
                .collect()
        }
        fn rev_edges(&self, dst: usize) -> Vec<(usize, W)> {
            (0..self.n)
                .map(|src| (src, self.matrix[src][dst].clone()))
                .collect()
        }
    }

    impl<W: Clone> Clone for GraphMatrix<W> {
        fn clone(&self) -> Self {
            Self {
                n: self.n,
                matrix: self.matrix.clone(),
            }
        }
    }

    impl<W: Debug> Debug for GraphMatrix<W> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{{").unwrap();
            for i in 0..self.n {
                for j in 0..self.n {
                    write!(f, " {:?}", self.matrix[i][j]).unwrap();
                }
            }
            writeln!(f, "}}")
        }
    }
    impl GraphMatrix<i64> {
        /// # i64の二次元Vecを隣接行列とみなして初期化する
        /// no_edgeにSome(値)を入れることで辺がないことを表せる
        pub fn new(mut matrix: Vec<Vec<i64>>, no_edge: Option<i64>) -> Self {
            let n = matrix.len();
            const INF: i64 = 1 << 60;
            matrix.iter_mut().for_each(|v| {
                v.iter_mut().for_each(|val| {
                    if Some(*val) == no_edge {
                        *val = INF
                    }
                })
            });
            GraphMatrix { n, matrix }
        }
    }
}
