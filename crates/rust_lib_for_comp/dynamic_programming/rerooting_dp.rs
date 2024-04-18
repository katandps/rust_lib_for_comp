//! 全方位木DP
//!
//! 未完成
use crate::algebra::*;
use crate::graph::GraphTrait;

pub struct RerootingDP<M: Monoid> {
    subtree: Vec<M::M>,
    dp: Vec<M::M>,
    monoid: M,
}

impl<M: Monoid> RerootingDP<M> {
    ///
    /// # Arguments
    ///
    /// * `graph` - 対象とする木
    /// * `apply` - 部分木の値を作る関数
    /// * `leaf` - 部分木が葉のときの値を作る関数
    ///
    ///  
    pub fn solve<G: GraphTrait>(
        graph: &G,
        apply: fn(usize, usize, G::Weight, &M::M) -> M::M,
        leaf: fn(usize) -> M::M,
        monoid: M,
    ) -> Vec<M::M> {
        let mut subtree = Vec::with_capacity(graph.size());
        for i in 0..graph.size() {
            subtree.push(leaf(i))
        }
        let dp = vec![M::unit(); graph.size()];
        let mut reroot = Self {
            subtree,
            dp,
            monoid,
        };

        reroot.dfs1(0, 0, graph, apply);
        reroot.dfs2(0, 0, graph, &M::unit(), apply);
        reroot.dp
    }

    fn dfs1<G: GraphTrait>(
        &mut self,
        src: usize,
        parent: usize,
        graph: &G,
        apply: fn(usize, usize, G::Weight, &M::M) -> M::M,
    ) {
        for (dst, val) in graph.edges(src) {
            if dst == parent {
                continue;
            }
            self.dfs1(dst, src, graph, apply);
            self.subtree[src] = self.monoid.op(
                &self.subtree[src],
                &apply(dst, src, val, &self.subtree[dst]),
            );
        }
    }

    fn dfs2<G: GraphTrait>(
        &mut self,
        src: usize,
        _parent: usize,
        graph: &G,
        _val: &M::M,
        apply: fn(usize, usize, G::Weight, &M::M) -> M::M,
    ) {
        let mut ds = Vec::new();
        for (dst, val) in graph.edges(src) {
            ds.push(apply(dst, src, val, &self.subtree[dst]))
        }
        todo!()
    }
}
