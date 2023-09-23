//! <https://onlinejudge.u-aizu.ac.jp/problems/DSL_5_B>
use addition::Addition;
use binary_indexed_tree_2d::BinaryIndexedTree2;
use min_max_macro::{chmax, max};

pub fn solve(_n: usize, lr: &[(usize, usize, usize, usize)]) -> i64 {
    let mut bit2d = BinaryIndexedTree2::<Addition<i64>>::new(1010, 1010);
    for &(lx, ly, rx, ry) in lr {
        bit2d.add(lx + 1, ly + 1, 1);
        bit2d.add(lx + 1, ry + 1, -1);
        bit2d.add(rx + 1, ly + 1, -1);
        bit2d.add(rx + 1, ry + 1, 1);
    }
    let mut ans = 0;
    for i in 0..1010 {
        for j in 0..1010 {
            chmax!(ans, bit2d.sum(i, j));
        }
    }
    ans
}

#[test]
fn test() {
    let n = 2;
    let lr = vec![(0, 0, 3, 2), (2, 1, 4, 3)];
    assert_eq!(solve(n, &lr), 2);

    let n = 2;
    let lr = vec![(0, 0, 2, 2), (2, 0, 4, 2)];
    assert_eq!(solve(n, &lr), 1);

    let n = 3;
    let lr = vec![(0, 0, 2, 2), (0, 0, 2, 2), (0, 0, 2, 2)];
    assert_eq!(solve(n, &lr), 3);
}
