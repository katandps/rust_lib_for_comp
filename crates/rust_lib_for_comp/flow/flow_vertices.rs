//! # 頂点の割り振り
//! 頂点のグループを表す列挙子とその番号から、頂点集合全体での番号を割り振って返す

use crate::prelude::*;

#[codesnip::entry("flow_vertices")]
#[derive(Clone, Copy, Debug)]
pub enum VertexType {
    S,
    T,
}
#[codesnip::entry("flow_vertices")]
impl VertexType {
    const PATTERNS: usize = 2;
}
#[codesnip::entry("flow_vertices")]
#[derive(Clone, Debug)]
pub struct FlowVertices {
    count: usize,
    data: Vec<Vec<usize>>,
}
#[codesnip::entry("flow_vertices")]
impl Default for FlowVertices {
    fn default() -> Self {
        FlowVertices {
            count: 0,
            data: vec![Vec::new(); VertexType::PATTERNS],
        }
    }
}

#[codesnip::entry("flow_vertices")]
impl FlowVertices {
    pub fn get(&mut self, t: VertexType, v: usize) -> usize {
        let u = t as usize;
        while self.data[u].len() <= v {
            self.data[u].push(self.count);
            self.count += 1;
        }
        self.data[u][v]
    }
}
