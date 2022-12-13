//! # 集合の上位k個の総和
//! Cmpを実装している集合の上位k個について、アーベル群の演算結果を求める
//! 集合の操作を高速に変更できる

use crate::prelude::*;

#[snippet(name = "priority-sum", doc_hidden)]
pub use priority_sum_impl::PrioritySum;
#[snippet(name = "priority-sum", doc_hidden)]
mod priority_sum_impl {
    use super::{AbelianGroup, BinaryHeap, Reverse};

    #[derive(Debug, Clone, Default)]
    pub struct PrioritySum<M: AbelianGroup>
    where
        M::M: Ord,
    {
        sum: M::M,
        k: usize,
        in_v: BinaryHeap<M::M>,
        out_v: BinaryHeap<Reverse<M::M>>,
        d_in: BinaryHeap<M::M>,
        d_out: BinaryHeap<Reverse<M::M>>,
    }

    impl<M: AbelianGroup> PrioritySum<M>
    where
        M::M: Ord,
    {
        pub fn new(k: usize) -> Self {
            Self {
                sum: M::unit(),
                k,
                in_v: BinaryHeap::new(),
                out_v: BinaryHeap::new(),
                d_in: BinaryHeap::new(),
                d_out: BinaryHeap::new(),
            }
        }

        pub fn add(&mut self, x: M::M) {
            self.sum = M::op(&self.sum, &x);
            self.in_v.push(x);
            self.modify();
        }

        pub fn erase(&mut self, x: M::M) {
            if let Some(xd) = self.in_v.pop() {
                if x == xd {
                    self.sum = M::op(&self.sum, &M::inv(&x));
                    self.modify();
                    return;
                }
                if xd > x {
                    self.sum = M::op(&self.sum, &M::inv(&x));
                    self.d_in.push(x);
                } else {
                    self.d_out.push(Reverse(x));
                }
                self.in_v.push(xd);
            } else {
                self.d_out.push(Reverse(x));
            }
            self.modify();
        }
        pub fn query(&self) -> M::M {
            self.sum.clone()
        }
        fn modify(&mut self) {
            while self.in_v.len() < self.k + self.d_in.len() && !self.out_v.is_empty() {
                if let Some(p) = self.out_v.pop() {
                    if let Some(q) = self.d_out.pop() {
                        if p != q {
                            self.d_out.push(q);
                            self.sum = M::op(&self.sum, &p.0);
                            self.in_v.push(p.0);
                        }
                    } else {
                        self.sum = M::op(&self.sum, &p.0);
                        self.in_v.push(p.0);
                    }
                }
            }
            while self.in_v.len() > self.k + self.d_in.len() {
                if let Some(p) = self.in_v.pop() {
                    if let Some(q) = self.d_in.pop() {
                        if p != q {
                            self.d_in.push(q);
                            self.sum = M::op(&self.sum, &M::inv(&p));
                            self.out_v.push(Reverse(p));
                        }
                    } else {
                        self.sum = M::op(&self.sum, &M::inv(&p));
                        self.out_v.push(Reverse(p));
                    }
                }
            }
            while let Some(p) = self.d_in.pop() {
                if let Some(q) = self.in_v.pop() {
                    if p != q {
                        self.d_in.push(p);
                        self.in_v.push(q);
                        break;
                    }
                } else {
                    self.d_in.push(p);
                    break;
                }
            }
        }
    }
}
