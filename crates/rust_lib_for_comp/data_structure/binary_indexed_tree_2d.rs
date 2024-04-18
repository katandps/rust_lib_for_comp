//! 2次元BIT
//! # Fenwick Tree 2D
//! アーベル群の二項演算を載せることができる二次元構造
//!

use crate::algebra::AbelianGroup;

#[codesnip::entry("binary-indexed-tree-2d", include("algebra"))]
pub use binary_indexed_tree_2d_impl::BinaryIndexedTree2;

#[codesnip::entry("binary-indexed-tree-2d", include("algebra"))]
mod binary_indexed_tree_2d_impl {
    use super::AbelianGroup;
    pub struct BinaryIndexedTree2<A: AbelianGroup> {
        h: usize,
        w: usize,
        bit: Vec<Vec<A::M>>,
        abelian_group: A,
    }

    impl<A: AbelianGroup> BinaryIndexedTree2<A> {
        pub fn new(h: usize, w: usize, abelian_group: A) -> Self {
            let (h, w) = (h + 1, w + 1);
            let bit = vec![vec![A::unit(); w]; h];
            BinaryIndexedTree2 {
                h,
                w,
                bit,
                abelian_group,
            }
        }

        pub fn add(&mut self, y: usize, x: usize, v: A::M) {
            let mut idx = x as i32 + 1;
            while idx < self.w as i32 {
                let mut idy = y as i32 + 1;
                while idy < self.h as i32 {
                    self.bit[idy as usize][idx as usize] = self
                        .abelian_group
                        .op(&self.bit[idy as usize][idx as usize], &v);
                    idy += idy & -idy;
                }
                idx += idx & -idx;
            }
        }

        /// sum of 0 <= y <= h & 0 <= x <= w
        pub fn sum(&mut self, y: usize, x: usize) -> A::M {
            let mut ret = A::unit();
            let mut idx = x as i32 + 1;
            while idx > 0 {
                let mut idy = y as i32 + 1;
                while idy > 0 {
                    ret = self
                        .abelian_group
                        .op(&ret, &self.bit[idy as usize][idx as usize]);
                    idy -= idy & -idy;
                }
                idx -= idx & -idx;
            }
            ret
        }

        pub fn sum_ab(&mut self, (y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> A::M {
            //      x1   x2
            //      |     |
            // y1 --a-----b
            //      |     |
            // y2 --c-----d
            //
            // 上記の表で (a + d) - (b + c)
            let a = self.sum(y1, x1);
            let b = self.sum(y1, x2);
            let c = self.sum(y2, x1);
            let d = self.sum(y2, x2);
            let ad = self.abelian_group.op(&a, &d);
            let bc = self.abelian_group.op(&b, &c);
            self.abelian_group.op(&ad, &A::inv(&bc))
        }

        pub fn into_string(mut self) -> String {
            let mut buf = String::new();
            buf += "\n";
            for y in 0..self.h - 1 {
                for x in 0..self.w - 1 {
                    if x > 0 {
                        buf += " ";
                    }
                    buf += format!("{:?}", self.sum(y, x)).as_str();
                }
                buf += "\n";
            }
            buf
        }
    }
}
