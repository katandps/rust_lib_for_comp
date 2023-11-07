//! # 全永続Stack
//! 時刻tを指定して、
//! - push
//!     - stackにValueを追加
//! - pop
//!     - stackの最後尾を削除
//! - top
//!     - stackの最後尾を取得(clone)
//! の操作を与える
//!
//! ## 時刻について
//! usizeで$0 = !0 = $空列
//! $!i$はiの逆列であることを表す
//!
use prelude::*;

#[snippet(name = "persistent-stack", doc_hidden)]
pub use persistent_stack_impl::PersistentStack;
#[snippet(name = "persistent-stack", doc_hidden)]
mod persistent_stack_impl {
    use super::Debug;
    #[derive(Clone, Debug)]
    pub struct PersistentStack<T> {
        /// index:=stackのid
        nodes: Vec<Node<T>>,
    }

    #[derive(Clone, Debug)]
    enum Node<T> {
        Simple(SimpleNode<T>),
        Empty,
        Reverse(usize),
        Combined(usize, usize),
    }

    #[derive(Clone, Debug)]
    struct SimpleNode<T> {
        value: T,
        size: usize,
        /// このノードの前に結合されたstackのid
        prev: usize,
    }

    impl<T> Default for PersistentStack<T> {
        fn default() -> Self {
            Self {
                nodes: vec![Node::Empty],
            }
        }
    }
    use Node::{Combined, Empty, Reverse, Simple};

    impl<T: Clone + Debug> PersistentStack<T> {
        /// 時刻$t$のstackが空かどうかを返す
        pub fn is_empty(&self, t: usize) -> bool {
            match self.nodes.get(t) {
                Some(Node::Empty) => true,
                Some(_) => false,
                _ => true,
            }
        }

        // reverseなノードを開く
        fn normalize(&mut self, node_id: usize) -> usize {
            let Some(Reverse(reversed)) = self.nodes.get(node_id).cloned() else {
                return node_id;
            };
            let (mut front, mut ret) = (reversed, 0);
            while let Some((v, t)) = self.pop(front) {
                ret = self.push(v, ret);
                front = t;
            }
            self.nodes[node_id] = self.nodes[ret].clone();
            ret
        }

        fn size(&self, node_id: usize) -> usize {
            self.nodes.get(node_id).map_or(0, |node| match node {
                Simple(node) => node.size,
                Empty => 0,
                Reverse(reversed) => self.size(*reversed),
                Combined(front, back) => self.size(*front) + self.size(*back),
            })
        }

        /// 時刻$t$の最後尾を返す
        pub fn top(&mut self, node_id: usize) -> Option<T> {
            self.nodes
                .get(node_id)
                .cloned()
                .and_then(|node| match node {
                    Empty => None,
                    Reverse(reversed) => {
                        let normalized = self.normalize(reversed);
                        self.top(normalized)
                    }
                    Simple(node) => Some(node.value),
                    Combined(_, back) => self.top(back),
                })
        }
        /// 時刻$t$の最後尾にvalueを追加する 追加したときの時刻を返す
        pub fn push(&mut self, value: T, node_id: usize) -> usize {
            self.nodes.push(Simple(SimpleNode {
                value,
                prev: node_id,
                size: self.size(node_id),
            }));
            self.nodes.len() - 1
        }

        /// 時刻$t$の最後尾の値を削除し、その値と削除したときの時刻を返す
        pub fn pop(&mut self, node_id: usize) -> Option<(T, usize)> {
            self.nodes
                .get(node_id)
                .cloned()
                .and_then(|node| match node {
                    Simple(node) => Some((node.value.clone(), node.prev)),
                    Empty => None,
                    Reverse(_) => {
                        let normalized = self.normalize(node_id);
                        self.pop(normalized)
                    }
                    Combined(front, back) => {
                        if let Some((v, back)) = self.pop(back) {
                            Some((v, self.combine(front, back)))
                        } else {
                            self.pop(front)
                        }
                    }
                })
        }
        /// 二つのstackを結合し、結合後の時刻を返す
        pub fn combine(&mut self, front_node_id: usize, back_node_id: usize) -> usize {
            if self.is_empty(front_node_id) {
                back_node_id
            } else if self.is_empty(back_node_id) {
                front_node_id
            } else {
                self.nodes.push(Node::Combined(front_node_id, back_node_id));
                self.nodes.len() - 1
            }
        }

        /// 時刻tのstackの内容からvecを生成して返す
        pub fn out_vec(&mut self, mut t: usize) -> Vec<T> {
            let mut ret = Vec::new();
            while let Some((v, t_)) = self.pop(t) {
                ret.push(v);
                t = t_;
            }
            ret.reverse();
            ret
        }

        /// 時刻tのstackのreverseを生成する
        pub fn reverse(&mut self, node_id: usize) -> usize {
            self.nodes
                .get(node_id)
                .cloned()
                .map(|node| match node {
                    Empty => node_id,
                    Reverse(reversed) => reversed,
                    Simple(_) => {
                        let reversed = Reverse(node_id);
                        self.nodes.push(reversed);
                        self.nodes.len() - 1
                    }
                    Combined(front, back) => {
                        let reverse_front = self.reverse(front);
                        let reverse_back = self.reverse(back);
                        let combined = Combined(reverse_back, reverse_front);
                        self.nodes.push(combined);
                        self.nodes.len() - 1
                    }
                })
                .unwrap()
        }
    }
}

#[test]
fn test() {
    let mut stack = PersistentStack::default();
    let t = stack.push(1, 0);
    assert_eq!(t, 1);
    assert_eq!(Some(1), stack.top(1));
    let t = stack.push(10, 1);
    assert_eq!(t, 2);
    assert_eq!(Some(10), stack.top(2));
    let t = stack.push(100, 1);
    assert_eq!(t, 3);
    assert_eq!(Some(100), stack.top(3));
    let (v, t) = stack.pop(3).unwrap();
    assert_eq!(v, 100);
    assert_eq!(t, 1);
    assert_eq!(Some(1), stack.top(1));
    let (v, t) = stack.pop(1).unwrap();
    assert_eq!(1, v);
    assert_eq!(t, 0);
    assert!(stack.is_empty(0));
    assert_eq!(None, stack.top(0));
}

#[test]
fn test_combine() {
    let mut stack = PersistentStack::default();
    let t1 = stack.push(1, 0);
    assert_eq!(t1, 1);
    let t2 = stack.push(2, t1);
    assert_eq!(t2, 2);
    let t3 = stack.push(3, 0);
    assert_eq!(t3, 3);
    let t4 = stack.push(4, t3);
    assert_eq!(t4, 4);
    assert_eq!(stack.out_vec(0), vec![]);
    assert_eq!(stack.out_vec(t1), vec![1]);
    assert_eq!(stack.out_vec(t2), vec![1, 2]);
    assert_eq!(stack.out_vec(t3), vec![3]);
    assert_eq!(stack.out_vec(t4), vec![3, 4]);
    let t5 = stack.combine(t1, t4);
    assert_eq!(stack.out_vec(t5), vec![1, 3, 4]);
}

#[test]
fn test_reverse() {
    let mut stack = PersistentStack::default();
    let t1 = stack.push(1, 0);
    let t2 = stack.push(2, t1);
    let t3 = stack.push(3, 0);
    let t4 = stack.push(4, t3);
    let t2r = stack.reverse(t2);
    assert_eq!(stack.out_vec(t2), vec![1, 2]);
    assert_eq!(stack.out_vec(t2r), vec![2, 1]);
    let t4r = stack.reverse(t4);
    assert_eq!(stack.out_vec(t4r), vec![4, 3]);
    let t5 = stack.combine(t2, t4);
    let t5r = stack.reverse(t5);
    assert_eq!(stack.out_vec(t5), vec![1, 2, 3, 4]);
    assert_eq!(stack.out_vec(t5r), vec![4, 3, 2, 1]);
    assert_eq!(stack.out_vec(t5r), vec![4, 3, 2, 1]);
}
