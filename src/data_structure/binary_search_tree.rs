//! # 二分探索木
//! $ 左の子孫の値 \leq 親 \leq 右の子孫の値$ の制約を満たす木構造

use crate::prelude::*;

#[derive(Clone)]
pub struct BinaryNode<T>(Option<ExistNode<T>>);

#[derive(Clone)]
struct ExistNode<T> {
    key: T,
    l: Box<BinaryNode<T>>,
    r: Box<BinaryNode<T>>,
}

impl<T> Default for BinaryNode<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Ord + Copy> BinaryNode<T> {
    fn node(key: T) -> Self {
        Self(Some(ExistNode {
            key,
            l: Box::new(Self::default()),
            r: Box::new(Self::default()),
        }))
    }

    pub fn insert(&mut self, t: T) {
        match self.0.as_mut() {
            Some(ExistNode { key, l, r }) => match (*key).cmp(&t) {
                Ordering::Greater => match r.0 {
                    Some(ExistNode { key: _, l: _, r: _ }) => l.insert(t),
                    None => {
                        // 右が空かつ左の子がある状態にならないようにする
                        r.insert(*key);
                        *key = t;
                    }
                },
                Ordering::Less => r.insert(t),
                _ => (),
            },
            None => *self = Self::node(t),
        }
    }

    pub fn contains(&self, t: T) -> bool {
        match &self.0 {
            Some(ExistNode { key, l, r }) => match (key).cmp(&t) {
                Ordering::Greater => l.contains(t),
                Ordering::Less => r.contains(t),
                Ordering::Equal => true,
            },
            None => false,
        }
    }

    pub fn min(&self) -> Option<T> {
        match &self.0 {
            Some(ExistNode { key, l, r: _r }) => {
                if let Some(k) = l.min() {
                    Some(min(*key, k))
                } else {
                    Some(*key)
                }
            }
            None => None,
        }
    }

    pub fn max(&self) -> Option<T> {
        match &self.0 {
            Some(ExistNode { key, l: _l, r }) => {
                if let Some(k) = r.max() {
                    Some(max(*key, k))
                } else {
                    Some(*key)
                }
            }
            None => None,
        }
    }

    /// tより真に大きい次の要素を返す
    pub fn larger(&self, t: T) -> Option<T> {
        match &self.0 {
            Some(ExistNode { key, l, r }) => match key.cmp(&t) {
                Ordering::Greater => {
                    if let Some(k) = l.larger(t) {
                        Some(min(*key, k))
                    } else {
                        Some(*key)
                    }
                }
                Ordering::Less => r.larger(t),
                Ordering::Equal => r.larger(t),
            },
            None => None,
        }
    }

    /// tより真に小さい次の要素を返す
    pub fn smaller(&self, t: T) -> Option<T> {
        match &self.0 {
            Some(ExistNode { key, l, r }) => match key.cmp(&t) {
                Ordering::Greater => l.smaller(t),
                Ordering::Less => {
                    if let Some(k) = r.smaller(t) {
                        Some(max(*key, k))
                    } else {
                        Some(*key)
                    }
                }
                Ordering::Equal => l.smaller(t),
            },
            None => None,
        }
    }

    pub fn delete(&mut self, t: T) {
        if let Some(e) = self.0.as_mut() {
            match (e.key).cmp(&t) {
                Ordering::Greater => e.l.delete(t),
                Ordering::Less => e.r.delete(t),
                Ordering::Equal => {
                    if let Some(el) = &e.l.0 {
                        e.key = el.key;
                        e.l.delete(e.key);
                    } else if let Some(er) = &e.r.0 {
                        e.key = er.key;
                        e.r.delete(e.key)
                    } else {
                        *self = Self::default()
                    }
                }
            }
        }
    }

    pub fn get_top(&self) -> Option<T> {
        self.0.as_ref().map(|e| e.key)
    }
}

impl<T: Ord + Copy + Display> Debug for BinaryNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.get_top() {
            Some(t) => {
                let mut k = t;
                let mut v = vec![t];
                while let Some(l) = self.larger(k) {
                    v.push(l);
                    k = l;
                }
                let mut k = t;
                let mut vals = Vec::new();
                while let Some(s) = self.smaller(k) {
                    vals.push(s);
                    k = s;
                }
                vals.reverse();
                vals.append(&mut v);
                write!(f, "[").expect("Failed to write.");
                vals.iter().fold("", |sep, arg| {
                    write!(f, "{}{}", sep, arg).expect("Failed to write.");
                    " "
                });
                writeln!(f, "]")
            }
            None => writeln!(f, "[]"),
        }
    }
}

#[test]
fn test() {
    let mut tree = BinaryNode::default();
    let mut v = vec![5, 2, 7, 1, 3, 9];
    for i in &v {
        tree.insert(*i);
        // dbg!(&tree);
    }
    let mut ne = vec![1];
    for i in 0..5 {
        ne.push(tree.larger(ne[i]).unwrap());
    }
    v.sort();
    assert_eq!(v, ne);
    let mut be = vec![9];
    for i in 0..5 {
        be.push(tree.smaller(be[i]).unwrap());
    }
    v.reverse();
    assert_eq!(v, be);

    assert_eq!(Some(1), tree.min());
    assert_eq!(Some(9), tree.max());

    // dbg!(&tree);
    tree.delete(7);
    // dbg!(&tree);
}
