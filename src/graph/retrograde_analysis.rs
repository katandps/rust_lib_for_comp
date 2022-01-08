//! 後退解析(ゲーム問題)

use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "retrograde-analysis", doc_hidden)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WinLose {
    DRAW,
    WIN,
    LOSE,
}

#[snippet(name = "retrograde-analysis", doc_hidden)]
/// 後退解析で各点をスタートとしたときの勝敗
pub struct RetrogradeAnalysis {
    result: Vec<WinLose>,
}

#[snippet(name = "retrograde-analysis", doc_hidden)]
impl RetrogradeAnalysis {
    pub fn build<W, G>(g: &G) -> RetrogradeAnalysis
    where
        G: GraphTrait<Weight = W>,
    {
        let mut deg = g.outdegree();
        let mut ret = vec![WinLose::DRAW; g.size()];

        let mut q = VecDeque::new();
        for i in 0..g.size() {
            if deg[i] == 0 {
                ret[i] = WinLose::LOSE;
                q.push_back(i);
            }
        }
        while let Some(src) = q.pop_front() {
            g.rev_edges(src).into_iter().for_each(|(dst, _weight)| {
                if ret[dst] == WinLose::DRAW {
                    deg[dst] -= 1;
                    if ret[src] == WinLose::LOSE {
                        ret[dst] = WinLose::WIN;
                        q.push_back(dst);
                    } else if deg[dst] == 0 {
                        ret[dst] = WinLose::LOSE;
                        q.push_back(dst);
                    }
                }
            });
        }
        RetrogradeAnalysis { result: ret }
    }

    pub fn get(&self, i: usize) -> WinLose {
        self.result[i]
    }
}
