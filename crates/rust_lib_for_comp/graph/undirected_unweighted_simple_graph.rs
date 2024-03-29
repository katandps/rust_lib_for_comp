//! # 辺の重みが均一(1)な単純無向グラフ

use crate::data_structure::bit_set::BitSet;
use crate::graph::GraphTrait;
use crate::prelude::*;
use crate::util::string_util::{AddLineTrait, JoinTrait};

#[codesnip::entry("unweighted-graph")]
pub use undirected_unweighted_simple_graph_impl::UndirectedUnweightedSimpleGraph;
#[codesnip::entry(
    "unweighted-graph",
    include("bit-set", "graph", "prelude", "string-util")
)]
mod undirected_unweighted_simple_graph_impl {
    use super::{swap, AddLineTrait, BitSet, Debug, Formatter, GraphTrait, JoinTrait};
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
                panic!("Can not add self loop.");
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

    impl Debug for UndirectedUnweightedSimpleGraph {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "\n{}",
                (0..self.n)
                    .map(|i| (0..self.n)
                        .map(|j| usize::from(self.get(i, j)))
                        .join("")
                        .line())
                    .join("")
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
        assert_eq!(graph.count_ones(), 0);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(3, 4);
        assert_eq!(graph.edges(0), vec![(1, 1)]);
        assert_eq!(graph.edges(1), vec![(0, 1), (2, 1)]);
        assert_eq!(graph.edges(2), vec![(1, 1)]);
        assert_eq!(graph.edges(3), vec![(4, 1)]);
        assert_eq!(graph.edges(4), vec![(3, 1)]);

        // reverse is same
        assert_eq!(graph.rev_edges(0), vec![(1, 1)]);
        assert_eq!(graph.rev_edges(1), vec![(0, 1), (2, 1)]);
        assert_eq!(graph.rev_edges(2), vec![(1, 1)]);
        assert_eq!(graph.rev_edges(3), vec![(4, 1)]);
        assert_eq!(graph.rev_edges(4), vec![(3, 1)]);

        assert_eq!(graph.count_ones(), 3);
        assert_eq!(graph.size(), 5);

        let cloned = graph.clone();
        let debug = format!("{:?}", cloned);
        assert_eq!(debug.as_str(), "\n01000\n10100\n01000\n00001\n00010\n");
    }

    #[test]
    #[should_panic]
    fn test_self_loop() {
        let mut graph = UndirectedUnweightedSimpleGraph::new(3);
        graph.add_edge(1, 1);
    }
}
