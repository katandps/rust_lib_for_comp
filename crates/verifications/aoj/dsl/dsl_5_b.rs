//! # The Maximum Number of Overlaps

use rust_lib_for_comp::algebra::binary_operation::addition::Addition;
use rust_lib_for_comp::data_structure::binary_indexed_tree_2d::BinaryIndexedTree2;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::{chmax, max};

#[verify::aizu_online_judge("DSL_5_B")]
pub fn dsl_5_b(read: impl std::io::Read, mut write: impl std::io::Write) {
    let mut reader = ReadHelper::new(read);
    let n = reader.v::<usize>();
    let lr = reader.vec4::<usize, usize, usize, usize>(n);
    let mut bit2d = BinaryIndexedTree2::<Addition<i64>>::new(1010, 1010);
    for (lx, ly, rx, ry) in lr {
        bit2d.add(lx + 1, ly + 1, 1);
        bit2d.add(lx + 1, ry + 1, -1);
        bit2d.add(rx + 1, ly + 1, -1);
        bit2d.add(rx + 1, ry + 1, 1);
    }
    let mut ans = 0;
    for i in 0..1010 {
        for j in 0..1010 {
            chmax!(ans, bit2d.sum(i, j));
        }
    }
    writeln!(write, "{ans}").ok();
    write.flush().ok();
}
