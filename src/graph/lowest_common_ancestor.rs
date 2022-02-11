//! # 最近共通祖先
//! LowestCommonAncestor(LCA)を求めるライブラリ
//! 事前処理 $`N \log N`$、クエリ$` \log N `$
use super::euler_tour::EulerTour;
use crate::algebra::binary_operation::minimization::Minimization;
use crate::data_structure::sparse_table::SparseTable;
use crate::element::int_with_index::*;
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "lowest-common-ancestor", doc_hidden)]
pub struct LowestCommonAncestor {
    tour: EulerTour,
    depth: SparseTable<Minimization<IntWithIndex<usize>>>,
}

#[snippet(name = "lowest-common-ancestor", doc_hidden)]
impl LowestCommonAncestor {
    pub fn new<G: GraphTrait>(g: &G, root: usize) -> Self {
        let tour = EulerTour::new(g, root);
        let depth = SparseTable::<Minimization<IntWithIndex<usize>>>::from(
            &tour
                .tour
                .iter()
                .map(|i| tour.depth[*i])
                .enumerate()
                .map(IntWithIndex::from)
                .collect::<Vec<_>>()[..],
        );
        Self { tour, depth }
    }

    /// u,vの最近共通祖先(LCA)を求める$` (O(1))`$
    pub fn query(&self, u: usize, v: usize) -> usize {
        let (mut l, mut r) = (self.tour.time_in[u], self.tour.time_out[v]);
        if l > r {
            swap(&mut l, &mut r)
        }
        self.tour.tour[self.depth.query(l..=r).index]
    }

    /// 2頂点u,v間の距離を求める
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let lca = self.query(u, v);
        self.tour.depth[u] + self.tour.depth[v] - 2 * self.tour.depth[lca]
    }

    /// u,vを結ぶパス上に頂点aが存在するかどうか
    pub fn on_path(&self, u: usize, v: usize, a: usize) -> bool {
        self.dist(u, a) + self.dist(a, v) == self.dist(u, v)
    }

    /// # 木構造の圧縮
    /// 木を指定した部分集合とそのLCAだけの木に変形する
    /// ## 挙動
    /// vsに必要な頂点を追加し、辺のリストを返す
    pub fn auxiliary_tree(&self, vs: &mut Vec<usize>) -> Vec<(usize, usize)> {
        vs.sort_by_key(|v| self.tour.time_in[*v]);
        let mut stack = vec![vs[0]];
        let mut edges = Vec::new();
        for i in 1..vs.len() {
            let lca = self.query(vs[i - 1], vs[i]);
            if lca != vs[i - 1] {
                let mut last = stack.pop().unwrap();
                while !stack.is_empty()
                    && self.tour.depth[lca] < self.tour.depth[stack[stack.len() - 1]]
                {
                    edges.push((stack[stack.len() - 1], last));
                    last = stack.pop().unwrap();
                }
                if stack.is_empty() || stack[stack.len() - 1] != lca {
                    stack.push(lca);
                    vs.push(lca);
                }
                edges.push((lca, last));
            }
            stack.push(vs[i]);
        }
        for i in 1..stack.len() {
            edges.push((stack[i - 1], stack[i]));
        }
        edges
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::graph::adjacency_list::Graph;

    #[test]
    pub fn it_works() {
        //[0]- 1 - 3 - 6
        // |   |   |
        // 2   4   7
        // |   |
        // 5   8
        let mut graph = Graph::new(9);
        graph.add_edge(0, 1, ());
        graph.add_edge(0, 2, ());
        graph.add_edge(1, 3, ());
        graph.add_edge(1, 4, ());
        graph.add_edge(2, 5, ());
        graph.add_edge(3, 6, ());
        graph.add_edge(3, 7, ());
        graph.add_edge(4, 8, ());

        let lca = LowestCommonAncestor::new(&graph, 0);

        assert_eq!(0, lca.query(1, 5));
        assert_eq!(2, lca.query(2, 5));
        assert_eq!(1, lca.query(3, 8));
        assert_eq!(3, lca.query(6, 7));

        assert_eq!(2, lca.dist(0, 5));
        assert_eq!(4, lca.dist(5, 4));

        assert_eq!(true, lca.on_path(5, 8, 1));
        assert_eq!(false, lca.on_path(5, 8, 3));
    }

    #[test]
    pub fn auxiliary_tree() {
        //[0]- 2 - 9 - 10 - 11
        // |   |   |
        // 1   3   12
        //     |
        // 5 - 4 - 7
        //     |   |
        //     6   8
        let mut graph = Graph::new(13);
        graph.add_edge(0, 1, ());
        graph.add_edge(0, 2, ());
        graph.add_edge(2, 3, ());
        graph.add_edge(2, 9, ());
        graph.add_edge(3, 4, ());
        graph.add_edge(4, 5, ());
        graph.add_edge(4, 6, ());
        graph.add_edge(4, 7, ());
        graph.add_edge(7, 8, ());
        graph.add_edge(9, 10, ());
        graph.add_edge(9, 12, ());
        graph.add_edge(10, 11, ());

        let lca = LowestCommonAncestor::new(&graph, 0);

        let mut vs = vec![1, 5, 8, 11];
        let mut edges = lca.auxiliary_tree(&mut vs);
        edges.sort();
        vs.sort();
        assert_eq!(vec![0, 1, 2, 4, 5, 8, 11], vs);
        assert_eq!(vec![(0, 1), (0, 2), (2, 4), (2, 11), (4, 5), (4, 8)], edges);
    }
}
