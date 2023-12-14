//! # 全永続Queue(Banker's queue)
//! 時間0が空のqueueになる
//! 時間tを指定して、
//! - push
//!     - queueにValueを追加
//! - pop
//!     - queueの先頭を削除
//! - top
//!     - queueの先頭を取得(clone)
//! の操作を与える
//!
//! ## 計算量
//! いずれの操作も ならし$O(1)$
use crate::data_structure::persistent_stack::PersistentStack;
use crate::prelude::*;

#[codesnip::entry("persistent-queue")]
pub use persistent_queue_impl::PersistentQueue;
#[codesnip::entry("persistent-queue", include("prelude", "persistent-stack"))]
mod persistent_queue_impl {
    use super::{Debug, PersistentStack};

    #[derive(Clone, Debug)]
    pub struct PersistentQueue<T> {
        stack: PersistentStack<T>,
        nodes: Vec<Node>,
    }

    #[derive(Clone, Debug)]
    struct Node {
        front_id: usize,
        rear_id: usize,
        front_size: usize,
        rear_size: usize,
    }

    impl<T> Default for PersistentQueue<T> {
        fn default() -> Self {
            Self {
                stack: PersistentStack::default(),
                nodes: vec![Node {
                    front_id: 0,
                    rear_id: 0,
                    front_size: 0,
                    rear_size: 0,
                }],
            }
        }
    }

    impl<T: Clone + Debug> PersistentQueue<T> {
        pub fn is_empty(&self, t: usize) -> bool {
            t >= self.nodes.len() || self.nodes[t].front_size == 0
        }

        /// nodeの最後尾にvalueを追加する 追加したときの時刻を返す
        pub fn push_rear(&mut self, value: T, node_id: usize) -> usize {
            let mut node = self.nodes[node_id].clone();
            node.rear_id = self.stack.push(value, node.rear_id);
            node.rear_size += 1;
            node = self.normalize(node);
            self.nodes.push(node);
            self.nodes.len() - 1
        }

        /// nodeの先頭の値を削除し、その値と削除したときの時刻を返す
        pub fn pop_front(&mut self, node_id: usize) -> Option<(T, usize)> {
            let mut node: Node = self.nodes[node_id].clone();
            if let Some((v, front_id)) = self.stack.pop(node.front_id) {
                node.front_id = front_id;
                node.front_size -= 1;
                node = self.normalize(node);
                self.nodes.push(node);
                Some((v, self.nodes.len() - 1))
            } else {
                None
            }
        }

        fn normalize(&mut self, node: Node) -> Node {
            if node.front_size >= node.rear_size {
                node
            } else {
                let rev = self.stack.reverse(node.rear_id);
                let combine = self.stack.combine(rev, node.front_id);
                Node {
                    front_size: node.front_size + node.rear_size,
                    front_id: combine,
                    rear_size: 0,
                    rear_id: 0,
                }
            }
        }

        pub fn out_vec(&mut self, mut node_id: usize) -> Vec<T> {
            let mut ret = Vec::new();
            while let Some((v, t)) = self.pop_front(node_id) {
                ret.push(v);
                node_id = t;
            }
            ret
        }
    }
}

#[test]
fn test() {
    let mut q = PersistentQueue::default();
    let t1 = q.push_rear(1, 0);
    let t2 = q.push_rear(2, t1);
    let t3 = q.push_rear(3, t2);
    assert_eq!(vec![1], q.out_vec(t1));
    assert_eq!(vec![1, 2], q.out_vec(t2));
    assert_eq!(vec![1, 2, 3], q.out_vec(t3));
}
