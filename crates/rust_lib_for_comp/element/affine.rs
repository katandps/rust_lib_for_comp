//! # 線形写像 $ax + b$
use crate::algebra::*;
use crate::prelude::*;

#[codesnip::entry("affine")]
pub use affine_impl::{Affine, Composition};
#[codesnip::entry("affine", include("algebra", "prelude"))]
mod affine_impl {
    use super::{
        Add, Associative, Debug, Default, Magma, Mapping, Mul, One, PhantomData, Unital, Zero,
    };

    /// # 線形写像 $ax + b$
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
    impl<T: Clone> Affine<T> {
        pub fn apply<S: Mul<T, Output = S> + Add<T, Output = S>>(&self, x: S) -> S {
            x * self.a.clone() + self.b.clone()
        }
    }

    impl<M: Clone + Debug + Mul<Output = M> + Add<Output = M> + One + Zero> Affine<M> {
        fn op(x: &Self, y: &Self) -> Self {
            Affine {
                a: x.a.clone() * y.a.clone(),
                b: x.b.clone() * y.a.clone() + y.b.clone(),
            }
        }
    }

    /// # 関数の合成
    #[derive(Debug, Clone, Default)]
    pub struct Composition<T, M = T>(PhantomData<fn() -> (T, M)>);
    trait CanComposite:
        Clone + Debug + PartialEq + Mul<Output = Self> + Add<Output = Self> + One + Zero
    {
    }
    impl<T: Clone + Debug + PartialEq + Mul<Output = Self> + Add<Output = Self> + One + Zero>
        CanComposite for T
    {
    }

    impl<T: CanComposite, M> Magma for Composition<T, M> {
        type M = Affine<T>;
        fn op(x: &Affine<T>, y: &Affine<T>) -> Self::M {
            Affine::op(x, y)
        }
    }
    impl<T: CanComposite, M> Unital for Composition<T, M> {
        fn unit() -> Affine<T> {
            Affine {
                a: T::one(),
                b: T::zero(),
            }
        }
    }
    impl<T: CanComposite, M> Associative for Composition<T, M> {}
    impl<T: CanComposite, M: Clone + Debug + Mul<T, Output = M> + Add<T, Output = M>> Mapping
        for Composition<T, M>
    {
        type Mapping = Affine<T>;
        type Domain = M;
        type Codomain = M;
        fn apply(map: &Affine<T>, value: &M) -> M {
            map.apply(value.clone())
        }
    }
}

#[test]
fn test() {
    let a = Affine::new(3, 2);
    let one = Composition::<i32>::unit();
    assert_eq!(Composition::<i32>::op(&a, &one), a);
    let b = Affine::new(5, 5);
    assert_eq!(Composition::<i32>::op(&a, &b), Affine::new(15, 15));
}
