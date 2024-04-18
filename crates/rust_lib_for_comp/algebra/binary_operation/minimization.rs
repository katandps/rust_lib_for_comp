//! # 最小化 $a \circ b \to min(a, b)$
//! $a > b$ のとき b
use crate::algebra::*;

#[codesnip::entry("minimization", include("algebra"))]
pub use minimization_impl::Minimization;
#[codesnip::entry("minimization", include("algebra"))]
mod minimization_impl {
    use super::{
        Associative, BoundedAbove, Commutative, Debug, Default, Idempotent, Magma, Mapping,
        PhantomData, Unital,
    };

    #[derive(Clone, Debug, Default)]
    pub struct Minimization<S>(PhantomData<fn() -> S>);
    impl<S: Clone + Debug + PartialOrd> Magma for Minimization<S> {
        type M = S;
        fn op(&mut self, x: &Self::M, y: &Self::M) -> Self::M {
            if x <= y {
                x.clone()
            } else {
                y.clone()
            }
        }
    }
    impl<S: BoundedAbove + Debug + Clone + PartialOrd> Unital for Minimization<S> {
        fn unit() -> Self::M {
            S::max_value()
        }
    }
    impl<S: Clone + Debug + PartialOrd> Associative for Minimization<S> {}
    impl<S: Clone + Debug + PartialOrd> Commutative for Minimization<S> {}
    impl<S: Clone + Debug + PartialOrd> Idempotent for Minimization<S> {}
    impl<S: Clone + Debug + PartialOrd> Mapping for Minimization<S> {
        type Mapping = S;
        type Domain = S;
        type Codomain = S;
        fn apply(&mut self, map: &Self::Mapping, a: &Self::Domain) -> Self::Codomain {
            self.op(map, a)
        }
    }
}
