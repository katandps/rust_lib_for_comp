//! 後退解析(ゲーム問題)

use crate::graph::GraphTrait;
use crate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WinLose {
    DRAW,
    WIN,
    LOSE,
}

/// 後退解析で各点をスタートとしたときの勝敗
pub struct RetrogradeAnalysis {
    result: Vec<WinLose>,
}

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
            g.rev_edges(src).iter().for_each(|e| {
                if ret[e.dst] == WinLose::DRAW {
                    deg[e.dst] -= 1;
                    if ret[src] == WinLose::LOSE {
                        ret[e.dst] = WinLose::WIN;
                        q.push_back(e.dst);
                    } else if deg[e.dst] == 0 {
                        ret[e.dst] = WinLose::LOSE;
                        q.push_back(e.dst);
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
