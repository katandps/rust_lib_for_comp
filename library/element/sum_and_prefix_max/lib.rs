//! # 数列の合計と、そのprefix sumの最大値
use prelude::*;

#[snippet(name = "sum_and_prefix_max", doc_hidden)]
pub use sum_and_prefix_max_impl::SumAndMax;
#[snippet(name = "sum_and_prefix_max", doc_hidden)]
mod sum_and_prefix_max_impl {
    use super::{Add, Debug};
    #[derive(Clone, Copy)]
    pub struct SumAndMax<T> {
        pub sum: T,
        pub max: T,
    }
    impl<T: Clone> SumAndMax<T> {
        pub fn new(sum: T) -> Self {
            Self {
                max: sum.clone(),
                sum,
            }
        }
    }

    impl<T: PartialEq> PartialEq for SumAndMax<T> {
        fn eq(&self, other: &Self) -> bool {
            self.max.eq(&other.max)
        }
    }

    impl<T: Add<Output = T> + Clone + PartialOrd> Add for SumAndMax<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            let sum = self.sum.clone() + rhs.sum;
            let max = if sum > self.max {
                sum.clone()
            } else {
                self.max
            };
            SumAndMax { sum, max }
        }
    }

    impl<T: Debug> Debug for SumAndMax<T> {
        fn fmt(&self, f: &mut prelude::Formatter<'_>) -> std::fmt::Result {
            write!(f, "sum:{:?} max:{:?}", self.sum, self.max)
        }
    }
}

#[test]
fn test() {
    let a = SumAndMax::new(5);
    let b = SumAndMax::new(-3);
    let c = SumAndMax::new(5);
    let d = SumAndMax::new(-5);
    let s = a + b + c + d;
    assert_eq!((2, 7), (s.sum, s.max));
}
