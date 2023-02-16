//! # Treap
//! 乱数を利用して平衡を保つ二分探索木 Tree + Heap
//!
//! ## dependency
//! [xor-shift](xor_shift::XorShift)

use prelude::*;
use xor_shift::XorShift;

#[snippet(name = "treap", doc_hidden)]
#[derive(Default, Clone, Debug)]
pub struct Treap<T> {
    randomizer: XorShift,
    root: Box<treap_impl::OptionalNode<T>>,
}

#[snippet(name = "treap", doc_hidden)]
mod treap_impl {
    use super::{swap, Debug, Display, Formatter, Ordering, Treap};
    impl<T> Treap<T> {
        /// # サイズ
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn len(&self) -> usize {
            self.root.len()
        }

        /// # 空かどうか
        ///
        /// ## 計算量
        /// $O(1)$
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<T: PartialOrd + Default> Treap<T> {
        /// # 挿入
        /// xを挿入
        ///
        /// ## 計算量
        /// $O(logN)$
        pub fn insert(&mut self, x: T) {
            self.root
                .insert(OptionalNode::new(x, self.randomizer.next().unwrap()))
        }

        /// # 削除
        /// keyを削除して返す
        ///
        /// ## 計算量
        /// $O(logN)$
        pub fn remove(&mut self, key: &T) -> Option<T> {
            self.root.erase(key)
        }

        /// # 検索
        /// keyが含まれるかどうかを返す
        ///
        /// ## 計算量
        /// $O(logN)$
        pub fn find(&self, key: &T) -> bool {
            self.root.find(key)
        }
    }

    impl<T: Display> Display for Treap<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "[{}]", self.root)
        }
    }

    impl<T: Clone + PartialOrd + Default + Debug> From<&[T]> for Treap<T> {
        fn from(src: &[T]) -> Self {
            let mut ret = Self::default();
            for t in src {
                ret.insert(t.clone())
            }
            ret
        }
    }

    #[derive(Debug)]
    pub struct OptionalNode<T>(Option<Node<T>>);

    #[derive(Clone, Debug, Default)]
    struct Node<T> {
        /// キー
        key: T,
        /// 優先度
        p: u64,
        /// 部分木のサイズ
        size: usize,
        /// 左の子
        l: Box<OptionalNode<T>>,
        /// 右の子
        r: Box<OptionalNode<T>>,
    }

    impl<T> OptionalNode<T> {
        pub fn new(key: T, p: u64) -> Self {
            Self(Some(Node {
                key,
                p,
                size: 1,
                l: Box::new(Self(None)),
                r: Box::new(Self(None)),
            }))
        }

        pub fn len(&self) -> usize {
            self.0.as_ref().map_or(0, |node| node.size)
        }

        fn propagate_from_children(&mut self) {
            if let Some(node) = self.0.as_mut() {
                node.size = 1 + node.l.len() + node.r.len()
            }
        }

        fn propagate_to_children(&mut self) {
            self.propagate_from_children()
        }
    }

    impl<T: PartialOrd> OptionalNode<T> {
        pub fn insert(&mut self, mut item: Self) {
            match (self.0.as_mut(), item.0.as_mut()) {
                (Some(tree_node), Some(item_node)) => {
                    if item_node.p > tree_node.p {
                        self.split(&item_node.key, &mut item_node.l, &mut item_node.r);
                        swap(self, &mut item);
                    } else if item_node.key < tree_node.key {
                        tree_node.l.insert(item)
                    } else {
                        tree_node.r.insert(item)
                    }
                }
                (None, Some(_)) => swap(&mut self.0, &mut item.0),
                (Some(_), None) => (),
                (None, None) => (),
            }
            self.propagate_from_children();
        }

        pub fn erase(&mut self, key: &T) -> Option<T> {
            if let Some(node) = self.0.as_mut() {
                match &node.key.partial_cmp(key) {
                    Some(Ordering::Equal) => {
                        let mut temp = Self::default();
                        temp.merge(&mut node.l);
                        temp.merge(&mut node.r);
                        swap(self, &mut temp);
                        temp.0.map(|node| node.key)
                    }
                    Some(Ordering::Greater) => node.l.erase(key),
                    Some(Ordering::Less) => node.r.erase(key),
                    _ => panic!("Ordering failed"),
                }
            } else {
                None
            }
        }

        /// selfを l: $[0, key)$ と r: $[key, n)$ に分割する
        fn split(&mut self, key: &T, l: &mut Self, r: &mut Self) {
            self.propagate_to_children();
            r.propagate_to_children();
            l.propagate_to_children();
            if let Some(ref mut node) = self.0 {
                let (mut l_temp, mut r_temp) = (Self::default(), Self::default());
                if key < &node.key {
                    // 左側の部分木を分割する 部分木の左側がl
                    node.l.split(key, &mut l_temp, &mut r_temp);
                    swap(l, &mut l_temp);
                    swap(&mut node.l, &mut Box::new(r_temp));
                    swap(r, self);
                } else {
                    // 右側の部分木を分割する
                    node.r.split(key, &mut l_temp, &mut r_temp);
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

        pub fn find(&self, key: &T) -> bool {
            if let Some(node) = &self.0 {
                match &node.key.partial_cmp(key) {
                    Some(Ordering::Equal) => true,
                    Some(Ordering::Greater) => node.l.find(key),
                    Some(Ordering::Less) => node.r.find(key),
                    _ => panic!("Ordering failed"),
                }
            } else {
                false
            }
        }
    }

    impl<T> Default for OptionalNode<T> {
        fn default() -> Self {
            Self(None)
        }
    }

    impl<T: Clone> Clone for OptionalNode<T> {
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

    impl<T: Display> Display for OptionalNode<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Some(node) => write!(f, "{} {} {}", node.l, node.key, node.r),
                _ => write!(f, ""),
            }
        }
    }
}
#[test]
fn size() {
    let mut treap = Treap::default();

    for i in 0..1000000 {
        treap.insert(i * 2);
    }
    assert_eq!(1000000, treap.len());
}

#[test]
fn test() {
    let mut treap = Treap::from(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9][..]);
    for i in 0..10 {
        assert!(treap.find(&i));
    }

    let del = treap.remove(&5);
    assert_eq!(Some(5), del);
    treap.remove(&3);
    for v in vec![0, 1, 2, 4, 6, 7, 8, 9] {
        assert!(treap.find(&v));
    }
}
