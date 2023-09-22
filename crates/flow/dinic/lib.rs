//! # ディニッツ法
//! 最大流問題を解く
//!
//! ## 計算量
//! 頂点数を$V$、辺数を$E$として
//! $O(V^{2}E)$
//! ただし、ほとんどの場合さらに高速に動作する
//!
use adjacency_list::Graph;
use algebra::*;
use graph::GraphTrait;
use prelude::*;

#[snippet(name = "dinic", doc_hidden)]
#[derive(Clone, Debug)]
pub struct Dinic<C: Clone + Debug> {
    /// グラフ
    graph: Graph<C>,
    /// sからの最短距離
    level: Vec<Option<usize>>,
    iter: Vec<usize>,
    rev: Vec<usize>,
}

#[snippet(name = "dinic", doc_hidden)]
impl<C: Copy + Debug + Zero + Ord + BoundedAbove + AddAssign + SubAssign> Dinic<C> {
    pub fn new(v: usize) -> Self {
        Dinic {
            graph: Graph::new(v),
            level: vec![None; v],
            iter: vec![0; v],
            rev: Vec::with_capacity(v),
        }
    }

    /// #最大流量capを持つ辺src->dstを設定する
    pub fn add_edge(&mut self, src: usize, dst: usize, cap: C) {
        let i = self.graph.add_arc(src, dst, cap);
        let j = self.graph.add_arc(dst, src, C::zero());
        self.rev.resize(j + 1, 0);
        self.rev[i] = j;
        self.rev[j] = i;
    }

    /// # 最大フロー問題を解く
    /// ## 計算量
    /// $O(V^2E)$
    pub fn max_flow(&mut self, s: usize, t: usize) -> C {
        let v = self.level.len();
        let mut flow = C::zero();
        loop {
            self.bfs(s);
            if self.level[t].is_none() {
                return flow;
            }
            self.iter = vec![0; v];
            while let Some(f) = self.dfs(s, t, C::max_value()) {
                flow += f;
            }
        }
    }

    /// 辺の重みを1としたときのsを始点としたDAGを求める
    fn bfs(&mut self, s: usize) {
        self.level = vec![None; self.level.len()];
        self.level[s] = Some(0);
        let mut deque = VecDeque::new();
        deque.push_back(s);
        while let Some(src) = deque.pop_front() {
            for (dst, cap) in self.graph.edges(src) {
                if cap > C::zero() && self.level[dst].is_none() {
                    self.level[dst] = self.level[src].map(|k| k + 1);
                    deque.push_back(dst);
                }
            }
        }
    }

    /// DAG上で流せるフローを発見し、流す
    fn dfs(&mut self, src: usize, t: usize, f: C) -> Option<C> {
        if src == t {
            return Some(f);
        }
        while self.iter[src] < self.graph.index[src].len() {
            let next = self.graph.index[src][self.iter[src]];
            let rev = self.rev[next];
            let (_src, dst, cap) = self.graph.edges[next];
            if cap > C::zero() && self.level[src] < self.level[dst] {
                let d = self.dfs(dst, t, min(f, cap));
                if let Some(d) = d {
                    self.graph.edges[next].2 -= d;
                    self.graph.edges[rev].2 += d;
                    return Some(d);
                }
            }
            self.iter[src] += 1;
        }
        None
    }
}

#[snippet(name = "dinic", doc_hidden)]
impl<C: Copy + Display + Debug + Add<Output = C>> Dinic<C> {
    pub fn result(&self) {
        for i in 0..self.graph.size() {
            for &j in &self.graph.index[i] {
                let (src, dst, cap) = self.graph.edges[j];
                let (_rev_src, _rev_dst, rev_cap) = self.graph.edges[self.rev[j]];
                println!("{} -> {} (flow: {}/{})", src, dst, rev_cap, rev_cap + cap);
            }
        }
    }
}

/// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_6_A
pub fn grl_6_a(v: usize, _e: usize, uvc: &[(usize, usize, i64)]) -> i64 {
    let mut dinic = Dinic::new(v);
    for &(u, v, c) in uvc {
        dinic.add_edge(u, v, c);
    }
    dinic.max_flow(0, v - 1)
}

#[test]
fn test_grl_6_a() {
    let (u, v) = (4, 5);
    let uvc = vec![(0, 1, 2), (0, 2, 1), (1, 2, 1), (1, 3, 1), (2, 3, 2)];
    assert_eq!(3, grl_6_a(u, v, &uvc));
}
