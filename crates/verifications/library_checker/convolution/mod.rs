pub mod bitwise_and_convolution;
pub mod bitwise_xor_convolution;

use rust_lib_for_comp::{
    algebra::mod_int::ModInt,
    convolution::fast_fourier_transform::FFT,
    util::{io_util::*, string_util::JoinTrait},
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct ConvolutionMod;
impl verify::Solver for ConvolutionMod {
    const PROBLEM_ID: &'static str = "convolution_mod";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let (n, m) = reader.v2::<usize, usize>();
        let a = reader.vec::<ModInt<998_244_353>>(n);
        let b = reader.vec::<ModInt<998_244_353>>(m);
        let fft = FFT::setup();
        let result = fft.convolution(a, b);
        writeln!(write, "{}", result.join(" ")).ok();
    }
}
#[test]
fn test() {
    ConvolutionMod::assert(
        "4 5
        1 2 3 4
        5 6 7 8 9",
        "5 16 34 60 70 70 59 36",
    );
    ConvolutionMod::assert(
        "1 1
        10000000
        10000000",
        "871938225",
    )
}
