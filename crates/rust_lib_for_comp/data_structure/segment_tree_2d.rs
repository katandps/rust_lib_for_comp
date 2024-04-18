//! # 2次元セグメント木
//! セグメント木(非再帰)
//!
use crate::algebra::*;

#[codesnip::entry("segment-tree-2d")]
pub use segment_tree_impl::SegmentTree2D;
#[codesnip::entry("segment-tree-2d", include("algebra"))]
mod segment_tree_impl {
    use super::Monoid;
    #[derive(Clone, Debug)]
    pub struct SegmentTree2D<M: Monoid, const REV: bool = false> {
        h: usize,
        w: usize,
        node: Vec<M::M>,
        monoid: M,
    }
    impl<M: Monoid, const REV: bool> SegmentTree2D<M, REV> {
        pub fn init(height: usize, width: usize, monoid: M) -> Self {
            let (mut h, mut w) = (1, 1);
            while h < height {
                h <<= 1;
            }
            while w <= width {
                w <<= 1;
            }
            Self {
                h,
                w,
                node: vec![M::unit(); 4 * h * w],
                monoid,
            }
        }
        fn id(&self, y: usize, x: usize) -> usize {
            y * 2 * self.w + x
        }
        fn query(&mut self, y: usize, mut x1: usize, mut x2: usize) -> M::M {
            let mut ret = M::unit();
            while x1 < x2 {
                if x1 & 1 == 1 {
                    ret = self.monoid.op(&ret, &self.node[self.id(y, x1)]);
                    x1 += 1;
                }
                if x2 & 1 == 1 {
                    x2 -= 1;
                    ret = self.monoid.op(&ret, &self.node[self.id(y, x2)]);
                }
                x1 >>= 1;
                x2 >>= 1;
            }
            ret
        }
        pub fn update(&mut self, mut y: usize, mut x: usize, v: M::M) {
            y += self.h;
            x += self.w;
            let id = self.id(y, x);
            self.node[id] = v;
            let mut i = y >> 1;
            while i > 0 {
                let id = self.id(i, x);
                self.node[id] = self.monoid.op(
                    &self.node[self.id(2 * i, x)],
                    &self.node[self.id(2 * i + 1, x)],
                );
                i >>= 1;
            }
            while y > 0 {
                let mut x = x >> 1;
                while x > 0 {
                    let id = self.id(y, x);
                    self.node[id] = self.monoid.op(
                        &self.node[self.id(y, 2 * x)],
                        &self.node[self.id(y, 2 * x + 1)],
                    );
                    x >>= 1;
                }
                y >>= 1;
            }
        }
        pub fn prod(&mut self, mut y1: usize, mut x1: usize, mut y2: usize, mut x2: usize) -> M::M {
            if y1 >= y2 || x1 >= x2 {
                return M::unit();
            }
            if y2 > self.h {
                y2 = self.h;
            }
            if x2 > self.w {
                x2 = self.w;
            }
            y1 += self.h;
            y2 += self.h;
            x1 += self.w;
            x2 += self.w;
            let mut ret = M::unit();
            while y1 < y2 {
                if y1 & 1 == 1 {
                    let t = self.query(y1, x1, x2);
                    ret = self.monoid.op(&ret, &t);
                    y1 += 1;
                }
                if y2 & 1 == 1 {
                    y2 -= 1;
                    let t = self.query(y2, x1, x2);
                    ret = self.monoid.op(&ret, &t);
                }
                y1 >>= 1;
                y2 >>= 1;
            }
            ret
        }
    }
}
