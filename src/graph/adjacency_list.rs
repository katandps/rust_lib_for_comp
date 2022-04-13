//! # 隣接リスト

use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "graph-adjacency-list", doc_hidden)]
pub struct Graph<W> {
    pub n: usize,
    /// edges[index] = (src, dst, weight)
    pub edges: Vec<(usize, usize, W)>,
    pub index: Vec<Vec<usize>>,
    pub rev_index: Vec<Vec<usize>>,
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Clone> GraphTrait for Graph<W> {
    type Weight = W;
    fn size(&self) -> usize {
        self.n
    }
    fn edges(&self, src: usize) -> Vec<(usize, W)> {
        self.index[src]
            .iter()
            .map(|i| {
                let (_src, dst, w) = &self.edges[*i];
                (*dst, w.clone())
            })
            .collect()
    }
    fn rev_edges(&self, dst: usize) -> Vec<(usize, W)> {
        self.rev_index[dst]
            .iter()
            .map(|i| {
                let (src, _dst, w) = &self.edges[*i];
                (*src, w.clone())
            })
            .collect()
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Clone> Clone for Graph<W> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            edges: self.edges.clone(),
            index: self.index.clone(),
            rev_index: self.rev_index.clone(),
        }
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Clone> Graph<W> {
    /// n: 頂点数
    pub fn new(n: usize) -> Self {
        Self {
            n,
            edges: Vec::new(),
            index: vec![Vec::new(); n],
            rev_index: vec![Vec::new(); n],
        }
    }
    /// 相互に行き来できる辺をつける
    pub fn add_edge(&mut self, src: usize, dst: usize, w: W) -> (usize, usize) {
        let i = self.edges.len();
        self.edges.push((src, dst, w.clone()));
        self.index[src].push(i);
        self.rev_index[dst].push(i);
        let j = self.edges.len();
        self.edges.push((dst, src, w));
        self.index[dst].push(j);
        self.rev_index[src].push(j);
        (i, j)
    }

    /// 一方にのみ移動できる辺をつける
    pub fn add_arc(&mut self, src: usize, dst: usize, w: W) -> usize {
        let i = self.edges.len();
        self.edges.push((src, dst, w));
        self.index[src].push(i);
        self.rev_index[dst].push(i);
        i
    }

    /// すべての辺を返す
    pub fn all_edges(&self) -> Vec<(usize, usize, W)> {
        self.edges.clone()
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Debug> Debug for Graph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "n: {}, m: {}", self.n, self.edges.len()).unwrap();
        for (src, dst, w) in &self.edges {
            writeln!(f, "({} -> {}): {:?}", src, dst, w).unwrap();
        }
        Ok(())
    }
}

#[test]
fn test() {
    let mut g = Graph::new(5);
    g.add_edge(1, 2, 3);
    g.add_arc(3, 4, 10);
    assert_eq!(vec![(2, 3)], g.edges(1));
    assert_eq!(vec![(2, 3)], g.rev_edges(1));
    assert_eq!(vec![(1, 3)], g.edges(2));
    assert_eq!(vec![(1, 3)], g.rev_edges(2));
    assert_eq!(vec![(4, 10)], g.edges(3));
    assert_eq!(Vec::<(usize, i32)>::new(), g.rev_edges(3));
    assert_eq!(Vec::<(usize, i32)>::new(), g.edges(4));
    assert_eq!(vec![(3, 10)], g.rev_edges(4));
}
