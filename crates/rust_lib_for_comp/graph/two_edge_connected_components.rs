//! # 二重辺連結成分分解
//! 辺を一つ取り除いても連結である部分グラフ $=$ 橋を含まない部分グラフ

use super::adjacency_list::Graph;
use super::low_link::LowLink;
use super::GraphTrait;

#[codesnip::entry("two-edge-connected-components")]
pub use two_edge_connected_components_impl::TwoEdgeConnectedComponents;
#[codesnip::entry("two-edge-connected-components", include("low-link", "adjacency-list"))]
mod two_edge_connected_components_impl {
    use super::{Graph, GraphTrait, LowLink};
    #[derive(Clone, Debug)]
    pub struct TwoEdgeConnectedComponents {
        components: Vec<usize>,
        pub tree: Graph<()>,
    }
    impl TwoEdgeConnectedComponents {
        pub fn build<G: GraphTrait>(graph: &G) -> Self {
            let ll = LowLink::build(graph);
            let n = graph.size();
            let mut components = vec![!0; n];
            let mut component = 0;
            for i in 0..n {
                if components[i] == !0 {
                    component = Self::dfs(i, !0, graph, &ll, &mut components, component);
                }
            }
            let mut tree = Graph::new(component);
            for &(s, t) in &ll.bridge {
                let (x, y) = (components[s], components[t]);
                tree.add_edge(x, y, ());
            }

            Self { components, tree }
        }

        fn dfs<G: GraphTrait>(
            src: usize,
            par: usize,
            graph: &G,
            ll: &LowLink,
            components: &mut [usize],
            mut component: usize,
        ) -> usize {
            if par != !0 && !ll.is_bridge(src, par) {
                components[src] = components[par]
            } else {
                components[src] = component;
                component += 1;
            }
            for (dst, _) in graph.edges(src) {
                if components[dst] == !0 {
                    component = Self::dfs(dst, src, graph, ll, components, component);
                }
            }
            component
        }

        pub fn group(&self, v: usize) -> usize {
            self.components[v]
        }
    }
}
