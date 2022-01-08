//! # 隣接リスト

use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "graph-adjacency-list", doc_hidden)]
pub struct Graph<W> {
    pub n: usize,
    /// edges[src][i] = (dst, weight)
    pub edges: Vec<Vec<(usize, W)>>,
    /// edges[dst][i] = (src, weight)
    pub rev_edges: Vec<Vec<(usize, W)>>,
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Clone> GraphTrait for Graph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.n
    }
    fn edges(&self, src: usize) -> Vec<(usize, W)> {
        self.edges[src].clone()
    }
    fn rev_edges(&self, src: usize) -> Vec<(usize, W)> {
        self.rev_edges[src].clone()
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Clone> Clone for Graph<W> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            edges: self.edges.clone(),
            rev_edges: self.rev_edges.clone(),
        }
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Clone> Graph<W> {
    /// n: 頂点数
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: vec![Vec::new(); n],
            rev_edges: vec![Vec::new(); n],
        }
    }
    /// 相互に行き来できる辺をつける
    pub fn add_edge(&mut self, src: usize, dst: usize, w: W) {
        self.edges[src].push((dst, w.clone()));
        self.edges[dst].push((src, w.clone()));
        self.rev_edges[src].push((dst, w.clone()));
        self.rev_edges[dst].push((src, w));
    }

    /// 1方向にのみ移動できる辺をつける
    pub fn add_arc(&mut self, src: usize, dst: usize, w: W) {
        self.edges[src].push((dst, w.clone()));
        self.rev_edges[dst].push((src, w));
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Debug> Debug for Graph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{").unwrap();
        for i in 0..self.n {
            writeln!(f, "  {{").unwrap();
            for j in 0..self.edges[i].len() {
                writeln!(f, "    {:?}", self.edges[i][j]).unwrap();
            }
            writeln!(f, "  }}").unwrap();
        }
        writeln!(f, "}}")
    }
}

#[test]
fn debug_test() {
    let mut graph = Graph::new(3);
    graph.add_edge(0, 1, 3);
    graph.add_arc(1, 2, 5);
    dbg!(graph);
}
