//! ワーシャルフロイド法
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::prelude::*;
use crate::{chmin, min};

///
/// ワーシャルフロイド法で(i,j)間の最短距離を求める
/// ## 計算量
/// 頂点数をNとしてO(N^3)
/// ```
/// use rust_competitive_programming::graph::Graph;
/// use rust_competitive_programming::graph::warshall_floyd::WarshallFloyd;
///
/// let mut graph = Graph::new(5);
/// graph.add_arc(0, 1, 1);
/// graph.add_arc(1, 2, 2);
/// graph.add_arc(2, 3, 3);
/// graph.add_arc(3, 4, 4);
/// graph.add_arc(4, 0, 5);
/// let wf = WarshallFloyd::from(&graph);
/// assert_eq!(1, wf.query(0, 1));
/// assert_eq!(3, wf.query(0, 2));
/// assert_eq!(6, wf.query(0, 3));
/// assert_eq!(10, wf.query(0, 4));
/// assert_eq!(12, wf.query(3, 2));
/// ```

pub struct WarshallFloyd<W> {
    dist: Vec<Vec<W>>,
}
impl<W, G> From<&G> for WarshallFloyd<W>
where
    W: Copy + BoundedAbove + Add<Output = W> + Zero + PartialOrd,
    G: GraphTrait<Weight = W>,
{
    fn from(g: &G) -> Self {
        let mut dist = vec![vec![W::max_value(); g.size()]; g.size()];
        dist.iter_mut()
            .enumerate()
            .for_each(|(i, reti)| reti[i] = W::zero());
        for src in 0..g.size() {
            for edge in g.edges(src) {
                chmin!(dist[edge.src][edge.dst], edge.weight);
            }
        }
        for i in 0..g.size() {
            for j in 0..g.size() {
                for k in 0..g.size() {
                    if dist[j][i] < W::max_value() && dist[i][k] < W::max_value() {
                        chmin!(dist[j][k], dist[j][i] + dist[i][k]);
                    }
                }
            }
        }
        WarshallFloyd { dist }
    }
}

impl<W: Clone> WarshallFloyd<W> {
    pub fn query(&self, i: usize, j: usize) -> W {
        self.dist[i][j].clone()
    }
}
