//! # デカルト木
//!
//! 列のインデックスをノードの値に持つ二分探索木
//!
//! $O(N)$ で構築でき、頂点$(u, v)$のLCAが区間[u, v]の最小値になる
use adjacency_list::Graph;
use prelude::*;

#[codesnip::entry("cartesian_tree", doc_hidden)]
pub use cartesian_tree_impl::CartesianTree;
#[codesnip::entry("cartesian_tree", doc_hidden)]
mod cartesian_tree_impl {
    use super::Graph;

    #[derive(Clone, Debug)]
    pub struct CartesianTree {
        pub root: usize,
        pub graph: Graph<()>,
    }

    impl CartesianTree {
        pub fn build<T: PartialOrd>(src: &[T]) -> Self {
            let n = src.len();
            let mut p = vec![!0; n];
            let mut stack = Vec::with_capacity(n);
            for i in 0..n {
                let mut prev = !0;
                while let Some(s) = stack.pop() {
                    if src[i] > src[s] {
                        stack.push(s);
                        break;
                    }
                    prev = s;
                }
                if prev != !0 {
                    p[prev] = i;
                }
                if let Some(s) = stack.pop() {
                    p[i] = s;
                    stack.push(s);
                }
                stack.push(i);
            }
            let mut root = !0;
            let mut graph = Graph::new(n);
            for (i, &pi) in p.iter().enumerate() {
                if pi != !0 {
                    graph.add_arc(pi, i, ());
                } else {
                    root = i
                }
            }
            Self { root, graph }
        }
    }
}
