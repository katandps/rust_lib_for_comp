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
                let (_dst, src, w) = &self.edges[*i];
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
    pub fn add_edge(&mut self, src: usize, dst: usize, w: W) {
        let i = self.edges.len();
        self.edges.push((src, dst, w.clone()));
        self.index[src].push(i);
        let i = self.edges.len();
        self.edges.push((dst, src, w));
        self.rev_index[src].push(i);
    }

    /// 1方向にのみ移動できる辺をつける
    pub fn add_arc(&mut self, src: usize, dst: usize, w: W) {
        let i = self.edges.len();
        self.edges.push((src, dst, w));
        self.index[src].push(i);
    }

    pub fn all_edges(&self) -> Vec<(usize, usize, W)> {
        self.edges.clone()
    }
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
impl<W: Debug> Debug for Graph<W> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (src, dst, w) in &self.edges {
            writeln!(f, "{} -> {} {:?}", src, dst, w).unwrap();
        }
        Ok(())
    }
}
