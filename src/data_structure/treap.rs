//! # Treap
//! 乱数を利用して平衡を保つ二分探索木Tree + Heap

use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

pub struct Treap<T> {
    randomizer: XorShift,
    node: Box<TreapNode<T>>,
}
impl<T: PartialOrd + Default> Treap<T> {
    /// # サイズ
    ///
    /// ## 計算量
    /// $`O(1)`$
    pub fn len(&self) -> usize {
        self.node.len()
    }

    /// # 挿入
    /// 先頭からpos(0-indexed)の位置にxを挿入
    ///
    /// ## 計算量
    /// $`O(logN)`$
    pub fn insert(&mut self, pos: usize, x: T) {
        self.node
            .insert(pos, TreapNode::new(x, self.randomizer.next().unwrap()))
    }

    /// # 削除
    /// 先頭からpos(0-indexed)の位置の要素を削除して返す
    ///
    /// ## 計算量
    /// $`O(logN)`$
    pub fn erase(&mut self, pos: usize) -> Option<T> {
        self.node.erase(pos)
    }
}

impl<T> Default for Treap<T> {
    fn default() -> Self {
        Treap {
            randomizer: XorShift::default(),
            node: Box::new(TreapNode(None)),
        }
    }
}

impl<T> Index<usize> for Treap<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        self.node.index(index)
    }
}

impl<T: Clone> Clone for Treap<T> {
    fn clone(&self) -> Self {
        Self {
            randomizer: self.randomizer.clone(),
            node: self.node.clone(),
        }
    }
}

impl<T: Display> Display for Treap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}]", self.node)
    }
}

pub struct TreapNode<T>(Option<Node<T>>);

#[derive(Clone)]
struct Node<T> {
    /// キー
    key: T,
    /// 優先度
    p: u64,
    /// 部分木のサイズ
    size: usize,
    /// 左の子
    l: Box<TreapNode<T>>,
    /// 右の子
    r: Box<TreapNode<T>>,
}
impl<T> TreapNode<T> {
    fn new(key: T, p: u64) -> Self {
        Self(Some(Node {
            key,
            p,
            size: 1,
            l: Box::new(Self(None)),
            r: Box::new(Self(None)),
        }))
    }

    fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.size)
    }

    fn update(&mut self) {
        if let Some(node) = self.0.as_mut() {
            node.size = 1 + node.l.len() + node.r.len()
        }
    }
}

impl<T: PartialOrd + Default> TreapNode<T> {
    fn insert(&mut self, pos: usize, mut item: Self) {
        let (mut l, mut r) = (Self::default(), Self::default());
        self.split(pos, &mut l, &mut r);
        self.merge(&mut l);
        self.merge(&mut item);
        self.merge(&mut r);
    }

    fn erase(&mut self, pos: usize) -> Option<T> {
        let (mut l, mut r) = (Self::default(), Self::default());
        self.split(pos, &mut l, &mut r);
        let (mut res, mut r2) = (Self::default(), Self::default());
        r.split(1, &mut res, &mut r2);
        self.merge(&mut l);
        self.merge(&mut r2);
        res.0.map(|node| node.key)
    }

    /// selfを l: $`[0, pos)`$ と r: $`[pos, n)`$ に分割する
    fn split(&mut self, pos: usize, l: &mut Self, r: &mut Self) {
        self.update();
        if let Some(ref mut node) = self.0 {
            if pos < node.l.len() + 1 {
                // 左側の部分木を分割する 部分木の左側がl
                let (mut l_temp, mut r_temp) = (Self::default(), Self::default());
                node.l.split(pos, &mut l_temp, &mut r_temp);
                swap(l, &mut l_temp);
                swap(&mut node.l, &mut Box::new(r_temp));
                swap(r, self);
            } else {
                // 右側の部分木を分割する
                let (mut l_temp, mut r_temp) = (Self::default(), Self::default());
                node.r
                    .split(pos - node.l.len() - 1, &mut l_temp, &mut r_temp);
                swap(r, &mut r_temp);
                swap(&mut node.r, &mut Box::new(l_temp));
                swap(l, self);
            }
        } else {
            swap(l, &mut Self::default());
            swap(r, &mut Self::default());
        }
        self.update();
        l.update();
        r.update();
    }

    // self の右に r をマージする
    fn merge(&mut self, r: &mut Self) {
        // dbg!(&self, &r);
        self.update();
        r.update();
        match (self.0.as_mut(), r.0.as_mut()) {
            (Some(left_node), Some(right_node)) => {
                if left_node.p > right_node.p {
                    // 左の根のほうが優先度が大きいとき、左の木の右の子と右の木をマージする
                    left_node.r.merge(r);
                } else {
                    let mut temp = Self::default();
                    swap(&mut temp, &mut right_node.l);
                    self.merge(&mut temp);
                    swap(self, &mut right_node.l);
                    swap(self, r);
                }
            }
            (Some(_), None) => (),
            (None, Some(_)) => swap(self, r),
            _ => (),
        }
        self.update();
        r.update();
    }
}

impl<T> Index<usize> for TreapNode<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < self.len());
        self.0
            .as_ref()
            .map(|node| {
                if node.l.len() > index {
                    node.l.index(index)
                } else if node.l.len() == index {
                    &node.key
                } else {
                    node.r.index(index - node.l.len() - 1)
                }
            })
            .unwrap()
    }
}

impl<T: Clone> Clone for TreapNode<T> {
    fn clone(&self) -> Self {
        match &self.0 {
            Some(node) => Self(Some(Node {
                key: node.key.clone(),
                p: node.p,
                size: node.size,
                l: node.l.clone(),
                r: node.r.clone(),
            })),
            _ => Self(None),
        }
    }
}

impl<T: Default> Default for TreapNode<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Display> Display for TreapNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(node) => write!(f, "{} {} {}", node.l, node.key, node.r),
            _ => write!(f, ""),
        }
    }
}

impl<T: Display> Debug for TreapNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(node) => write!(
                f,
                "key{}size{}:[l:{:?} r:{:?}]",
                node.key, node.size, node.l, node.r
            ),
            _ => write!(f, ""),
        }
    }
}

#[test]
fn size() {
    let mut treap = Treap::<usize>::default();

    for i in 0..1000000 {
        treap.insert(i, i * 2);
    }
    assert_eq!(1000000, treap.len());
}

#[test]
fn test() {
    let mut treap = Treap::<usize>::default();
    for i in 0..10 {
        treap.insert(i, i);
    }
    let del = treap.erase(5);
    assert_eq!(Some(5), del);
    treap.erase(3);

    let mut v = Vec::new();
    for i in 0..treap.len() {
        v.push(treap[i]);
    }
    assert_eq!(vec![0, 1, 2, 4, 6, 7, 8, 9], v);
}
