//! # GCD $a \circ b \to gcd(a, b)$
//! 最小公倍数をユークリッドの互除法で求める
//!
//! ## 使い方
//!```
//! # use crate::binary_operation::greatest_common_divisor::*;
//! # use crate::algebra::*;
//! assert_eq!(1, Gcd::op(&3, &5));
//! assert_eq!(2, Gcd::op(&4, &6));
//! assert_eq!(3, Gcd::op(&3, &9));
//! assert_eq!(3, Gcd::op(&9, &3));
//! assert_eq!(11, Gcd::op(&11, &11));
//! assert_eq!(1, Gcd::op(&1_000_000_007, &998_244_353));
//! assert_eq!(100, Gcd::op(&100, &0));
//! assert_eq!(100, Gcd::op(&0, &100));
//! assert_eq!(100, Gcd::op(&Gcd::unit(), &100));
//! assert_eq!(100, Gcd::op(&100, &Gcd::unit()));
//!```
//!
//! ## dependency
//! prelude
//! algebra
use crate::algebra::*;
use crate::prelude::*;

pub mod naive_impl;

#[codesnip::entry("gcd-operation")]
pub use gcd_impl::Gcd;
#[codesnip::entry("gcd-operation")]
mod gcd_impl {
    use std::ops::{BitOr, Shl, ShrAssign, SubAssign};

    use super::{
        swap, Associative, Commutative, Debug, Idempotent, Magma, PhantomData, TrailingZeros,
        Unital, Zero,
    };

    #[derive(Clone, Debug, Default)]
    pub struct Gcd<S>(PhantomData<fn() -> S>);

    #[rustfmt::skip]
    pub trait GcdNeedTrait: Clone + Copy + Debug + PartialOrd + Zero + BitOr<Output = Self>
        + ShrAssign + Shl<Output = Self> + SubAssign + TrailingZeros{}
    #[rustfmt::skip]
    impl<S: Clone + Copy + Debug + PartialOrd + Zero + BitOr<Output = S>
        + ShrAssign + Shl<Output = S> + SubAssign + TrailingZeros> GcdNeedTrait for S{}

    impl<S: GcdNeedTrait> Magma for Gcd<S> {
        type M = S;
        #[inline]
        fn op(x: &S, y: &S) -> S {
            if x == &S::zero() {
                return *y;
            }
            if y == &S::zero() {
                return *x;
            }
            let (mut x, mut y) = (*x, *y);
            let s = (x | y).trailing_zero();
            x >>= x.trailing_zero();
            // do-while
            while {
                y >>= y.trailing_zero();
                if x > y {
                    swap(&mut x, &mut y);
                }
                y -= x;
                y > S::zero()
            } {}
            x << s
        }
    }
    impl<S: GcdNeedTrait> Associative for Gcd<S> {}
    impl<S: GcdNeedTrait> Unital for Gcd<S> {
        fn unit() -> S {
            S::zero()
        }
    }
    impl<S: GcdNeedTrait> Commutative for Gcd<S> {}
    impl<S: GcdNeedTrait> Idempotent for Gcd<S> {}
}
