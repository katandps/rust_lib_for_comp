//! # 区間加算区間和
//!
//! 遅延セグメント木に適用すると表題のクエリを使用できる

use crate::algebra::binary_operation::addition::*;
use crate::algebra::mapping::add_mapping::*;
use crate::algebra::*;
use crate::element::section::Section;

#[derive(Clone, Debug, Default)]
pub struct AddSum {
    map: AddMapping<i64, Section<i64>, Section<i64>>,
    mono: Addition<Section<i64>>,
}
impl MapMonoid for AddSum {
    type Mono = Addition<Section<i64>>;
    type Map = AddMapping<i64, Section<i64>, Section<i64>>;
    fn map(&mut self) -> &mut Self::Map {
        &mut self.map
    }
    fn monoid(&mut self) -> &mut Self::Mono {
        &mut self.mono
    }
}
