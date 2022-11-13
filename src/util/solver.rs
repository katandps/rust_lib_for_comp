//! # Solver
//! メイン関数

use crate::prelude::*;

#[snippet(name = "template", doc_hidden)]
#[snippet(include = "reader")]
#[snippet(include = "writer")]
#[snippet(include = "algebra")]
#[snippet(include = "min_max")]
#[snippet(include = "range")]
#[snippet(include = "prelude")]
#[snippet(include = "debug")]
#[snippet(include = "faster-hashmap")]
#[snippet(include = "in-out")]
#[rustfmt::skip]
pub fn main() {
    std::thread::Builder::new()
        .name("extend stack size".into())
        .stack_size(32 * 1024 * 1024)
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
