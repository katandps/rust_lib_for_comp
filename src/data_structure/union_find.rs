//! # 素集合データ構造 (UnionFind)
//! size圧縮かつrank圧縮を行うUnionFindTree
//! ## 計算量
//!  - クエリ: $O( \alpha )$
//! ## verify
//! [ARC056B](https://atcoder.jp/contests/arc056/submissions/26705121)

use crate::prelude::*;

#[snippet(name = "union-find-tree", doc_hidden)]
#[derive(Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

#[snippet(name = "union-find-tree", doc_hidden)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n + 1).collect::<Vec<_>>();
        let rank = vec![0; n + 1];
        let size = vec![1; n + 1];
        Self { parent, rank, size }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.root(self.parent[x]);
            self.parent[x]
        }
    }

    pub fn rank(&self, x: usize) -> usize {
        self.rank[x]
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// # 併合する
    /// ## 返り値
    /// 新たに併合したときtrue 何もしなかった場合はfalse
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
        }
        if self.rank(x) < self.rank(y) {
            swap(&mut x, &mut y);
        }
        if self.rank(x) == self.rank(y) {
            self.rank[x] += 1;
        }
        self.parent[x] = y;
        self.size[y] += self.size[x];
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn it_works() {
        let mut uf = UnionFind::new(5);

        uf.unite(1, 2);
        uf.unite(2, 3);
        uf.unite(3, 4);

        assert_eq!(uf.root(1), uf.root(2));
        assert_eq!(uf.root(1), uf.root(3));
        assert_eq!(uf.root(1), uf.root(4));
        assert_ne!(uf.root(1), uf.root(5));
    }
}
