//! # Primal-dual法
//! 最小費用流問題を最短路反復(with Dijkstra)で解く。
//!
//! ## verify
//! [GRL_6_B](https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=6482630#2)
use crate::graph::adjacency_list::Graph;
use crate::graph::GraphTrait;
use crate::prelude::*;

macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

#[snippet(name = "primal_dual", doc_hidden)]
#[derive(Clone, Debug)]
pub struct PrimalDual<Co: Copy + Debug, Fl: Copy + Debug> {
    graph: Graph<(Fl, Co)>,
    rev: Vec<usize>,
}

#[snippet(name = "primal_dual", doc_hidden)]
impl<Co: Copy + Debug, Fl: Copy + Debug> PrimalDual<Co, Fl>
where
    Co: Ord + Zero + AddAssign + SubAssign + Add<Output = Co> + Sub<Output = Co>,
    Fl: Ord + Zero + AddAssign + SubAssign + Sub<Output = Fl> + Mul<Co, Output = Fl>,
{
    pub fn new(n: usize) -> Self {
        Self {
            graph: Graph::new(n),
            rev: Vec::with_capacity(n),
        }
    }
    pub fn add_edge(&mut self, src: usize, dst: usize, cap: Fl, cost: Co) {
        let i = self.graph.add_arc(src, dst, (cap, cost));
        let j = self
            .graph
            .add_arc(dst, src, (Fl::zero(), Co::zero() - cost));
        self.rev.resize(j + 1, 0);
        self.rev[i] = j;
        self.rev[j] = i;
    }

    pub fn min_cost_flow(&mut self, s: usize, t: usize, mut f: Fl) -> Option<Fl> {
        let v = self.graph.size();
        let mut ret = Fl::zero();
        let mut pq: BinaryHeap<Reverse<(Co, usize)>> = BinaryHeap::new();
        let mut potential = vec![Co::zero(); v];
        while f > Fl::zero() {
            let mut min_cost = vec![None; v];
            let mut prev_edge: Vec<Option<usize>> = vec![None; v];
            min_cost[s] = Some(Co::zero());
            pq.push(Reverse((Co::zero(), s)));
            while let Some(Reverse((cost, src))) = pq.pop() {
                if min_cost[src] < Some(cost) {
                    continue;
                }
                for &i in &self.graph.index[src] {
                    let (_src, dst, (cap, cost)) = self.graph.edges[i];
                    if cap == Fl::zero() {
                        continue;
                    }
                    if let Some(m) = min_cost[src] {
                        let next_cost = m + cost + potential[src] - potential[dst];
                        if min_cost[dst].is_none() || min_cost[dst] > Some(next_cost) {
                            min_cost[dst] = Some(next_cost);
                            prev_edge[dst] = Some(i);
                            pq.push(Reverse((min_cost[dst].unwrap(), dst)))
                        }
                    }
                }
            }
            if min_cost[t].is_none() {
                return None;
            }
            (0..v).for_each(|i| {
                if let Some(m) = min_cost[i] {
                    potential[i] += m
                }
            });
            let mut addflow = f;
            let mut cur = t;
            while cur != s {
                let prev_i = prev_edge[cur].unwrap();
                let (src, _dst, (cap, _cost)) = self.graph.edges[prev_i];
                chmin!(addflow, cap);
                cur = src;
            }
            f -= addflow;
            ret += addflow * potential[t];
            let mut cur = t;
            while cur != s {
                let prev_i = prev_edge[cur].unwrap();
                self.graph.edges[prev_i].2 .0 -= addflow;
                self.graph.edges[self.rev[prev_i]].2 .0 += addflow;
                cur = self.graph.edges[prev_i].0;
            }
        }
        Some(ret)
    }

    /// # 流量最大fまでの傾斜を求める
    /// ## usage
    /// ## verify
    /// [ABC247G](https://atcoder.jp/contests/abc247/submissions/30989081)
    pub fn slope(&mut self, s: usize, t: usize, mut f: Fl) -> Vec<(Fl, Co)> {
        let v = self.graph.size();
        let mut ret = vec![(Fl::zero(), Co::zero())];
        let mut pq: BinaryHeap<Reverse<(Co, usize)>> = BinaryHeap::new();
        let mut potential = vec![Co::zero(); v];
        while f > Fl::zero() {
            let mut min_cost = vec![None; v];
            let mut prev_edge: Vec<Option<usize>> = vec![None; v];
            min_cost[s] = Some(Co::zero());
            pq.push(Reverse((Co::zero(), s)));
            while let Some(Reverse((cost, src))) = pq.pop() {
                if min_cost[src] < Some(cost) {
                    continue;
                }
                for &i in &self.graph.index[src] {
                    let (_src, dst, (cap, cost)) = self.graph.edges[i];
                    if cap == Fl::zero() {
                        continue;
                    }
                    if let Some(m) = min_cost[src] {
                        let next_cost = m + cost + potential[src] - potential[dst];
                        if min_cost[dst].is_none() || min_cost[dst] > Some(next_cost) {
                            min_cost[dst] = Some(next_cost);
                            prev_edge[dst] = Some(i);
                            pq.push(Reverse((min_cost[dst].unwrap(), dst)))
                        }
                    }
                }
            }
            if min_cost[t].is_none() {
                break;
            }
            (0..v).for_each(|i| {
                if let Some(m) = min_cost[i] {
                    potential[i] += m
                }
            });
            let mut addflow = f;
            let mut cur = t;
            while cur != s {
                let prev_i = prev_edge[cur].unwrap();
                let (src, _dst, (cap, _cost)) = self.graph.edges[prev_i];
                chmin!(addflow, cap);
                cur = src;
            }
            f -= addflow;
            ret.push((addflow, potential[t]));
            let mut cur = t;
            while cur != s {
                let prev_i = prev_edge[cur].unwrap();
                self.graph.edges[prev_i].2 .0 -= addflow;
                self.graph.edges[self.rev[prev_i]].2 .0 += addflow;
                cur = self.graph.edges[prev_i].0;
            }
        }
        ret
    }
}

#[snippet(name = "primal_dual", doc_hidden)]
impl<Co: Copy + Debug, Fl: Copy + Debug + Add<Output = Fl> + Display> PrimalDual<Co, Fl> {
    pub fn result(&self) {
        for i in 0..self.graph.size() {
            for &j in &self.graph.index[i] {
                let (src, dst, (cap, _cost)) = self.graph.edges[j];
                let (_rev_src, _rev_dst, (rev_cap, _rev_cost)) = self.graph.edges[self.rev[j]];
                println!("{} -> {} (flow: {}/{})", src, dst, rev_cap, rev_cap + cap);
            }
        }
    }
}
