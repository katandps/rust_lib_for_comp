//! # 集合の上位k個の総和
//! アーベル群である順序集合の上位k個について、総和を求める

use crate::algebra::AbelianGroup;
use crate::prelude::*;

#[codesnip::entry("priority-sum")]
pub use priority_sum_impl::PrioritySum;
#[codesnip::entry("priority-sum", include("algebra", "prelude"))]
mod priority_sum_impl {
    use super::{AbelianGroup, BinaryHeap, Ordering, Reverse};

    #[derive(Debug, Clone, Default)]
    pub struct PrioritySum<A: AbelianGroup>
    where
        A::M: Ord,
    {
        sum: A::M,
        k: usize,
        in_v: BinaryHeap<A::M>,
        out_v: BinaryHeap<Reverse<A::M>>,
        d_in: BinaryHeap<A::M>,
        d_out: BinaryHeap<Reverse<A::M>>,
        abelian_group: A,
    }

    impl<A: AbelianGroup> PrioritySum<A>
    where
        A::M: Ord,
    {
        /// # $k$で初期化
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn new(k: usize, abelian_group: A) -> Self {
            Self {
                sum: A::unit(),
                k,
                in_v: BinaryHeap::new(),
                out_v: BinaryHeap::new(),
                d_in: BinaryHeap::new(),
                d_out: BinaryHeap::new(),
                abelian_group,
            }
        }

        /// # 集合に$x$を追加
        ///
        /// ## 計算量
        /// ならし$O(\logN)$ ($N$は集合の要素数)
        pub fn add(&mut self, x: A::M) {
            self.sum = self.abelian_group.op(&self.sum, &x);
            self.in_v.push(x);
            self.modify();
        }

        /// # 集合から$x$を削除
        ///
        /// ## 計算量
        /// ならし$O(\logN)$ ($N$は集合の要素数)
        pub fn erase(&mut self, x: A::M) {
            if let Some(xd) = self.in_v.pop() {
                match x.cmp(&xd) {
                    Ordering::Equal => self.sum = self.abelian_group.op(&self.sum, &A::inv(&x)),
                    Ordering::Greater => {
                        self.in_v.push(xd);
                        self.d_out.push(Reverse(x));
                    }
                    Ordering::Less => {
                        self.sum = self.abelian_group.op(&self.sum, &A::inv(&x));
                        self.in_v.push(xd);
                        self.d_in.push(x);
                    }
                }
            } else {
                self.d_out.push(Reverse(x));
            }
            self.modify();
        }

        /// # 総和を取得
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn query(&self) -> A::M {
            self.sum.clone()
        }

        /// # 集合が空かどうかを返す
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// # 集合の個数を取得
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn len(&self) -> usize {
            self.in_v.len() + self.out_v.len() - self.d_in.len() - self.d_out.len()
        }

        /// # $k$の更新
        ///
        /// ## 計算量
        /// $O((kの変更量)*\logN) ($N$は集合の要素数)
        pub fn set_k(&mut self, k: usize) {
            self.k = k;
            self.modify()
        }

        /// # 現在の$k$を取得
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn k(&self) -> usize {
            self.k
        }

        fn modify(&mut self) {
            while self.in_v.len() < self.k + self.d_in.len() && !self.out_v.is_empty() {
                if let Some(p) = self.out_v.pop() {
                    if let Some(q) = self.d_out.pop() {
                        if p != q {
                            self.d_out.push(q);
                            self.sum = self.abelian_group.op(&self.sum, &p.0);
                            self.in_v.push(p.0);
                        }
                    } else {
                        self.sum = self.abelian_group.op(&self.sum, &p.0);
                        self.in_v.push(p.0);
                    }
                }
            }
            while self.in_v.len() > self.k + self.d_in.len() {
                if let Some(p) = self.in_v.pop() {
                    if let Some(q) = self.d_in.pop() {
                        if p != q {
                            self.d_in.push(q);
                            let p_inv = A::inv(&p);
                            self.sum = self.abelian_group.op(&self.sum, &p_inv);
                            self.out_v.push(Reverse(p));
                        }
                    } else {
                        let p_inv = A::inv(&p);
                        self.sum = self.abelian_group.op(&self.sum, &p_inv);
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

#[test]
fn test() {
    use crate::algebra::binary_operation::addition::Addition;
    use crate::algo::xor_shift::XorShift;
    let mut xorshift = XorShift::default();

    let k = 50;
    let mut v = Vec::new();
    let mut ps = PrioritySum::<Addition<i64>>::new(k, Addition::default());
    for _ in 0..1000 {
        let ai = xorshift.rand_range(0..100000);
        v.push(ai);
        ps.add(ai);
        v.sort();
        let sum = v.iter().take(k).sum::<i64>();
        assert_eq!(sum, ps.query());
    }
}
