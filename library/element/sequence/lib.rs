//! # 数列そのもの
//! segment treeのverifyなどに使う
use algebra::*;
use prelude::*;
use string_util::JoinTrait;

#[snippet(name = "sum_and_prefix_max", doc_hidden)]
pub use sequence_impl::Sequence;
#[snippet(name = "sum_and_prefix_max", doc_hidden)]
mod sequence_impl {
    use super::{Add, Debug, Display, JoinTrait, Zero};
    #[derive(Clone, Default)]
    pub struct Sequence<T>(pub Vec<T>);
    impl<T: Clone> Sequence<T> {
        pub fn new(v: T) -> Self {
            Self(vec![v])
        }
    }

    impl<T: PartialEq> PartialEq for Sequence<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T> Zero for Sequence<T> {
        fn zero() -> Self {
            Self(Vec::new())
        }
    }

    impl<T: Add<Output = T> + Clone + PartialOrd> Add for Sequence<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self(vec![self.0, rhs.0].into_iter().flatten().collect())
        }
    }

    impl<T: Display + Clone> Debug for Sequence<T> {
        fn fmt(&self, f: &mut prelude::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.0.iter().join(", "))
        }
    }
}

#[test]
fn test() {
    let a = Sequence::new(5);
    let b = Sequence::new(-3);
    let c = Sequence::new(5);
    let d = Sequence::new(-5);
    let s = a + b + c + d;
    assert_eq!(vec![5, -3, 5, -5], s.0);
}
