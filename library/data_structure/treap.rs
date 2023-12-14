//! # Treap
//! 乱数を利用して平衡を保つ二分探索木 Tree + Heap
//!
//! ## dependency
//! [xor-shift](xor_shift::XorShift)

use crate::algo::xor_shift::XorShift;
use crate::prelude::*;

#[codesnip::entry("treap")]
pub use treap_impl::{Treap, TreapSet};
#[codesnip::entry("treap", include("xor-shift", "prelude"))]
mod treap_impl {

    use super::{swap, Debug, Display, Formatter, FromIterator, Ordering, XorShift};

    #[derive(Default, Clone, Debug)]
    pub struct Treap<K, V> {
        randomizer: XorShift,
        root: Box<OptionalNode<K, V>>,
    }
    impl<K, V> Treap<K, V> {
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

    impl<K: PartialOrd + Default, V> Treap<K, V> {
        /// # 挿入
        /// xを挿入
        ///
        /// ## 計算量
        /// $O(logN)$
        pub fn insert(&mut self, k: K, v: V) {
            self.root
                .insert(OptionalNode::new(k, v, self.randomizer.next().unwrap()))
        }

        /// # 削除
        /// keyを削除して返す
        ///
        /// ## 計算量
        /// $O(logN)$
        pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
            self.root.erase(key)
        }

        /// # 検索
        /// keyが含まれるかどうかを返す
        ///
        /// ## 計算量
        /// $O(logN)$
        pub fn find(&self, key: &K) -> Option<&V> {
            self.root.find(key)
        }
    }

    impl<K: Display, V: Display> Display for Treap<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "[{}]", self.root)
        }
    }

    impl<K: Default + PartialOrd, V: Default> FromIterator<(K, V)> for Treap<K, V> {
        #[inline]
        fn from_iter<I: IntoIterator<Item = (K, V)>>(keys: I) -> Self {
            let mut ret = Self::default();
            for (k, v) in keys {
                ret.insert(k, v);
            }
            ret
        }
    }

    #[derive(Debug)]
    pub struct OptionalNode<K, V>(Option<Node<K, V>>);

    #[derive(Clone, Debug, Default)]
    struct Node<K, V> {
        /// キー
        key: K,
        /// 値
        value: V,
        /// 優先度
        p: u64,
        /// 部分木のサイズ
        size: usize,
        /// 左の子
        l: Box<OptionalNode<K, V>>,
        /// 右の子
        r: Box<OptionalNode<K, V>>,
    }

    impl<K, V> OptionalNode<K, V> {
        pub fn new(key: K, value: V, p: u64) -> Self {
            Self(Some(Node {
                key,
                value,
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

    impl<K: PartialOrd, V> OptionalNode<K, V> {
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

        pub fn erase(&mut self, key: &K) -> Option<(K, V)> {
            if let Some(node) = self.0.as_mut() {
                match &node.key.partial_cmp(key) {
                    Some(Ordering::Equal) => {
                        let mut temp = Self::default();
                        temp.merge(&mut node.l);
                        temp.merge(&mut node.r);
                        swap(self, &mut temp);
                        temp.0.map(|node| (node.key, node.value))
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
        fn split(&mut self, key: &K, l: &mut Self, r: &mut Self) {
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

        pub fn find(&self, key: &K) -> Option<&V> {
            if let Some(node) = &self.0 {
                match &node.key.partial_cmp(key) {
                    Some(Ordering::Equal) => Some(&node.value),
                    Some(Ordering::Greater) => node.l.find(key),
                    Some(Ordering::Less) => node.r.find(key),
                    _ => panic!("Ordering failed"),
                }
            } else {
                None
            }
        }
    }

    impl<K, V> Default for OptionalNode<K, V> {
        fn default() -> Self {
            Self(None)
        }
    }

    impl<K: Clone, V: Clone> Clone for OptionalNode<K, V> {
        fn clone(&self) -> Self {
            match &self.0 {
                Some(node) => Self(Some(Node {
                    key: node.key.clone(),
                    value: node.value.clone(),
                    p: node.p,
                    size: node.size,
                    l: node.l.clone(),
                    r: node.r.clone(),
                })),
                _ => Self(None),
            }
        }
    }

    impl<K: Display, V: Display> Display for OptionalNode<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Some(node) => write!(f, "{} {} {}", node.l, node.key, node.r),
                _ => write!(f, ""),
            }
        }
    }

    pub type TreapSet<K> = Treap<K, ()>;
    impl<K: Default + PartialOrd> FromIterator<K> for TreapSet<K> {
        #[inline]
        fn from_iter<I: IntoIterator<Item = K>>(keys: I) -> Self {
            let mut ret = Self::default();
            for k in keys {
                ret.insert(k, ());
            }
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Treap;
    #[test]
    fn size() {
        let mut treap = Treap::default();

        for i in 0..1000000 {
            treap.insert(i * 2, 1);
        }
        assert_eq!(1000000, treap.len());
    }

    #[test]
    fn test() {
        let mut treap = vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 8),
            (8, 9),
            (9, 10),
        ]
        .into_iter()
        .collect::<Treap<_, _>>();
        for i in 0..10 {
            assert_eq!(Some(&(i + 1)), treap.find(&i));
        }

        let del = treap.remove(&5);
        assert_eq!(Some((5, 6)), del);
        treap.remove(&3);
        for v in vec![0, 1, 2, 4, 6, 7, 8, 9] {
            assert_eq!(Some(&(v + 1)), treap.find(&v));
        }
    }
}
