//! # 素集合データ構造 (UnionFind)
//! size圧縮かつrank圧縮を行うUnionFindTree
//! ## 計算量
//!  - クエリ: $O( \alpha )$

use crate::prelude::*;

#[codesnip::entry("union-find-tree")]
#[derive(Clone, Default)]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

#[codesnip::entry("union-find-tree", include("prelude"))]
impl UnionFind {
    /// # 初期化
    /// 1-indexedで$n$まで初期化される
    pub fn new(n: usize) -> Self {
        // 0-indexedでも1-indexedでも使えるよう、1拡張しておく
        let parent = (0..n + 1).collect::<Vec<_>>();
        let rank = vec![0; n + 1];
        let size = vec![1; n + 1];
        Self { parent, rank, size }
    }

    pub fn resize(&mut self, n: usize) {
        while self.parent.len() < n {
            self.parent.push(self.parent.len());
            self.rank.push(0);
            self.size.push(1);
        }
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
        self.parent[y] = x;
        self.size[x] += self.size[y];
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
