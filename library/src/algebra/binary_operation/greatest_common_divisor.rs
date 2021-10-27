//! $`a \circ b \to gcd(a, b)`$
//! 最小公倍数をユークリッドの互除法で求める
use crate::algebra::{Associative, Commutative, Idempotent, Magma, Unital, Zero};
use crate::*;

#[derive(Clone, Debug)]
pub struct Gcd<S>(Infallible, PhantomData<fn() -> S>);
impl<S: Copy + RemAssign + Ord + Debug + Zero> Magma for Gcd<S> {
    type M = S;
    fn op(x: &S, y: &S) -> S {
        let (mut x, mut y) = (*max(x, y), *min(x, y));
        while y != S::zero() {
            x %= y;
            swap(&mut x, &mut y);
        }
        x
    }
}
impl<S: Copy + RemAssign + Ord + Debug + Zero> Associative for Gcd<S> {}
impl<S: Copy + RemAssign + Ord + Debug + Zero> Unital for Gcd<S> {
    fn unit() -> S {
        S::zero()
    }
}
impl<S: Copy + RemAssign + Ord + Debug + Zero> Commutative for Gcd<S> {}
impl<S: Copy + RemAssign + Ord + Debug + Zero> Idempotent for Gcd<S> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, Gcd::op(&3, &5));
        assert_eq!(2, Gcd::op(&4, &6));
        assert_eq!(3, Gcd::op(&3, &9));
        assert_eq!(3, Gcd::op(&9, &3));
        assert_eq!(11, Gcd::op(&11, &11));
        assert_eq!(1, Gcd::op(&1_000_000_007, &998_244_353));
    }
}
