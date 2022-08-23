//! # GCD $a \circ b \to gcd(a, b)$
//! 最小公倍数をユークリッドの互除法で求める
//!
//! ## 使い方
//!```
//! # use rust_lib_for_comp::algebra::binary_operation::greatest_common_divisor::*;
//! # use rust_lib_for_comp::algebra::Magma;
//! assert_eq!(1, Gcd::op(&3, &5));
//! assert_eq!(2, Gcd::op(&4, &6));
//! assert_eq!(3, Gcd::op(&3, &9));
//! assert_eq!(3, Gcd::op(&9, &3));
//! assert_eq!(11, Gcd::op(&11, &11));
//! assert_eq!(1, Gcd::op(&1_000_000_007, &998_244_353));
//!```
//!
//! ## dependency
//! [algebra](crate::algebra)
use crate::prelude::*;

#[snippet(name = "gcd-operation", doc_hidden)]
#[derive(Clone, Debug)]
pub struct Gcd<S>(Infallible, PhantomData<fn() -> S>);
#[snippet(name = "gcd-operation", doc_hidden)]
mod gcd_impl {
    use super::{
        swap, Associative, Commutative, Debug, Gcd, Idempotent, Magma, RemAssign, Unital, Zero,
    };
    impl<S: Clone + Debug + RemAssign + PartialOrd + Zero> Magma for Gcd<S> {
        type M = S;
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
