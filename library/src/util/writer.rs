//! # 出力ヘルパ
//! バッファリングされた出力を提供する
//! ## 使い方
//! std::io::Writeを実装している型を渡して初期化する
use crate::prelude::*;

pub struct Writer<W: Write> {
    writer: BufWriter<W>,
}
impl<W: Write> Writer<W> {
    pub fn new(write: W) -> Self {
        Self {
            writer: BufWriter::new(write),
        }
    }
    pub fn println<S: Display>(&mut self, s: S) {
        writeln!(self.writer, "{}", s).expect("Failed to write.")
    }
    pub fn print<S: Display>(&mut self, s: S) {
        write!(self.writer, "{}", s).expect("Failed to write.")
    }
    pub fn print_join<S: Display>(&mut self, v: &[S], separator: &str) {
        v.iter().fold("", |sep, arg| {
            write!(self.writer, "{}{}", sep, arg).expect("Failed to write.");
            separator
        });
        writeln!(self.writer).expect("Failed to write.");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut buf = Vec::new();
        {
            let mut writer = Writer::new(&mut buf);
            writer.print(123);
            writer.println(456);
            writer.print_join(&[1, 2, 3, 4, 5], " ");
        }
        assert_eq!(Ok("123456\n1 2 3 4 5\n"), from_utf8(&buf));
    }
}
