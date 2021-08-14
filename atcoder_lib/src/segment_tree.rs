#[allow(unused_imports)]
use segment_tree::*;

#[allow(dead_code)]
pub mod segment_tree {
    use std::fmt::Debug;

    /// セグメント木
    /// TはFを二項演算とするモノイド
    #[derive(Clone)]
    pub struct SegmentTree<F, T> {
        n: usize,
        node: Vec<T>,
        ident: T,
        eval: F,
    }

    impl<F: Fn(T, T) -> T, T: Copy + Eq + Debug> SegmentTree<F, T> {
        /// 単位元identと二項演算Fの組を使用する、初期値をvとしたセグメント木を生成する
        pub fn new(v: &Vec<T>, ident: T, eval: F) -> Self {
            let n = (v.len() + 1).next_power_of_two();
            let mut node = vec![ident; 2 * n - 1];
            for i in 0..v.len() {
                node[i + n] = v[i]
            }
            for i in (0..n - 1).rev() {
                node[i] = eval(node[2 * i + 1], node[2 * i + 2]);
            }
            Self {
                n,
                node,
                ident,
                eval,
            }
        }

        /// index の値をvalで更新する
        pub fn set(&mut self, mut index: usize, val: T) {
            index += self.n - 1;
            self.node[index] = (self.eval)(self.node[index], val);

            while index > 0 {
                index = (index - 1) / 2;
                self.node[index] = (self.eval)(self.node[2 * index + 1], self.node[2 * index + 2]);
            }
        }

        /// get for [a, b)
        pub fn get(&self, a: usize, b: usize) -> T {
            self.g(a, b, None, None, None)
        }

        fn g(&self, a: usize, b: usize, k: Option<usize>, l: Option<usize>, r: Option<usize>) -> T {
            let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.n));
            if r <= a || b <= l {
                self.ident
            } else if a <= l && r <= b {
                self.node[k]
            } else {
                let vl = self.g(a, b, Some(2 * k + 1), Some(l), Some((l + r) / 2));
                let vr = self.g(a, b, Some(2 * k + 2), Some((l + r) / 2), Some(r));
                (self.eval)(vl, vr)
            }
        }
    }
}
