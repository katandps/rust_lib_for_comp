//! # 線形写像 $ax + b$
use algebra::*;
use prelude::*;

pub use affine_impl::{Affine, Composition};
mod affine_impl {
    use super::{
        Add, Associative, Debug, Default, Magma, MonoidOperation, Mul, One, PhantomData, Unital,
        Zero,
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Affine<T> {
        a: T,
        b: T,
    }

    impl<T> Affine<T> {
        pub fn new(a: T, b: T) -> Self {
            Affine { a, b }
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

    #[derive(Debug, Clone)]
    pub struct Composition<T>(fn() -> PhantomData<T>);

    impl<T: Clone + PartialEq + Debug + Mul<Output = T> + Add<Output = T>> Magma for Composition<T> {
        type M = Affine<T>;
        fn op(x: &Self::M, y: &Self::M) -> Self::M {
            Affine {
                a: x.a.clone() * y.a.clone(),
                b: x.a.clone() * y.b.clone() + x.b.clone(),
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
            value.clone() * f.a.clone() + f.b.clone()
        }
    }
}

#[test]
fn test() {
    let a = Affine::new(3, 2);
    let one = Affine::one();
    assert_eq!(Composition::op(&a, &one), a);
    let b = Affine::new(5, 5);
    assert_eq!(Composition::op(&a, &b), Affine::new(15, 17));
}
