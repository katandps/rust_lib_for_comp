//! # GCD
//! ナイーブなユークリッドの互除法によるGCDの計算
use algebra::*;
use prelude::*;

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

#[test]
fn test() {
    assert_eq!(1, Gcd::op(&3, &5));
    assert_eq!(2, Gcd::op(&4, &6));
    assert_eq!(3, Gcd::op(&3, &9));
    assert_eq!(3, Gcd::op(&9, &3));
    assert_eq!(11, Gcd::op(&11, &11));
    assert_eq!(1, Gcd::op(&1_000_000_007, &998_244_353));
    assert_eq!(100, Gcd::op(&100, &0));
    assert_eq!(100, Gcd::op(&0, &100));
}
