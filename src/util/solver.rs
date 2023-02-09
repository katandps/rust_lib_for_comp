//! # Solver
//! メイン関数

use crate::prelude::*;

pub fn main() {
    std::thread::Builder::new()
        .name("extend stack size".into())
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            let mut io = IO::default();
            solve(&mut io);
            io.flush();
        })
        .unwrap()
        .join()
        .unwrap()
}

#[snippet("solver")]

pub fn solve<IO: ReaderTrait + WriterTrait>(io: &mut IO) {
    let n: usize = io.v();
    io.out(n.ln());
}
