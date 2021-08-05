#[allow(unused_imports)]
use segment_tree::*;

#[allow(dead_code)]
pub mod segment_tree {
    use std::fmt::Debug;

    /// 最小値を求めるセグメント木
    #[derive(Clone, Debug)]
    pub struct SegmentTree<M> {
        n: usize,
        node: Vec<M>,
    }

    pub trait Monoid: Debug + Clone + Copy {
        // 0元
        fn ident() -> Self;
        fn op(&self, rhs: &Self) -> Self;
    }

    impl<M: Monoid> SegmentTree<M> {
        pub fn new(v: &Vec<M>) -> Self {
            let size = v.len();
            let mut n = 1;
            while n < size {
                n *= 2
            }
            let mut node = vec![M::ident(); 2 * n - 1];
            for i in 0..size {
                node[i + n - 1] = v[i]
            }
            for i in (0..n - 1).rev() {
                node[i] = node[2 * i + 1].op(&node[2 * i + 2]);
            }
            Self { n, node }
        }

        /// index の値をvalに更新する
        pub fn set(&mut self, mut index: usize, val: M) {
            index += self.n - 1;
            self.node[index] = val;

            while index > 0 {
                index = (index - 1) / 2;
                self.node[index] = self.node[2 * index + 1].op(&self.node[2 * index + 2]);
            }
        }

        /// get for [a, b)
        pub fn get(&self, a: usize, b: usize) -> M {
            self.g(a, b, None, None, None)
        }

        fn g(&self, a: usize, b: usize, k: Option<usize>, l: Option<usize>, r: Option<usize>) -> M {
            let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.n));
            if r <= a || b <= l {
                M::ident()
            } else if a <= l && r <= b {
                self.node[k]
            } else {
                let vl = self.g(a, b, Some(2 * k + 1), Some(l), Some((l + r) / 2));
                let vr = self.g(a, b, Some(2 * k + 2), Some((l + r) / 2), Some(r));
                vl.op(&vr)
            }
        }
    }
}
