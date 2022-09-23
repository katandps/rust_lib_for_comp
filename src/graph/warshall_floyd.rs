//! # ワーシャルフロイド法
use crate::algebra::{BoundedAbove, Zero};
use crate::graph::GraphTrait;
use crate::prelude::*;

macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

///
/// ワーシャルフロイド法で(i,j)間の最短距離を求める
/// ## 計算量
/// 頂点数をNとしてO(N^3)
/// ```
/// use rust_lib_for_comp::graph::adjacency_list::Graph;
/// use rust_lib_for_comp::graph::warshall_floyd::WarshallFloyd;
///
/// let mut graph = Graph::new(5);
/// graph.add_arc(0, 1, 1);
/// graph.add_arc(1, 2, 2);
/// graph.add_arc(2, 3, 3);
/// graph.add_arc(3, 4, 4);
/// graph.add_arc(4, 0, 5);
/// let wf = graph.warshall_froyd();
/// assert_eq!(1, wf[0][1]);
/// assert_eq!(3, wf[0][2]);
/// assert_eq!(6, wf[0][3]);
/// assert_eq!(10, wf[0][4]);
/// assert_eq!(12, wf[3][2]);
/// ```
#[snippet(name = "warshall-floyd", doc_hidden)]
pub trait WarshallFloyd<W> {
    fn warshall_froyd(&self) -> Vec<Vec<W>>;
}

#[snippet(name = "warshall-floyd", doc_hidden)]
impl<G: GraphTrait<Weight = W>, W: Copy + PartialOrd + BoundedAbove + Add<Output = W> + Zero>
    WarshallFloyd<W> for G
{
    fn warshall_froyd(&self) -> Vec<Vec<W>> {
        let mut dist = vec![vec![W::max_value(); self.size()]; self.size()];
        dist.iter_mut()
            .enumerate()
            .for_each(|(i, reti)| reti[i] = W::zero());
        (0..self.size()).for_each(|src| {
            for (dst, weight) in self.edges(src) {
                chmin!(dist[src][dst], weight);
            }
        });
        for i in 0..self.size() {
            for j in 0..self.size() {
                for k in 0..self.size() {
                    if dist[j][i] < W::max_value() && dist[i][k] < W::max_value() {
                        chmin!(dist[j][k], dist[j][i] + dist[i][k]);
                    }
                }
            }
        }
        dist
    }
}
