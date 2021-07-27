#[allow_unused_import]
use segment_tree::*;

#[allow_dead_code]
pub mod segment_tree {

    /// 最小値を求めるセグメント木
    pub struct SegmentTree {
        n: usize,
        node: Vec<VALUE>,
    }

    type VALUE = i64;
    const INF: VALUE = std::i64::MAX;

    impl SegmentTree {
        pub fn new(v: &Vec<VALUE>) -> Self {
            let size = v.len();
            let mut n = 1;
            while n < sz {
                n *= 2
            }
            let mut node = vec![INF; 2 * n - 1];
            for i in 0..size {
                node[i + n - 1] = v[i]
            }
            for i in (0..n - 1).rev() {
                // 最小値
                node[i] = std::cmp::min(node[2 * i + 1], node[2 * i + 2])
            }
            Self { n, node }
        }

        fn update(&mut self, mut index: usize, val: VALUE) {
            index += self.n - 1;
            self.node[x] = val;

            while index > 0 {
                index = (index - 1) / 2;
                node[x] = std::cmp::min(node[2 * index + 1], node[2 * index + 2])
            }
        }

        fn get_min(
            &self,
            a: usize,
            b: usize,
            k: Option<usize>,
            l: Option<usize>,
            r: Option<usize>,
        ) -> VALUE {
            let (k, l, r) = (k.unwrap_or(0), l.unwrap_or(0), r.unwrap_or(self.n));
            if r <= a || b <= l {
                INF
            } else if a <= l && r <= b {
                self.node[k]
            } else {
                let vl = get_min(a, b, 2 * k + 1, l, (l + r) / 2);
                let vr = get_min(a, b, 2 * k + 2, (l + r) / 2, r);
                std::cmp::min(vl, vr)
            }
        }
    }
}
