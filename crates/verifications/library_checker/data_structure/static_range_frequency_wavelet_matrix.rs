use rust_lib_for_comp::{data_structure::wavelet_matrix::WaveletMatrix, util::io_util::*};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct StaticRangeFrequencyWaveletMatrix;
impl verify::Solver for StaticRangeFrequencyWaveletMatrix {
    const PROBLEM_ID: &'static str = "static_range_frequency";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, q) = reader.v2::<usize, usize>();
        let wm = WaveletMatrix::from(reader.vec::<u64>(n));
        for _ in 0..q {
            let (l, r, x) = reader.v3::<usize, usize, u64>();
            let ans = wm.rank_section(l..r, x);
            writeln!(write, "{ans}").ok();
        }
    }
}
#[test]
fn test() {
    StaticRangeFrequencyWaveletMatrix::assert(
        "5 3
        3 7 1 2 1
        1 5 1
        3 3 0
        0 4 3",
        "2
        0
        1",
    );
}
