//! # 隣接リスト

use crate::graph::{Edge, GraphTrait};
use crate::prelude::*;

#[snippet(name = "graph-adjacency_list", doc_hidden)]
pub struct Graph<W> {
    pub n: usize,
    pub edges: Vec<Vec<Edge<W>>>,
    pub rev_edges: Vec<Vec<Edge<W>>>,
}

#[snippet(name = "graph", doc_hidden)]
impl<W: Clone> GraphTrait for Graph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.n
    }
    fn edges(&self, src: usize) -> Vec<Edge<W>> {
        self.edges[src].clone()
    }
    fn rev_edges(&self, src: usize) -> Vec<Edge<W>> {
        self.rev_edges[src].clone()
    }
}

#[snippet(name = "graph-adjacency_list", doc_hidden)]
impl<W: Clone> Clone for Graph<W> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            edges: self.edges.clone(),
            rev_edges: self.rev_edges.clone(),
        }
    }
}

#[snippet(name = "graph-adjacency_list", doc_hidden)]
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
    pub fn add_edge(&mut self, a: usize, b: usize, w: W) {
        self.edges[a].push(Edge::new(a, b, w.clone()));
        self.edges[b].push(Edge::new(b, a, w.clone()));
        self.rev_edges[a].push(Edge::new(a, b, w.clone()));
        self.rev_edges[b].push(Edge::new(b, a, w));
    }

    /// 1方向にのみ移動できる辺をつける
    pub fn add_arc(&mut self, a: usize, b: usize, w: W) {
        self.edges[a].push(Edge::new(a, b, w.clone()));
        self.rev_edges[b].push(Edge::new(b, a, w));
    }
}

#[snippet(name = "graph-adjacency_list", doc_hidden)]
impl<W: Display> Debug for Graph<W> {
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
