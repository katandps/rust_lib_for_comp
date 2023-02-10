//! # 辺の重みが均一(1)な単純無向グラフ

use bit_set::BitSet;
use graph::GraphTrait;
use prelude::*;
use string_util::{AddLineTrait, JoinTrait};

#[snippet(name = "unweighted-graph", doc_hidden)]
pub use undirected_unweighted_simple_graph_impl::UndirectedUnweightedSimpleGraph;
#[snippet(name = "unweighted-graph", doc_hidden)]
mod undirected_unweighted_simple_graph_impl {
    use super::{swap, AddLineTrait, BitSet, Debug, Display, Formatter, GraphTrait, JoinTrait};
    #[derive(Clone)]
    pub struct UndirectedUnweightedSimpleGraph {
        b: BitSet,
        n: usize,
    }

    impl UndirectedUnweightedSimpleGraph {
        #[inline]
        pub fn new(n: usize) -> Self {
            Self {
                n,
                b: BitSet::new(n * (n - 1) / 2),
            }
        }
        #[inline]
        pub fn index(&self, mut i: usize, mut j: usize) -> usize {
            if i > j {
                swap(&mut i, &mut j)
            }
            (self.n - 1 + self.n - 1 - i + 1) * i / 2 + (j - i - 1)
        }
        #[inline]
        pub fn get(&self, i: usize, j: usize) -> bool {
            if i == j {
                return false;
            }
            self.b[self.index(i, j)]
        }
        #[inline]
        pub fn add_edge(&mut self, i: usize, j: usize) {
            if i == j {
                return;
            }
            self.b.set(self.index(i, j), true)
        }
        #[inline]
        pub fn count_ones(&self) -> usize {
            self.b.count_ones() as usize
        }
    }

    impl GraphTrait for UndirectedUnweightedSimpleGraph {
        type Weight = i64;

        fn size(&self) -> usize {
            self.n
        }

        fn edges(&self, src: usize) -> Vec<(usize, Self::Weight)> {
            (0..self.n)
                .filter(|dst| self.get(src, *dst))
                .map(|i| (i, 1))
                .collect()
        }

        fn rev_edges(&self, dst: usize) -> Vec<(usize, Self::Weight)> {
            self.edges(dst)
        }
    }

    impl Display for UndirectedUnweightedSimpleGraph {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                (0..self.n * (self.n - 1) / 2)
                    .map(|i| if self.b[i] { "1" } else { "0" })
                    .join("")
                    .ln()
            )
        }
    }

    impl Debug for UndirectedUnweightedSimpleGraph {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "\n{}",
                (0..self.n)
                    .map(|i| (0..self.n)
                        .map(|j| if self.get(i, j) { "1" } else { "0" })
                        .join("")
                        .ln())
                    .join("")
                    .ln()
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut graph = UndirectedUnweightedSimpleGraph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        assert_eq!(graph.edges(0), vec![(1, 1)]);
        assert_eq!(graph.edges(1), vec![(0, 1), (2, 1)]);
        assert_eq!(graph.edges(2), vec![(1, 1)]);
        assert_eq!(graph.edges(3), vec![(4, 1)]);
        assert_eq!(graph.edges(4), vec![(3, 1)]);
    }
}
