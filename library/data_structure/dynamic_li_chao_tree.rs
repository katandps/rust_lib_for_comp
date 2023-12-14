//! # Dynamic Li-Chao Tree (for CHT)
//! Convex-hull-trick問題を解く
//! 複数の直線のうち、各$x$座標について最小の$y$をオンラインで求める
//!
pub(crate) use crate::min_max_macro::{chmin, min};
use crate::prelude::*;
use crate::range_traits::ToBounds;

#[codesnip::entry("dynamic-li-chao-tree")]
pub use dynamic_li_chao_tree_impl::DynamicLiChaoTree;

#[codesnip::entry("dynamic-li-chao-tree", include("chmin", "range-traits"))]
mod dynamic_li_chao_tree_impl {
    use std::hint::unreachable_unchecked;

    use super::{chmin, min, swap, Debug, Ordering, ToBounds, VecDeque};

    const LEFT_LIMIT: i64 = -1_000_000_010;
    const RIGHT_LIMIT: i64 = 1_000_000_010;

    const EMPTY_NODE: usize = !0u32 as usize;

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
        pub const INF: i64 = std::i64::MAX;

        pub fn new(left_limit: i64, right_limit: i64) -> Self {
            Self {
                nodes: vec![Node::new(Line::default())],
                root: 0,
                left_limit,
                right_limit,
            }
        }

        /// # nodeに直線$ax+b$を追加
        /// ## 計算量
        /// $O(\log V)$
        pub fn add_line(&mut self, a: i64, b: i64) {
            let line = Line::new(a, b);
            self._add_line(self.root, line, self.left_limit, self.right_limit)
        }
        fn _add_line(&mut self, node_id: usize, mut line: Line, node_l: i64, node_r: i64) {
            let next_child_id = self.nodes.len();
            let node = &mut self.nodes[node_id];
            if node.line_is_above(&line, node_l, node_r) {
                // 区間内では追加する直線が最小値をとることはない
                return;
            }
            if node.replace_line(&mut line, node_l, node_r) {
                // この区間内では追加する直線が常に最小値をとる
                return;
            }
            if node_l + 1 == node_r {
                return;
            };
            let m = (node_l + node_r) / 2;

            // 直線を追加する区間を半分以下にする
            if line.eval(m) < node.eval(m) {
                swap(&mut node.line, &mut line);
            }
            // 傾きは単調減少にならなければならない
            match line.cmp(&node.line) {
                Ordering::Greater => {
                    let l_child = node.children[0] as usize;
                    if l_child != EMPTY_NODE {
                        self._add_line(l_child, line, node_l, m)
                    } else {
                        let l_child = next_child_id;
                        node.children[0] = l_child as u32;
                        self.nodes.push(Node::new(line.clone()));
                    }
                }
                Ordering::Less => {
                    let r_child = node.children[1] as usize;
                    if r_child != EMPTY_NODE {
                        self._add_line(r_child, line, m, node_r)
                    } else {
                        let r_child = next_child_id;
                        node.children[1] = r_child as u32;
                        self.nodes.push(Node::new(line.clone()))
                    }
                }
                // 上の分岐で傾きが同じものはすでに除かれるので、ここには来ない
                Ordering::Equal => unsafe { unreachable_unchecked() },
            }
        }

        /// # rangeに区間rの線分$ax+b$を追加
        /// ## 計算量
        /// $O(\log^2 V)$
        pub fn add_segment<R: ToBounds<i64> + Clone>(&mut self, range: R, a: i64, b: i64) {
            self._add_segment(
                range,
                self.root,
                Line::new(a, b),
                self.left_limit,
                self.right_limit,
            )
        }

        fn _add_segment<R: ToBounds<i64> + Clone>(
            &mut self,
            range: R,
            node_id: usize,
            line: Line,
            node_l: i64,
            node_r: i64,
        ) {
            if self.nodes[node_id].exclusive_range(&range, node_l, node_r) {
                return;
            }
            if self.nodes[node_id].contains_range(&range, node_l, node_r) {
                return self._add_line(node_id, line, node_l, node_r);
            }
            if node_l + 1 == node_r {
                return;
            };
            let m = (node_l + node_r) / 2;

            // 左右の子にも追加する
            let l_child = self.nodes[node_id].children[0] as usize;
            if l_child != EMPTY_NODE {
                self._add_segment(range.clone(), l_child, line.clone(), node_l, m);
            } else {
                let l_child = self.nodes.len();
                self.nodes[node_id].children[0] = l_child as u32;
                self.nodes.push(Node::new(Line::default()));
                self._add_segment(range.clone(), l_child, line.clone(), node_l, m);
            }
            let r_child = self.nodes[node_id].children[1] as usize;
            if r_child != EMPTY_NODE {
                self._add_segment(range, r_child, line, m, node_r);
            } else {
                let r_child = self.nodes.len();
                self.nodes[node_id].children[1] = r_child as u32;
                self.nodes.push(Node::new(Line::default()));
                self._add_segment(range, r_child, line, m, node_r);
            }
        }

        /// # $x$座標における$y$の最小値を取得
        pub fn query(&self, x: i64) -> i64 {
            self._query(0, x, self.left_limit, self.right_limit)
        }
        fn _query(&self, node_id: usize, x: i64, node_l: i64, node_r: i64) -> i64 {
            let node = &self.nodes[node_id];
            if node.exclusive_range(&(x..=x), node_l, node_r) {
                Self::INF
            } else {
                let m = (node_l + node_r) / 2;
                let mut ret = node.eval(x);
                let (l_child, r_child) = (node.children[0] as usize, node.children[1] as usize);
                if l_child != EMPTY_NODE {
                    chmin!(ret, self._query(l_child, x, node_l, m));
                }
                if r_child != EMPTY_NODE {
                    chmin!(ret, self._query(r_child, x, m, node_r));
                }
                ret
            }
        }
    }
    impl Debug for DynamicLiChaoTree {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut deq = VecDeque::new();
            deq.push_back((self.root, self.left_limit, self.right_limit));
            let mut v = Vec::new();
            while let Some((node, l, r)) = deq.pop_front() {
                let m = (l + r) / 2;
                v.push((l, r, self.nodes[node].line.clone()));
                let (l_node, r_node) = (self.nodes[node].children[0], self.nodes[node].children[1]);
                if l_node != !0 {
                    deq.push_back((l_node as usize, l, m));
                }
                if r_node != !0 {
                    deq.push_back((r_node as usize, m, r));
                }
            }
            v.sort_unstable();
            let mut buf = "\n".to_string();
            for (l, r, line) in v {
                if line == Line::default() {
                    buf.push_str(format!("{}..{} INF\n", l, r).as_str());
                } else {
                    buf.push_str(format!("{}..{} a: {} b: {}\n", l, r, line.a, line.b).as_str());
                }
            }
            write!(f, "{}", buf)
        }
    }

    #[derive(Clone, Debug)]
    struct Node {
        line: Line,
        // 子のノード番号
        children: [u32; 2],
    }

    impl Default for Node {
        #[inline]
        fn default() -> Self {
            Node {
                line: Line::default(),
                children: [EMPTY_NODE as u32; 2],
            }
        }
    }

    impl Node {
        #[inline]
        fn new(line: Line) -> Self {
            Node {
                line,
                children: [EMPTY_NODE as u32, EMPTY_NODE as u32],
            }
        }

        /// 直線でこのノードを完全に置き換える
        #[inline]
        fn replace_line(&mut self, line: &mut Line, node_l: i64, node_r: i64) -> bool {
            if self.line_is_below(line, node_l, node_r) {
                // 常に、追加する直線が既存の直線より下にある
                swap(&mut self.line, line);
                true
            } else {
                false
            }
        }

        /// # 常に、既存の直線が追加する直線より下にある
        #[inline]
        fn line_is_above(&self, line: &Line, node_l: i64, node_r: i64) -> bool {
            self.eval(node_l) <= line.eval(node_l) && self.eval(node_r) <= line.eval(node_r)
        }
        /// # 常に、追加する直線が既存の直線より下にある
        #[inline]
        fn line_is_below(&self, line: &Line, node_l: i64, node_r: i64) -> bool {
            self.eval(node_l) >= line.eval(node_l) && self.eval(node_r) >= line.eval(node_r)
        }
        /// # このノード全体が区間に含まれるか
        #[inline]
        fn contains_range<R: ToBounds<i64>>(&self, range: &R, node_l: i64, node_r: i64) -> bool {
            let (l, r) = range.lr();
            l <= node_l && node_r <= r
        }
        /// # このノードと区間が共通点をもたないか
        #[inline]
        fn exclusive_range<R: ToBounds<i64>>(&self, range: &R, node_l: i64, node_r: i64) -> bool {
            let (l, r) = range.lr();
            r <= node_l || node_r <= l
        }
        /// # 指定されたx座標でのyの値
        #[inline]
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
        #[inline]
        fn new(a: i64, b: i64) -> Self {
            Self { a, b }
        }
        #[inline]
        fn eval(&self, x: i64) -> i64 {
            self.a.saturating_mul(x).saturating_add(self.b)
        }
    }
    impl Default for Line {
        #[inline]
        fn default() -> Self {
            let (a, b) = (0, DynamicLiChaoTree::INF);
            Self { a, b }
        }
    }
    impl Ord for Line {
        #[inline]
        fn cmp(&self, other: &Self) -> Ordering {
            self.a.cmp(&other.a)
        }
    }
    impl PartialOrd for Line {
        #[inline]
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
                "\ni: {}, x: {}\nexpect:{}, actual:{}",
                i,
                j,
                t,
                cht.query(j),
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

#[test]
fn test_segment() {
    let mut cht = DynamicLiChaoTree::new(-10, 10);
    cht.add_segment(-5..5, 0, 0);
    assert_eq!(cht.query(-6), DynamicLiChaoTree::INF);
    assert_eq!(cht.query(-5), 0);
    assert_eq!(cht.query(-4), 0);
    assert_eq!(cht.query(-3), 0);
    assert_eq!(cht.query(-2), 0);
    assert_eq!(cht.query(-1), 0);
    assert_eq!(cht.query(0), 0);
    assert_eq!(cht.query(1), 0);
    assert_eq!(cht.query(2), 0);
    assert_eq!(cht.query(3), 0);
    assert_eq!(cht.query(4), 0);
    assert_eq!(cht.query(5), DynamicLiChaoTree::INF);
    cht.add_segment(-1..0, 12, -5);
    assert_eq!(
        "\n-10..-5 INF\n-10..0 INF\n-10..10 INF\n-5..-2 INF\n-5..0 a: 0 b: 0\n-2..-1 INF\n-2..0 INF\n-1..0 a: 12 b: -5\n0..5 a: 0 b: 0\n0..10 INF\n5..10 INF\n",
        format!("{:?}", cht)
    );
}
