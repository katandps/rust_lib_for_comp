//! # オイラーツアー
//! 木の頂点をdfsでたどって、通った頂点を記録する
use super::GraphTrait;
use crate::prelude::*;

#[snippet(name = "euler_tour", doc_hidden)]
#[derive(Clone, Debug)]
/// # オイラーツアー
/// - time_in: 初めてその頂点を訪れた時刻(tourのindex)
/// - time_out: 最後にその頂点を訪れた時刻(tourのindex)
/// - depth: rootから各頂点までの距離
/// - parent: 頂点の親
/// - tour: 訪れた順の頂点リスト
pub struct EulerTour {
    pub time_in: Vec<usize>,
    pub time_out: Vec<usize>,
    pub depth: Vec<usize>,
    pub parent: Vec<usize>,
    pub tour: Vec<usize>,
}

#[snippet(name = "euler_tour", doc_hidden)]
impl EulerTour {
    pub fn new<G: GraphTrait>(g: &G, root: usize) -> Self {
        let mut tour = EulerTour {
            time_in: vec![0; g.size()],
            time_out: vec![0; g.size()],
            depth: vec![0; g.size()],
            parent: vec![0; g.size()],
            tour: Vec::new(),
        };
        tour.dfs(root, root, 0, g);
        tour
    }

    fn dfs<G: GraphTrait>(&mut self, cur: usize, par: usize, d: usize, g: &G) {
        self.parent[cur] = par;
        self.depth[cur] = d;
        self.time_in[cur] = self.tour.len();

        self.tour.push(cur);

        for (dst, _) in g.edges(cur) {
            if dst == par {
                continue;
            }
            self.dfs(dst, cur, d + 1, g);
            self.tour.push(cur);
        }
        self.time_out[cur] = self.tour.len();
    }
}
