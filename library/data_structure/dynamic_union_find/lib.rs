//! # 素集合データ構造 (UnionFind)
//! size圧縮かつrank圧縮を行うUnionFindTree 引数をDynamicに取れる
//! ## 計算量
//!  - クエリ: $O( \alpha )$

use fxhasher::HashMap;
use prelude::*;
use union_find::UnionFind;

#[codesnip::entry("dynamic-union-find-tree", doc_hidden)]
pub use dynamic_union_find_impl::DynamicUnionFind;
#[codesnip::entry("dynamic-union-find-tree", doc_hidden)]
mod dynamic_union_find_impl {
    use super::{Hash, HashMap, UnionFind};
    #[derive(Clone, Default)]
    pub struct DynamicUnionFind<T> {
        union_find: UnionFind,
        ids: Vec<T>,
        map: HashMap<T, usize>,
    }

    impl<T: Clone + Eq + Hash> DynamicUnionFind<T> {
        pub fn root(&mut self, x: &T) -> T {
            if let Some(v) = self.map.get(x) {
                self.ids[self.union_find.root(*v)].clone()
            } else {
                x.clone()
            }
        }

        pub fn rank(&self, x: &T) -> usize {
            if let Some(v) = self.map.get(x) {
                self.union_find.rank(*v)
            } else {
                0
            }
        }

        pub fn size(&mut self, x: &T) -> usize {
            if let Some(v) = self.map.get(x) {
                self.union_find.size(*v)
            } else {
                1
            }
        }

        pub fn same(&mut self, x: &T, y: &T) -> bool {
            self.root(x) == self.root(y)
        }

        fn add(&mut self, x: &T) {
            if self.map.contains_key(x) {
                return;
            }
            self.map.insert(x.clone(), self.ids.len());
            self.ids.push(x.clone());
            self.union_find.resize(self.ids.len())
        }

        /// # 併合する
        /// ## 返り値
        /// 新たに併合したときtrue 何もしなかった場合はfalse
        pub fn unite(&mut self, x: &T, y: &T) -> bool {
            self.add(x);
            self.add(y);
            let (Some(vx), Some(vy)) = (self.map.get(x), self.map.get(y)) else {
                return false;
            };
            self.union_find.unite(*vx, *vy)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_works() {
        let mut uf = DynamicUnionFind::default();

        uf.unite(&1, &2);
        uf.unite(&2, &3);
        uf.unite(&3, &4);

        assert_eq!(uf.root(&1), uf.root(&2));
        assert_eq!(uf.root(&1), uf.root(&3));
        assert_eq!(uf.root(&1), uf.root(&4));
        assert_ne!(uf.root(&1), uf.root(&5));
    }
}
