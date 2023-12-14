//! 2次元BIT
//! # Fenwick Tree 2D
//! アーベル群の二項演算を載せることができる二次元構造
//!

use algebra::AbelianGroup;
use prelude::*;

#[codesnip::entry("binary-indexed-tree-2d", doc_hidden)]
pub struct BinaryIndexedTree2<A: AbelianGroup> {
    h: usize,
    w: usize,
    bit: Vec<Vec<A::M>>,
}

#[codesnip::entry("binary-indexed-tree-2d", doc_hidden)]
impl<A: AbelianGroup> BinaryIndexedTree2<A> {
    pub fn new(h: usize, w: usize) -> Self {
        let (h, w) = (h + 1, w + 1);
        let bit = vec![vec![A::unit(); w]; h];
        BinaryIndexedTree2 { h, w, bit }
    }

    pub fn add(&mut self, y: usize, x: usize, v: A::M) {
        let mut idx = x as i32 + 1;
        while idx < self.w as i32 {
            let mut idy = y as i32 + 1;
            while idy < self.h as i32 {
                self.bit[idy as usize][idx as usize] =
                    A::op(&self.bit[idy as usize][idx as usize], &v);
                idy += idy & -idy;
            }
            idx += idx & -idx;
        }
    }

    /// sum of 0 <= y <= h & 0 <= x <= w
    pub fn sum(&self, y: usize, x: usize) -> A::M {
        let mut ret = A::unit();
        let mut idx = x as i32 + 1;
        while idx > 0 {
            let mut idy = y as i32 + 1;
            while idy > 0 {
                ret = A::op(&ret, &self.bit[idy as usize][idx as usize]);
                idy -= idy & -idy;
            }
            idx -= idx & -idx;
        }
        ret
    }

    pub fn sum_ab(&self, (y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> A::M {
        A::op(
            &A::op(&self.sum(y2, x2), &self.sum(y1, x1)),
            &A::op(&A::inv(&self.sum(y1, x2)), &A::inv(&self.sum(y2, x1))),
        )
    }
}

#[codesnip::entry("binary-indexed-tree-2d", doc_hidden)]
impl<A: AbelianGroup> Debug for BinaryIndexedTree2<A>
where
    A::M: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
        write!(f, "{}", buf)
    }
}
