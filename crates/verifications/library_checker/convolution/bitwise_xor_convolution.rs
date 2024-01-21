use rust_lib_for_comp::convolution::bitwise_convolution::convolution;
use rust_lib_for_comp::util::io_util::*;
use rust_lib_for_comp::util::string_util::JoinTrait;
use rust_lib_for_comp::{
    algebra::mod_int::ModInt, convolution::bitwise_convolution::xor_convolution::XorConvolution,
};
use verify::{LibraryChecker, Solver};

#[derive(LibraryChecker)]
pub struct BitwiseXorConvolution;
impl verify::Solver for BitwiseXorConvolution {
    const PROBLEM_ID: &'static str = "bitwise_xor_convolution";
    const TIME_LIMIT_MILLIS: u64 = 5000;
    fn solve(read: impl std::io::Read, mut write: impl std::io::Write) {
        let mut reader = ReadHelper::new(read);
        let n = reader.v::<usize>();
        let a = reader.vec::<ModInt<998_244_353>>(1 << n);
        let b = reader.vec::<ModInt<998_244_353>>(1 << n);

        writeln!(
            write,
            "{}",
            &convolution::<XorConvolution, 998_244_353>(a, b).join(" ")
        )
        .ok();
        write.flush().ok();
    }
}
#[test]
fn test() {
    BitwiseXorConvolution::assert(
        "3
    1 2 3 4 5 6 7 8
    9 10 11 12 13 14 15 16",
        "492 488 476 472 428 424 412 408",
    );
}
