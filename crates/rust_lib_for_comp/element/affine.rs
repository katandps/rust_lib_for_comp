//! # 線形写像 $ax + b$
use crate::algebra::*;
use crate::prelude::*;

#[codesnip::entry("affine")]
pub use affine_impl::{Affine, Composition};
#[codesnip::entry("affine", include("algebra", "prelude"))]
mod affine_impl {
    use super::{
        Add, Associative, Debug, Default, Magma, MonoidOperation, Mul, One, PhantomData, Unital,
        Zero,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Affine<T> {
        pub a: T,
        pub b: T,
    }

    impl<T> Affine<T> {
        pub fn new(a: T, b: T) -> Self {
            Affine { a, b }
        }
    }
    impl<T: Clone + Mul<Output = T> + Add<Output = T>> Affine<T> {
        pub fn apply(&self, x: T) -> T {
            self.a.clone() * x + self.b.clone()
        }
    }

    impl<T: Zero + One> Default for Affine<T> {
        fn default() -> Self {
            Affine {
                a: T::one(),
                b: T::zero(),
            }
        }
    }
    impl<T: One + Zero> One for Affine<T> {
        fn one() -> Self {
            Affine {
                a: T::one(),
                b: T::zero(),
            }
        }
    }

    impl<T: Zero> Zero for Affine<T> {
        fn zero() -> Self {
            Affine {
                a: T::zero(),
                b: T::zero(),
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct Composition<T>(PhantomData<T>);

    impl<T: Clone + PartialEq + Debug + Mul<Output = T> + Add<Output = T>> Magma for Composition<T> {
        type M = Affine<T>;
        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            Affine {
                a: x.a.clone() * y.a.clone(),
                b: x.b.clone() * y.a.clone() + y.b.clone(),
            }
        }
    }
    impl<T: Zero + One + Clone + PartialEq + Debug + Mul<Output = T> + Add<Output = T>> Unital
        for Composition<T>
    {
        fn unit() -> Self::M {
            Affine {
                a: T::one(),
                b: T::zero(),
            }
        }
    }
    impl<T> Associative for Composition<T> {}
    impl<T: Zero + One + Clone + PartialEq + Debug + Mul<Output = T> + Add<Output = T>>
        MonoidOperation for Composition<T>
    {
        type V = T;
        fn apply(&self, f: &Self::M, value: &Self::V) -> Self::V {
            f.apply(value.clone())
        }
    }
}

#[test]
fn test() {
    let a = Affine::new(3, 2);
    let one = Affine::one();
    assert_eq!(Composition::op(&a, &one), a);
    let b = Affine::new(5, 5);
    assert_eq!(Composition::op(&a, &b), Affine::new(15, 15));
}
