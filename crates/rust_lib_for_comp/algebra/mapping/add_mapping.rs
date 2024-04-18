use crate::algebra::*;
use crate::prelude::*;

pub use add_mapping_impl::AddMapping;

mod add_mapping_impl {
    use super::{Add, Associative, Debug, Magma, Mapping, PhantomData, Unital, Zero};
    type Phantom<Map, Domain, Codomain> = PhantomData<fn() -> (Map, Domain, Codomain)>;
    #[derive(Clone, Debug, Default)]
    pub struct AddMapping<Map, Domain, Codomain>(Phantom<Map, Domain, Codomain>);
    impl<
            Map: Debug + Clone,
            Domain: Debug + Clone + Add<Map, Output = Codomain>,
            Codomain: Debug + Clone,
        > Mapping for AddMapping<Map, Domain, Codomain>
    {
        type Mapping = Map;
        type Domain = Domain;
        type Codomain = Codomain;
        fn apply(&mut self, map: &Self::Mapping, a: &Self::Domain) -> Self::Codomain {
            a.clone() + map.clone()
        }
    }

    impl<M: Debug + Clone + Add<M, Output = M> + PartialEq, D, C> Magma for AddMapping<M, D, C> {
        type M = M;
        fn op(&mut self, x: &M, y: &M) -> M {
            x.clone() + y.clone()
        }
    }

    impl<M: Debug + Clone + Add<M, Output = M> + PartialEq, D, C> Associative for AddMapping<M, D, C> {}

    impl<M: Debug + Clone + Add<M, Output = M> + PartialEq + Zero, D, C> Unital
        for AddMapping<M, D, C>
    {
        fn unit() -> Self::M {
            M::zero()
        }
    }
}
