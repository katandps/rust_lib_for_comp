//! # 部分永続UnionFindTree
//!
use crate::prelude::*;

#[codesnip::entry("partially-persistent-union-find-tree")]
pub use partial_persistent_union_find_impl::PartiallyPersistentUnionFind;
#[codesnip::entry("partially-persistent-union-find-tree", include("prelude"))]
mod partial_persistent_union_find_impl {
    use super::swap;
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

    impl PartiallyPersistentUnionFind {
        /// # 初期化
        /// 1-indexedで$n$まで初期化される
        pub fn new(n: usize) -> Self {
            let parent = (0..n + 1).collect::<Vec<_>>();
            let rank = vec![0; n + 1];
            let time = vec![0; n + 1];
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
        ///
        /// ## 計算量
        /// $O(logN)$
        /// 木の高さで抑えられる
        pub fn root(&self, x: usize, t: usize) -> usize {
            if t < self.time[x] || self.parent[x] == x {
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
            self.parent[y] = x;
            self.time[y] = self.now;
            true
        }

        /// # 連結成分の大きさ
        /// 二分探索による
        /// ## 計算量
        /// $O(\log N)$
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
}

#[test]
fn test() {
    let mut uf = PartiallyPersistentUnionFind::new(5);
    uf.unite(1, 3);
    uf.unite(2, 4);
    uf.unite(0, 4);
    uf.unite(0, 2);
    uf.unite(0, 1);

    let expect = [
        vec![0, 1, 2, 3, 4],
        vec![0, 1, 2, 1, 4],
        vec![0, 1, 2, 1, 2],
        vec![2, 1, 2, 1, 2],
        vec![2, 1, 2, 1, 2],
        vec![2, 2, 2, 2, 2],
    ];
    let expect_size = [
        vec![1, 1, 1, 1, 1],
        vec![1, 2, 1, 2, 1],
        vec![1, 2, 2, 2, 2],
        vec![3, 2, 3, 2, 3],
        vec![3, 2, 3, 2, 3],
        vec![5, 5, 5, 5, 5],
    ];
    for t in 0..expect.len() {
        assert_eq!(expect[t], (0..5).map(|i| uf.root(i, t)).collect::<Vec<_>>());
        assert_eq!(
            expect_size[t],
            (0..5).map(|i| uf.size(i, t)).collect::<Vec<_>>()
        );
    }
}
