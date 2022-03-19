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
pub fn main() {
    let stdin = stdin();
    let stdout = stdout();
    solve(Reader::new(|| stdin.lock()), Writer::new(stdout.lock()));
}

#[snippet(name = "solver", doc_hidden)]
pub fn solve<R: BufRead, W: Write, F: FnMut() -> R>(mut reader: Reader<F>, mut writer: Writer<W>) {
    let n: usize = reader.v();
    writer.ln(n);
}
