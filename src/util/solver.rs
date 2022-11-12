//! # Solver
//! メイン関数

use crate::prelude::*;
use crate::util::writer::Writer;

#[snippet(name = "template", doc_hidden)]
#[snippet(include = "reader")]
#[snippet(include = "writer")]
#[snippet(include = "algebra")]
#[snippet(include = "min_max")]
#[snippet(include = "range")]
#[snippet(include = "prelude")]
#[snippet(include = "debug")]
#[snippet(include = "faster-hashmap")]
#[rustfmt::skip]
pub fn main() {
    let stdout = stdout();
    std::thread::Builder::new()
        .name("extend stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || solve(ReaderFromStdin::default(), Writer::new(stdout.lock())))
        .unwrap()
        .join()
        .unwrap()
}

#[snippet("solver")]
pub fn solve<R: ReaderTrait, W: Write>(mut reader: R, mut writer: Writer<W>) {
    let n: usize = reader.v();
    writer.ln(n);
}
