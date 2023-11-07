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

pub use persistent_stack_impl::PersistentStack;
mod persistent_stack_impl {
    use super::Debug;
    #[derive(Clone, Debug)]
    pub struct PersistentStack<T> {
        /// index:=nodeのid
        nodes: Vec<Node<T>>,
        /// index:=stackのid
        timeline: Vec<usize>,
    }

    #[derive(Clone, Debug)]
    struct Node<T> {
        value: T,
        size: usize,
        /// このノードの前に結合されたstackのid
        prev: usize,
        // このノードの後ろに結合されたstackのid
        combined: usize,
    }

    impl<T> Default for PersistentStack<T> {
        fn default() -> Self {
            Self {
                nodes: Vec::new(),
                timeline: vec![!0],
            }
        }
    }

    impl<T: Clone + Debug> PersistentStack<T> {
        /// 時刻$t$のstackが空かどうかを返す
        pub fn is_empty(&self, t: usize) -> bool {
            t >= self.timeline.len()
                || self.timeline[t] == !0
                || self.nodes[self.timeline[t]].size == 0
        }

        /// 時刻$t$の最後尾を返す
        pub fn top(&self, mut t: usize) -> T {
            while self.nodes[self.timeline[t]].combined != !0 {
                t = self.nodes[self.timeline[t]].combined
            }
            self.nodes[self.timeline[t]].value.clone()
        }

        /// 時刻$t$の最後尾にvalueを追加する 追加したときの時刻を返す
        pub fn push(&mut self, value: T, t: usize) -> usize {
            let prev = self.timeline[t];
            self.nodes.push(Node {
                value,
                prev: t,
                size: if prev == !0 { 0 } else { self.nodes[prev].size } + 1,
                combined: !0,
            });
            self.timeline.push(self.nodes.len() - 1);
            self.timeline.len() - 1
        }

        /// 時刻$t$の最後尾の値を削除し、その値と削除したときの時刻を返す
        pub fn pop(&mut self, stack_id: usize) -> (T, usize) {
            let node_id = self.timeline[stack_id];
            let combined = self.nodes[node_id].combined;
            if self.is_empty(combined) {
                let node = &self.nodes[node_id];
                (node.value.clone(), node.prev)
            } else {
                let (v, back_t) = self.pop(combined);
                (v, self.combine(stack_id, back_t))
            }
        }
        /// 二つのstackを結合し、結合後の時刻を返す
        pub fn combine(&mut self, front_t: usize, back_t: usize) -> usize {
            let mut node = self.nodes[self.timeline[front_t]].clone();
            if node.combined != !0 {
                node.size -= self.nodes[self.timeline[node.combined]].size;
            }
            if self.is_empty(back_t) {
                node.combined = !0;
            } else {
                node.combined = back_t;
                node.size = node.size + self.nodes[self.timeline[back_t]].size;
            }
            self.nodes.push(node);
            self.timeline.push(self.nodes.len() - 1);
            self.timeline.len() - 1
        }

        /// 時刻tのstackの内容からvecを生成して返す
        pub fn out_vec(&mut self, mut t: usize) -> Vec<T> {
            let mut ret = Vec::new();
            while !self.is_empty(t) {
                let (v, t_) = self.pop(t);
                ret.push(v);
                t = t_;
            }
            ret.reverse();
            ret
        }
    }
}

#[test]
fn test() {
    let mut stack = PersistentStack::default();
    let t = stack.push(1, 0);
    assert_eq!(t, 1);
    assert_eq!(1, stack.top(1));
    let t = stack.push(10, 1);
    assert_eq!(t, 2);
    assert_eq!(10, stack.top(2));
    let t = stack.push(100, 1);
    assert_eq!(t, 3);
    assert_eq!(100, stack.top(3));
    let (v, t) = stack.pop(3);
    assert_eq!(v, 100);
    assert_eq!(t, 1);
    assert_eq!(1, stack.top(1));
    let (v, t) = stack.pop(1);
    assert_eq!(1, v);
    assert_eq!(t, 0);
    assert!(stack.is_empty(0));
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
#[should_panic]
fn panic_if_pop_from_empty_stack() {
    let mut stack = PersistentStack::<i32>::default();
    stack.pop(0);
}

#[test]
#[should_panic]
fn panic_if_top_from_empty_stack() {
    let stack = PersistentStack::<i32>::default();
    stack.top(0);
}
