//! # 部分永続UnionFindTree
//!
//! ## verify
//! [AGC002D](https://atcoder.jp/contests/agc002/submissions/35202210)
//!

use crate::prelude::*;

#[snippet(name = "partially-persistent-union-find-tree", doc_hidden)]
pub struct PartiallyPersistentUnionFind {
    // 各頂点の親
    parent: Vec<usize>,
    // 各頂点を親とする木の高さ
    rank: Vec<usize>,
    // (時刻、各頂点と連結な頂点の数)
    size: Vec<Vec<(usize, usize)>>,
    // 現在時刻
    now: usize,
    // 直接の親を更新した時刻
    time: Vec<usize>,
}

#[snippet(name = "partially-persistent-union-find-tree", doc_hidden)]
impl PartiallyPersistentUnionFind {
    /// # 初期化
    /// 1-indexedで$n$まで初期化される
    pub fn new(n: usize) -> Self {
        let parent = (0..n + 1).collect::<Vec<_>>();
        let rank = vec![0; n + 1];
        let time = vec![1 << 32; n + 1];
        let size = vec![vec![(0, 1)]; n + 1];
        let now = 0;
        Self {
            parent,
            rank,
            time,
            size,
            now,
        }
    }

    /// # 根
    /// 時刻$t$における$x$を含む木の根
    pub fn root(&self, x: usize, t: usize) -> usize {
        if t < self.time[x] {
            x
        } else {
            self.root(self.parent[x], t)
        }
    }

    /// # 同根
    /// 時刻$t$において$x$と$y$が同じ木に属するか
    pub fn same(&self, x: usize, y: usize, t: usize) -> bool {
        self.root(x, t) == self.root(y, t)
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        self.now += 1;
        let mut x = self.root(x, self.now);
        let mut y = self.root(y, self.now);
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            swap(&mut x, &mut y);
        }
        if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        let p = (self.now, self.size(x, self.now) + self.size(y, self.now));
        self.size[x].push(p);
        self.parent[y] = y;
        self.time[y] = self.now;
        true
    }

    pub fn size(&self, x: usize, t: usize) -> usize {
        let r = self.root(x, t);
        let (mut ok, mut ng) = (0, self.size[r].len());
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            if self.size[r][mid].0 <= t {
                ok = mid
            } else {
                ng = mid
            }
        }
        self.size[r][ok].1
    }
}
