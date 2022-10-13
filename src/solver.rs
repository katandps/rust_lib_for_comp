//! # Solver
//! メイン関数

use crate::prelude::*;
use crate::util::{reader::Reader, writer::Writer};

#[snippet(name = "template", doc_hidden)]
#[snippet(include = "reader")]
#[snippet(include = "writer")]
#[snippet(include = "algebra")]
#[snippet(include = "min_max")]
#[snippet(include = "range")]
#[snippet(include = "prelude")]
#[snippet(include = "debug")]
#[rustfmt::skip]
pub fn main() {
    let stdin = stdin();
    let stdout = stdout();
    std::thread::Builder::new()
        .name("extend stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || solve(Reader::new(|| stdin.lock()), Writer::new(stdout.lock())))
        .unwrap()
        .join()
        .unwrap()
}

#[snippet(name = "solver", doc_hidden)]
pub fn solve<R: BufRead, W: Write, F: FnMut() -> R>(mut reader: Reader<F>, mut writer: Writer<W>) {
    let n: usize = reader.v();
    writer.ln(n);
}
