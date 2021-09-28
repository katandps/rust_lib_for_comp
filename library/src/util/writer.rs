pub mod writer {
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

        pub fn print_join<S: Display>(&mut self, v: &[S], separator: Option<&str>) {
            let sep = separator.unwrap_or_else(|| "\n");
            writeln!(
                self.w,
                "{}",
                v.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(sep)
            )
            .unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::util::writer::writer::Writer;
    use std::io::stdout;

    #[test]
    fn t() {
        let stdout = stdout();
        let mut writer = Writer::new(stdout.lock());
        writer.println(&123);
        writer.print_join(&vec![123, 45, 678], None);
    }
}