//! 作用付きモノイド: 最大値＋最大値
use crate::algebra::impl_monoid::max::Max;
use crate::algebra::{MapMonoid, Monoid};

#[derive(Debug)]
pub struct MaxMax;
impl MapMonoid for MaxMax {
    type Mono = Max<i64>;
    type Func = i64;

    fn apply(f: &Self::Func, value: &<Self::Mono as Monoid>::M) -> <Self::Mono as Monoid>::M {
        std::cmp::max(*f, *value)
    }

    fn identity_map() -> Self::Func {
        <Self::Mono as Monoid>::unit()
    }

    fn compose(f: &Self::Func, g: &Self::Func) -> Self::Func {
        std::cmp::max(*f, *g)
    }
}
