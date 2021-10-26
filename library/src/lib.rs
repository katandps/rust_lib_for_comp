//! 競技プログラミング向けライブラリ

pub mod algebra;
pub mod algo;
pub mod convolution;
pub mod data_structure;
pub mod flow;
pub mod geometry;
pub mod graph;
pub mod prelude;
pub mod string;
pub mod util;

pub use crate::algo::range::to_lr;
pub use prelude::*;
pub use util::min_max;
