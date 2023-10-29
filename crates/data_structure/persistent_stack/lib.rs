//! # 全永続Stack
//! 時間0が空のstackになる
//! 時間tを指定して、
//! - push
//!     - stackにValueを追加
//! - pop
//!     - stackの最後尾を削除
//! - top
//!     - stackの最後尾を取得(clone)
//! の操作を与える
//!
//! ## 計算量
//! いずれの操作も $O(1)$

pub use persistent_stack_impl::PersistentStack;
mod persistent_stack_impl {
    #[derive(Clone, Debug)]
    pub struct PersistentStack<T> {
        nodes: Vec<Node<T>>,
        timeline: Vec<usize>,
    }

    #[derive(Clone, Debug)]
    struct Node<T> {
        value: T,
        prev: usize,
    }

    impl<T> Default for PersistentStack<T> {
        fn default() -> Self {
            Self {
                nodes: Vec::new(),
                timeline: vec![!0],
            }
        }
    }

    impl<T: Clone> PersistentStack<T> {
        /// 時刻$t$のstackが空かどうかを返す
        pub fn is_empty(&self, t: usize) -> bool {
            self.timeline[t] == !0
        }

        /// 時刻$t$の最後尾を返す
        pub fn top(&self, t: usize) -> T {
            self.nodes[self.timeline[t]].value.clone()
        }

        /// 時刻$t$の最後尾にvalueを追加する 追加したときの時刻を返す
        pub fn push(&mut self, value: T, t: usize) -> usize {
            self.nodes.push(Node {
                value,
                prev: self.timeline[t],
            });
            self.timeline.push(self.nodes.len() - 1);
            self.timeline.len() - 1
        }

        /// 時刻$t$の最後尾の値を削除し、その値と削除したときの時刻を返す
        pub fn pop(&mut self, t: usize) -> (T, usize) {
            let node = self.nodes[self.timeline[t]].clone();
            self.timeline.push(node.prev);
            (node.value, self.timeline.len() - 1)
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
    assert_eq!(t, 4);
    assert_eq!(1, stack.top(4));
    let (v, t) = stack.pop(4);
    assert_eq!(1, v);
    assert_eq!(t, 5);
    assert!(stack.is_empty(5));
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
