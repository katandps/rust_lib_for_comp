//! # グラフの隣接リスト表現

use graph::GraphTrait;
use prelude::*;

#[snippet(name = "graph-adjacency-list", doc_hidden)]
pub struct Graph<W> {
    pub n: usize,
    /// Vec<(src, dst, weight)>
    pub edges: Vec<(usize, usize, W)>,
    pub index: Vec<Vec<usize>>,
    pub rev_index: Vec<Vec<usize>>,
    // 反転した辺の番号
    pub rev: Vec<Option<usize>>,
}

#[snippet(name = "graph-adjacency-list", doc_hidden)]
mod impl_graph_adjacency_list {
    use super::{Debug, Formatter, Graph, GraphTrait, Index};

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

    impl<W: Clone> Clone for Graph<W> {
        fn clone(&self) -> Self {
            Self {
                n: self.n,
                edges: self.edges.clone(),
                index: self.index.clone(),
                rev_index: self.rev_index.clone(),
                rev: self.rev.clone(),
            }
        }
    }
    impl<W> Graph<W> {
        /// n: 頂点数
        pub fn new(n: usize) -> Self {
            Self {
                n,
                edges: Vec::new(),
                index: vec![Vec::new(); n],
                rev_index: vec![Vec::new(); n],
                rev: Vec::new(),
            }
        }

        /// 一方にのみ移動できる辺をつける
        pub fn add_arc(&mut self, src: usize, dst: usize, w: W) -> usize {
            let i = self.edges.len();
            self.edges.push((src, dst, w));
            self.index[src].push(i);
            self.rev_index[dst].push(i);
            self.rev.push(None);
            i
        }
    }
    impl<W> Index<usize> for Graph<W> {
        type Output = (usize, usize, W);
        fn index(&self, index: usize) -> &Self::Output {
            &self.edges[index]
        }
    }

    impl<W: Clone> Graph<W> {
        /// 相互に行き来できる辺をつける
        pub fn add_edge(&mut self, src: usize, dst: usize, w: W) -> (usize, usize) {
            let i = self.add_arc(src, dst, w.clone());
            let j = self.add_arc(dst, src, w);
            self.rev.push(None);
            self.rev.push(None);
            self.rev[i] = Some(j);
            self.rev[j] = Some(i);
            (i, j)
        }
        /// すべての辺を返す
        pub fn all_edges(&self) -> Vec<(usize, usize, W)> {
            self.edges.clone()
        }
    }

    impl<W: Debug> Debug for Graph<W> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "n: {}, m: {}", self.n, self.edges.len()).unwrap();
            for (src, dst, w) in &self.edges {
                writeln!(f, "({} -> {}): {:?}", src, dst, w).unwrap();
            }
            Ok(())
        }
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

#[test]
fn index_rev() {
    use xor_shift::XorShift;
    let mut xorshift = XorShift::default();
    let n = xorshift.rand_range(5000..10000) as usize;
    let mut graph = Graph::new(n);
    let m = xorshift.rand_range(10000..20000) as usize;
    for _ in 0..m {
        graph.add_arc(
            xorshift.rand_range(0..n as i64) as usize,
            xorshift.rand_range(0..n as i64) as usize,
            xorshift.rand_range(0..1000000000),
        );
    }
    for e in 0..graph.edges.len() {
        if let Some(rev) = graph.rev[e] {
            let (s, d, w) = graph[e];
            let (s_, d_, w_) = graph[rev];
            assert_eq!(s, d_);
            assert_eq!(s_, d);
            assert_eq!(w, w_);
        }
    }
}
