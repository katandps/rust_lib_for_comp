//! # ポテンシャル付きUnionFindTree
//! アーベル群の値をポテンシャルとして、頂点間のポテンシャルを管理する
//!

use crate::algebra::AbelianGroup;
use crate::prelude::*;

#[codesnip::entry("weighted-union-find")]
#[derive(Clone, Debug)]
pub struct WeightedUnionFind<A: AbelianGroup> {
    parent: Vec<usize>,
    rank: Vec<usize>,
    weight_diff: Vec<A::M>,
    abelian_group: A,
}

#[codesnip::entry("weighted-union-find", include("algebra"))]
impl<A: AbelianGroup> WeightedUnionFind<A> {
    pub fn new(n: usize, abelian_group: A) -> Self {
        let parent = (0..n + 1).collect::<Vec<_>>();
        let rank = vec![0; n + 1];
        let weight_diff = vec![A::unit(); n + 1];
        Self {
            parent,
            rank,
            weight_diff,
            abelian_group,
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let r = self.root(self.parent[x]);
            self.weight_diff[x] = self
                .abelian_group
                .op(&self.weight_diff[x], &self.weight_diff[self.parent[x]]);
            self.parent[x] = r;
            self.parent[x]
        }
    }

    pub fn rank(&self, x: usize) -> usize {
        self.rank[x]
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn weight(&mut self, x: usize) -> A::M {
        self.root(x);
        self.weight_diff[x].clone()
    }

    /// xとyがすでに併合されているとき、併合せずfalseを返す
    pub fn unite(&mut self, x: usize, y: usize, mut weight: A::M) -> bool {
        let (wx, wy) = (self.weight(x), self.weight(y));
        weight = self.abelian_group.op(&weight, &wx);
        weight = self.abelian_group.op(&weight, &A::inv(&wy));
        let (mut x, mut y) = (self.root(x), self.root(y));
        if x == y {
            return false;
        }
        if self.rank(x) < self.rank(y) {
            swap(&mut x, &mut y);
            weight = A::inv(&weight);
        }
        if self.rank(x) == self.rank(y) {
            self.rank[x] += 1;
        }
        self.parent[y] = x;
        self.weight_diff[y] = weight;
        true
    }

    pub fn diff(&mut self, x: usize, y: usize) -> A::M {
        assert_eq!(self.root(x), self.root(y));
        let (wx, wy) = (self.weight(x), self.weight(y));
        self.abelian_group.op(&wy, &A::inv(&wx))
    }
}

#[test]
fn test() {
    use crate::algebra::binary_operation::addition::Addition;
    let n = 5;
    let mut wuf = WeightedUnionFind::<Addition<i64>>::new(n, Addition::default());
    wuf.unite(1, 2, 1);
    assert_eq!(1, wuf.diff(1, 2));
    wuf.unite(1, 3, 2);
    assert_eq!(1, wuf.diff(2, 3));
    assert_eq!(2, wuf.diff(1, 3));
    assert_eq!(-2, wuf.diff(3, 1));
    assert_eq!(-1, wuf.diff(2, 1));
}
