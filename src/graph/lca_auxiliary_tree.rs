//! # 木構造の圧縮
//! LCAを利用して、木を指定した部分集合とそのLCAだけの木に変形する
use super::lowest_common_ancestor::LowestCommonAncestor;
use crate::graph::GraphTrait;
use crate::prelude::*;

#[snippet(name = "lca_auxiliary_tree", doc_hidden)]
pub struct LCAAuxiliaryTree {
    _lca: LowestCommonAncestor,
}

#[snippet(name = "lca_auxiliary_tree", doc_hidden)]
impl LCAAuxiliaryTree {
    pub fn new<G: GraphTrait>(_g: &G, _root: usize) -> Self {
        unimplemented!();
    }
}
