use rust_lib_for_comp::{data_structure::wavelet_matrix::WaveletMatrix, util::io_util::*};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct RangeKthSmallest;
impl verify::Solver for RangeKthSmallest {
    const PROBLEM_ID: &'static str = "range_kth_smallest";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let wm = WaveletMatrix::from(reader.vec::<u64>(n));
        for _ in 0..q {
            let (l, r, k) = reader.v3::<usize, usize, usize>();
            let ans = wm.kth_smallest(l..r, k);
            writeln!(write, "{ans}").ok();
        }
    }
}
#[test]
fn test() {
    RangeKthSmallest::assert(
        "5 3
        1 4 0 1 3
        0 5 2
        1 3 1
        3 4 0",
        "1
        4
        1",
    );
}
