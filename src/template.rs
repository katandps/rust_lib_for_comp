use crate::prelude::*;
use crate::util::{reader::Reader, writer::Writer};

#[snippet(name = "template", doc_hidden)]
pub fn main() {
    let stdin = stdin();
    let stdout = stdout();
    solve(Reader::new(stdin.lock()), Writer::new(stdout.lock()));
}

#[snippet(name = "template", doc_hidden)]
pub fn solve<R: BufRead, W: Write>(mut reader: Reader<R>, mut writer: Writer<W>) {
    let n = reader.val::<usize>();
    writer.println(&n);
}
