//! # GCD
//! ナイーブなユークリッドの互除法によるGCDの計算
use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Gcd<S>(PhantomData<fn() -> S>);
mod gcd_impl {
    use super::{
        swap, Associative, Commutative, Debug, Gcd, Idempotent, Magma, RemAssign, Unital, Zero,
    };
    impl<S: Clone + Debug + RemAssign + PartialOrd + Zero> Magma for Gcd<S> {
        type M = S;
        #[inline]
        fn op(x: &S, y: &S) -> S {
            let (mut x, mut y) = (x.clone(), y.clone());
            if y > x {
                swap(&mut x, &mut y);
            }
            while y != S::zero() {
                x %= y.clone();
                swap(&mut x, &mut y);
            }
            x
        }
    }
    impl<S: Clone + Debug + RemAssign + PartialOrd + Zero> Associative for Gcd<S> {}
    impl<S: Clone + Debug + RemAssign + PartialOrd + Zero> Unital for Gcd<S> {
        fn unit() -> S {
            S::zero()
        }
    }
    impl<S: Clone + Debug + RemAssign + PartialOrd + Zero> Commutative for Gcd<S> {}
    impl<S: Clone + Debug + RemAssign + PartialOrd + Zero> Idempotent for Gcd<S> {}
}
