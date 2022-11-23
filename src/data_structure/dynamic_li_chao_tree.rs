//! # Dynamic Li-Chao Tree (for CHT)
//! Convex-hull-trick問題を解く
//! 複数の直線のうち、各$x$座標について最小の$y$をオンラインで求める
//!
//! ## verify
//! [Line Add Get Min](https://judge.yosupo.jp/submission/108762)
use crate::prelude::*;

macro_rules! chmin {($base:expr, $($cmps:expr),+ $(,)*) => {{let cmp_min = min!($($cmps),+);if $base > cmp_min {$base = cmp_min;true} else {false}}};}
macro_rules! min {
    ($a:expr $(,)*) => {{$a}};
    ($a:expr, $b:expr $(,)*) => {{if $a > $b {$b} else {$a}}};
    ($a:expr, $($rest:expr),+ $(,)*) => {{let b = min!($($rest),+);if $a > b {b} else {$a}}};
}

#[snippet(name = "dynamic-li-chao-tree", doc_hidden)]
pub use dynamic_li_chao_tree_impl::DynamicLiChaoTree;

#[snippet(name = "dynamic-li-chao-tree", doc_hidden)]
mod dynamic_li_chao_tree_impl {
    use super::{swap, Debug, Ordering, VecDeque};

    const LEFT_LIMIT: i64 = -1_000_000_010;
    const RIGHT_LIMIT: i64 = 1_000_000_010;
    const INF: i64 = std::i64::MAX;

    #[derive(Clone)]
    pub struct DynamicLiChaoTree {
        nodes: Vec<Node>,
        root: usize,
        right_limit: i64,
        left_limit: i64,
    }

    impl Default for DynamicLiChaoTree {
        fn default() -> Self {
            Self {
                nodes: vec![Node::default()],
                root: 0,
                left_limit: LEFT_LIMIT,
                right_limit: RIGHT_LIMIT,
            }
        }
    }

    impl DynamicLiChaoTree {
        pub fn new(left_limit: i64, right_limit: i64) -> Self {
            Self {
                nodes: vec![Node::default()],
                root: 0,
                left_limit,
                right_limit,
            }
        }

        /// # 直線$ax+b$を追加
        pub fn add_line(&mut self, a: i64, b: i64) {
            let mut line = Line { a, b };
            let (mut node, mut left, mut right) = (self.root, self.left_limit, self.right_limit);
            loop {
                // 追加する区間における、追加する直線の両端の値
                let (line_l, line_r) = (line.eval(left), line.eval(right));
                // すでに追加されている直線の両端の値
                let (y_l, y_r) = (self.nodes[node].eval(left), self.nodes[node].eval(right));
                if y_l <= line_l && y_r <= line_r {
                    // 現在の区間に設定された直線が常に追加する直線以下である
                    return;
                } else if y_l >= line_l && y_r >= line_r {
                    // 現在の区間に設定された直線より追加する直線のほうが常に下にある
                    self.nodes[node].line = line.clone();
                    return;
                } else if left + 1 < right {
                    let m = (left + right) / 2;
                    if line.eval(m) < self.nodes[node].eval(m) {
                        swap(&mut self.nodes[node].line, &mut line);
                    }
                    // 傾きは単調減少にならなければならない
                    match line.cmp(&self.nodes[node].line) {
                        Ordering::Greater => {
                            if self.nodes[node].l.is_none() {
                                self.nodes[node].l = Some(self.nodes.len());
                                self.nodes.push(Node::new(line.clone()));
                                return;
                            }
                            node = self.nodes[node].l.unwrap();
                            right = m;
                        }
                        Ordering::Less => {
                            if self.nodes[node].r.is_none() {
                                self.nodes[node].r = Some(self.nodes.len());
                                self.nodes.push(Node::new(line.clone()));
                                return;
                            }
                            node = self.nodes[node].r.unwrap();
                            left = m;
                        }
                        // ここには落ちない
                        Ordering::Equal => unreachable!(),
                    }
                } else {
                    return;
                }
            }
        }
        /// # $x$座標における$y$の最小値を取得
        pub fn query(&self, x: i64) -> i64 {
            let mut node = self.root;
            let (mut left, mut right) = (self.left_limit, self.right_limit);
            let mut ret = INF;
            while left < right {
                chmin!(ret, self.nodes[node].eval(x));
                let m = (left + right) / 2;
                match (self.nodes[node].l, self.nodes[node].r) {
                    (Some(n), _) if x < m => {
                        node = n;
                        right = m;
                    }
                    (_, Some(n)) if x > m => {
                        node = n;
                        left = m;
                    }
                    _ => break,
                }
            }
            ret
        }
    }
    impl Debug for DynamicLiChaoTree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut deq = VecDeque::new();
            deq.push_back((self.root, self.left_limit, self.right_limit));
            let mut v = Vec::new();
            while let Some((node, l, r)) = deq.pop_front() {
                let m = (l + r) / 2;
                v.push((l, r, self.nodes[node].line.a, self.nodes[node].line.b));
                match (self.nodes[node].l, self.nodes[node].r) {
                    (Some(left), Some(right)) => {
                        deq.push_back((left, l, m));
                        deq.push_back((right, m, r));
                    }
                    (Some(left), _) => {
                        deq.push_back((left, l, m));
                    }
                    (_, Some(right)) => {
                        deq.push_back((right, m, r));
                    }
                    (_, _) => (),
                }
            }
            v.sort_unstable();
            let mut buf = "\n".to_string();
            for (l, r, a, b) in v {
                buf.push_str(format!("{}..{} a: {} b: {}\n", l, r, a, b).as_str());
            }
            writeln!(f, "{}", buf)
        }
    }

    #[derive(Clone, Debug, Default)]
    struct Node {
        line: Line,
        l: Option<usize>,
        r: Option<usize>,
    }

    impl Node {
        fn new(line: Line) -> Self {
            let (l, r) = (None, None);
            Self { line, l, r }
        }

        fn eval(&self, x: i64) -> i64 {
            self.line.eval(x)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Line {
        a: i64,
        b: i64,
    }
    impl Line {
        fn eval(&self, x: i64) -> i64 {
            self.a.saturating_mul(x).saturating_add(self.b)
        }
    }
    impl Default for Line {
        fn default() -> Self {
            let (a, b) = (0, INF);
            Self { a, b }
        }
    }
    impl Ord for Line {
        fn cmp(&self, other: &Self) -> Ordering {
            self.a.cmp(&other.a)
        }
    }
    impl PartialOrd for Line {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.a.cmp(&other.a))
        }
    }
}

#[test]
fn test() {
    let mut cht = DynamicLiChaoTree::new(-1000, 1000);
    assert_eq!(std::i64::MAX, cht.query(-100));
    assert_eq!(std::i64::MAX, cht.query(0));
    assert_eq!(std::i64::MAX, cht.query(100));
    let lines = vec![(1, 0), (-1, 10), (0, -10), (0, -20), (3, -150), (-2, 500)];
    for i in 0..lines.len() {
        let (a, b) = lines[i];
        cht.add_line(a, b);
        for j in -1000..1000 {
            let mut t = std::i64::MAX;
            for k in 0..=i {
                let (a, b) = lines[k];
                chmin!(t, a * j + b);
            }
            assert_eq!(
                t,
                cht.query(j),
                "\ni: {}, x: {}\nexpect:{}, actual:{}\ncht: {:?}",
                i,
                j,
                t,
                cht.query(j),
                cht
            );
        }
    }
}

#[test]
fn test_rand() {
    use crate::algo::xor_shift::XorShift;
    let mut xorshift = XorShift::default();
    let n = xorshift.rand_range(1..=50);
    let q = xorshift.rand_range(1..=50);
    const A_ABS_MAX: i64 = 1_000_000_000;
    const B_ABS_MAX: i64 = 1_000_000_000_000_000_000;
    let mut cht = DynamicLiChaoTree::default();
    let mut lines = Vec::new();
    for _ in 0..n {
        let (a, b) = (
            xorshift.rand_range(-A_ABS_MAX..=A_ABS_MAX),
            xorshift.rand_range(-B_ABS_MAX..=B_ABS_MAX),
        );
        lines.push((a, b));
        cht.add_line(a, b);
    }
    for _ in 0..q {
        let t = xorshift.rand_range(0..=1);
        if t == 0 {
            let (a, b) = (
                xorshift.rand_range(-A_ABS_MAX..=A_ABS_MAX),
                xorshift.rand_range(-B_ABS_MAX..=B_ABS_MAX),
            );
            lines.push((a, b));
            cht.add_line(a, b);
        } else {
            let x = xorshift.rand_range(-A_ABS_MAX..=A_ABS_MAX);
            let mut m = 1 << 60;
            for &(a, b) in &lines {
                chmin!(m, x * a + b);
            }
            assert_eq!(m, cht.query(x));
        }
    }
}
