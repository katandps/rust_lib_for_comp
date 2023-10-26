//! # Dynamic Li-Chao Tree (for CHT)
//! Convex-hull-trick問題を解く
//! 複数の直線のうち、各$x$座標について最小の$y$をオンラインで求める
//!
use min_max_macro::{chmin, min};
use prelude::*;
use range_traits::ToBounds;

#[snippet(name = "dynamic-li-chao-tree", doc_hidden)]
pub use dynamic_li_chao_tree_impl::DynamicLiChaoTree;

#[snippet(name = "dynamic-li-chao-tree", doc_hidden)]
mod dynamic_li_chao_tree_impl {
    use std::hint::unreachable_unchecked;

    use super::{chmin, min, swap, Debug, Ordering, ToBounds, VecDeque};

    const LEFT_LIMIT: i64 = -1_000_000_010;
    const RIGHT_LIMIT: i64 = 1_000_000_010;

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
                nodes: vec![Node::new(Line::default(), left_limit, right_limit)],
                root: 0,
                left_limit,
                right_limit,
            }
        }

        /// # nodeに直線$ax+b$を追加
        /// ## 計算量
        /// $O(\log V)$
        pub fn add_line(&mut self, a: i64, b: i64) {
            let line = Line { a, b };
            self._add_line(self.root, line)
        }
        fn _add_line(&mut self, node_id: usize, mut line: Line) {
            let next_child_id = self.nodes.len();
            let node = &mut self.nodes[node_id];
            if node.line_is_above(&line) {
                // 常に、既存の直線が追加する直線より下にある
                return;
            }
            if node.line_is_below(&line) {
                // 常に、追加する直線が既存の直線より下にある
                node.line = line;
                return;
            }
            let (left, right) = (node.l, node.r);
            // 直線を追加する幅が1の時は子の処理をしない
            if left + 1 == right {
                return;
            }
            let m = (left + right) / 2;

            // 直線を追加する区間を半分以下にする
            if line.eval(m) < node.eval(m) {
                swap(&mut node.line, &mut line);
            }
            // 傾きは単調減少にならなければならない
            match line.cmp(&node.line) {
                Ordering::Greater => {
                    let l_child = node.children[0];
                    if l_child != !0 {
                        self._add_line(l_child, line)
                    } else {
                        let l_child = next_child_id;
                        node.children[0] = l_child;
                        self.nodes.push(Node::new(line.clone(), left, m));
                    }
                }
                Ordering::Less => {
                    let r_child = node.children[1];
                    if r_child != !0 {
                        self._add_line(r_child, line)
                    } else {
                        let r_child = next_child_id;
                        node.children[1] = r_child;
                        self.nodes.push(Node::new(line.clone(), m, right))
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
            let line = Line { a, b };
            self._add_segment(range, self.root, line)
        }

        fn _add_segment<R: ToBounds<i64> + Clone>(&mut self, range: R, node_id: usize, line: Line) {
            if self.nodes[node_id].exclusive_range(range.clone()) {
                return;
            }
            if self.nodes[node_id].contains_range(range.clone()) {
                return self._add_line(node_id, line);
            }

            let (left, right) = (self.nodes[node_id].l, self.nodes[node_id].r);
            if left + 1 == right {
                return;
            }
            let m = (left + right) / 2;

            // 左右の子にも追加する
            let l_child = self.nodes[node_id].children[0];
            if l_child != !0 {
                self._add_segment(range.clone(), l_child, line.clone());
            } else {
                let l_child = self.nodes.len();
                self.nodes[node_id].children[0] = l_child;
                self.nodes
                    .push(Node::new(Line::default(), self.nodes[node_id].l, m));
                self._add_segment(range.clone(), l_child, line.clone());
            }
            let r_child = self.nodes[node_id].children[1];
            if r_child != !0 {
                self._add_segment(range, r_child, line);
            } else {
                let r_child = self.nodes.len();
                self.nodes[node_id].children[1] = r_child;
                self.nodes
                    .push(Node::new(Line::default(), m, self.nodes[node_id].r));
                self._add_segment(range, r_child, line);
            }
        }

        /// # $x$座標における$y$の最小値を取得
        pub fn query(&self, x: i64) -> i64 {
            self._query(0, x)
        }
        fn _query(&self, node_id: usize, x: i64) -> i64 {
            let node = &self.nodes[node_id];
            if node.exclusive_range(x..=x) {
                Self::INF
            } else {
                let mut ret = node.eval(x);
                let (l_child, r_child) = (node.children[0], node.children[1]);
                if l_child != !0 {
                    chmin!(ret, self._query(l_child, x));
                }
                if r_child != !0 {
                    chmin!(ret, self._query(r_child, x));
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
                v.push((l, r, self.nodes[node].line.a, self.nodes[node].line.b));
                let (l_node, r_node) = (self.nodes[node].children[0], self.nodes[node].children[1]);
                if l_node != !0 {
                    deq.push_back((l_node, l, m));
                }
                if r_node != !0 {
                    deq.push_back((r_node, m, r));
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

    #[derive(Clone, Debug)]
    struct Node {
        line: Line,
        // ノードの左端
        l: i64,
        // ノードの右端(含む)
        r: i64,
        // 子のノード番号
        children: [usize; 2],
        // // 左の子のノード番号
        // l_child: Option<usize>,
        // // 右の子のノード番号
        // r_child: Option<usize>,
    }

    impl Default for Node {
        fn default() -> Self {
            Node {
                l: LEFT_LIMIT,
                r: RIGHT_LIMIT,
                line: Line::default(),
                children: [!0; 2], // l_child: None,
                                   // r_child: None,
            }
        }
    }

    impl Node {
        fn new(line: Line, l: i64, r: i64) -> Self {
            let children = [!0, !0];
            Self {
                line,
                l,
                r,
                children,
            }
        }
        /// # 常に、既存の直線が追加する直線より下にある
        fn line_is_above(&self, line: &Line) -> bool {
            self.left_value() <= line.eval(self.l) && self.right_value() <= line.eval(self.r)
        }
        /// # 常に、追加する直線が既存の直線より下にある
        fn line_is_below(&self, line: &Line) -> bool {
            line.eval(self.l) <= self.left_value() && line.eval(self.r) <= self.right_value()
        }
        /// # 左端の値
        fn left_value(&self) -> i64 {
            self.line.eval(self.l)
        }
        /// # 右端の値
        fn right_value(&self) -> i64 {
            self.line.eval(self.r)
        }
        /// # このノード全体が区間に含まれるか
        fn contains_range<R: ToBounds<i64>>(&self, range: R) -> bool {
            let (l, r) = range.lr();
            l <= self.l && self.r <= r
        }
        /// # このノードと区間が共通点をもたないか
        fn exclusive_range<R: ToBounds<i64>>(&self, range: R) -> bool {
            let (l, r) = range.lr();
            r <= self.l || self.r <= l
        }
        /// # 指定されたx座標でのyの値
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
            let (a, b) = (0, DynamicLiChaoTree::INF);
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
    use xor_shift::XorShift;
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
}
