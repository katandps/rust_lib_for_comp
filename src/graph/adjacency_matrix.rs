//! # 隣接行列

use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
pub struct GraphMatrix<W> {
    pub n: usize,
    pub matrix: Vec<Vec<W>>,
}

#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
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

#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
impl<W: Clone> Clone for GraphMatrix<W> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            matrix: self.matrix.clone(),
        }
    }
}

#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
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
#[snippet(name = "graph-adjacency-matrix", doc_hidden)]
impl From<Vec<Vec<i64>>> for GraphMatrix<i64> {
    #[snippet(name = "graph-adjacency-matrix", doc_hidden)]

    fn from(mut matrix: Vec<Vec<i64>>) -> Self {
        let n = matrix.len();
        const INF: i64 = 1 << 60;
        matrix.iter_mut().for_each(|v| {
            v.iter_mut().for_each(|val| {
                if *val == -1 {
                    *val = INF
                }
            })
        });
        GraphMatrix { n, matrix }
    }
}
