pub mod writer {
    use itertools::Itertools;
    use std::fmt::Display;
    use std::io::{BufWriter, Write};

    pub struct Writer<W: Write> {
        w: BufWriter<W>,
    }
    impl<W: Write> Writer<W> {
        pub fn new(writer: W) -> Writer<W> {
            Writer {
                w: BufWriter::new(writer),
            }
        }

        pub fn println<S: Display>(&mut self, s: &S) {
            writeln!(self.w, "{}", s).unwrap()
        }

        pub fn print<S: Display>(&mut self, s: &S) {
            write!(self.w, "{}", s).unwrap()
        }

        pub fn join_space<S: Display>(&mut self, v: &[S]) {
            writeln!(self.w, "{}", v.iter().join(" ")).unwrap()
        }

        pub fn join_newline<S: Display>(&mut self, v: &[S]) {
            writeln!(self.w, "{}", v.iter().join("\n")).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::util::writer::writer::Writer;
    use std::io::stdout;

    fn t() {
        let stdout = stdout();
        let mut writer = Writer::new(stdout.lock());
        writer.println(&123);
        writer.join_newline(&vec![123, 45, 678]);
    }
}
