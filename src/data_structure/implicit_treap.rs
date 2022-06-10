//! # Implicit Treap
//! 配列のようなインターフェースを持つTreap

use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

pub struct ImplicitTreap<T> {
    randomizer: XorShift,
    root: Box<TreapNode<T>>,
}
impl<T> ImplicitTreap<T> {
    /// # サイズ
    ///
    /// ## 計算量
    /// $`O(1)`$
    pub fn len(&self) -> usize {
        self.root.len()
    }

    /// # 空かどうか
    ///
    /// ## 計算量
    /// $`O(1)`$
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: PartialOrd + Default> ImplicitTreap<T> {
    /// # 挿入
    /// 先頭からpos(0-indexed)の位置にxを挿入
    ///
    /// ## 計算量
    /// $`O(logN)`$
    pub fn insert(&mut self, pos: usize, x: T) {
        self.root
            .insert(pos, TreapNode::new(x, self.randomizer.next().unwrap()))
    }

    /// # 削除
    /// 先頭からpos(0-indexed)の位置の要素を削除して返す
    ///
    /// ## 計算量
    /// $`O(logN)`$
    pub fn erase(&mut self, pos: usize) -> Option<T> {
        self.root.erase(pos)
    }

    /// # 反転
    /// rangeの範囲を反転する
    pub fn reverse<R: RangeBounds<usize>>(&mut self, range: R) {
        let (l, r) = to_lr(&range, self.len());
        self.root.reverse(l, r);
    }

    /// # 回転
    /// rangeの範囲をtopが先頭に来るように回転する
    pub fn rotate<R: RangeBounds<usize>>(&mut self, range: R, top: usize) {
        let (l, r) = to_lr(&range, self.len());
        assert!(l <= top && top < r);
        self.root.rotate(l, r, top);
    }

    /// # 先頭の要素を取る
    /// $`O(logN)`$
    pub fn pop_front(&mut self) -> Option<T> {
        self.root.erase(0)
    }

    /// # 最後尾の要素を取る
    /// $`O(logN)`$
    pub fn pop_back(&mut self) -> Option<T> {
        self.root.erase(self.len() - 1)
    }

    /// # 配列に変換する
    /// $`O(NlogN)`$
    pub fn to_vec(mut self) -> Vec<T> {
        let mut v = Vec::new();
        while let Some(t) = self.pop_front() {
            v.push(t);
        }
        v
    }
}

impl<T> Default for ImplicitTreap<T> {
    fn default() -> Self {
        ImplicitTreap {
            randomizer: XorShift::default(),
            root: Box::new(TreapNode(None)),
        }
    }
}

impl<T> Index<usize> for ImplicitTreap<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        self.root.index(index)
    }
}

impl<T: Clone> Clone for ImplicitTreap<T> {
    fn clone(&self) -> Self {
        Self {
            randomizer: self.randomizer.clone(),
            root: self.root.clone(),
        }
    }
}

impl<T: Display> Display for ImplicitTreap<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}]", self.root)
    }
}

impl<T: Clone + PartialOrd + Default> From<&[T]> for ImplicitTreap<T> {
    fn from(src: &[T]) -> Self {
        let mut ret = Self::default();
        for t in src {
            ret.insert(ret.len(), t.clone())
        }
        ret
    }
}

#[derive(Default)]
pub struct TreapNode<T>(Option<Node<T>>);

#[derive(Clone, Default)]
struct Node<T> {
    /// キー
    key: T,
    /// 優先度
    p: u64,
    /// 部分木のサイズ
    size: usize,
    /// 左右反転フラグ
    rev: bool,
    /// 左の子
    l: Box<TreapNode<T>>,
    /// 右の子
    r: Box<TreapNode<T>>,
}

impl<T> TreapNode<T> {
    fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |node| node.size)
    }

    fn propagate_from_children(&mut self) {
        if let Some(node) = self.0.as_mut() {
            node.size = 1 + node.l.len() + node.r.len()
        }
    }

    fn propagate_to_children(&mut self) {
        if let Some(node) = self.0.as_mut() {
            if node.rev {
                node.rev = false;
                swap(&mut node.r, &mut node.l);
                if let Some(r_node) = node.r.0.as_mut() {
                    r_node.rev ^= true;
                }
                if let Some(l_node) = node.l.0.as_mut() {
                    l_node.rev ^= true;
                }
            }
        }
        self.propagate_from_children()
    }
}

impl<T: Default> TreapNode<T> {
    fn new(key: T, p: u64) -> Self {
        Self(Some(Node {
            key,
            p,
            ..Default::default()
        }))
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
        self.propagate_to_children();
        r.propagate_to_children();
        l.propagate_to_children();
        if let Some(ref mut node) = self.0 {
            let (mut l_temp, mut r_temp) = (Self::default(), Self::default());
            if pos < node.l.len() + 1 {
                // 左側の部分木を分割する 部分木の左側がl
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
        self.propagate_from_children();
        l.propagate_from_children();
        r.propagate_from_children();
    }

    // self の右に r をマージする
    fn merge(&mut self, r: &mut Self) {
        self.propagate_to_children();
        r.propagate_to_children();
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
        self.propagate_from_children();
        r.propagate_from_children();
    }

    pub fn reverse(&mut self, l: usize, r: usize) {
        // println!("{} {}", l, r);
        let (mut l_tree, mut c_tree, mut r_tree, mut temp) = (
            Self::default(),
            Self::default(),
            Self::default(),
            Self::default(),
        );
        self.split(r, &mut temp, &mut r_tree);
        temp.split(l, &mut l_tree, &mut c_tree);
        if let Some(node) = c_tree.0.as_mut() {
            node.rev ^= true;
        }
        self.merge(&mut l_tree);
        self.merge(&mut c_tree);
        self.merge(&mut r_tree);
    }

    pub fn rotate(&mut self, l: usize, r: usize, top: usize) {
        // println!("{} {} {} {}", l, r, top, l + r - top);
        self.reverse(l, r);
        self.reverse(l, l + r - top);
        self.reverse(l + r - top, r);
    }
}

impl<T> Index<usize> for TreapNode<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < self.len());
        self.0
            .as_ref()
            .map(|node| match () {
                () if node.l.len() > index => node.l.index(index),
                () if node.l.len() < index => node.r.index(index - node.l.len() - 1),
                _ => &node.key,
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
                rev: node.rev,
                l: node.l.clone(),
                r: node.r.clone(),
            })),
            _ => Self(None),
        }
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
    let mut treap = ImplicitTreap::<usize>::default();

    for i in 0..1000000 {
        treap.insert(i, i * 2);
    }
    assert_eq!(1000000, treap.len());
}

#[test]
fn test() {
    let mut treap = ImplicitTreap::from(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..]);

    let del = treap.erase(5);
    assert_eq!(Some(5), del);
    treap.erase(3);
    assert_eq!(vec![0, 1, 2, 4, 6, 7, 8, 9], treap.clone().to_vec());

    treap.reverse(2..6);
    assert_eq!(vec![0, 1, 7, 6, 4, 2, 8, 9], treap.clone().to_vec());

    let del2 = treap.erase(0);
    assert_eq!(Some(0), del2);
    assert_eq!(vec![1, 7, 6, 4, 2, 8, 9], treap.clone().to_vec());
    treap.rotate(2..6, 4);
    assert_eq!(vec![1, 7, 2, 8, 6, 4, 9], treap.to_vec());

    treap = ImplicitTreap::from(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8][..]);
    treap.rotate(2..7, 4);
    assert_eq!(vec![0, 1, 4, 5, 6, 2, 3, 7, 8], treap.to_vec());
}
