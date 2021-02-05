mod libraries;
mod solver;
mod template;

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    #[test]
    fn a_run() {
        use crate::solver::a::*;
        solve(Reader::new(BufReader::new(file("a"))));
    }

    #[test]
    fn b_run() {
        use crate::solver::b::*;
        solve(Reader::new(BufReader::new(file("b"))));
    }

    #[test]
    fn c_run() {
        use crate::solver::c::*;
        solve(Reader::new(BufReader::new(file("c"))));
    }

    #[test]
    fn d_run() {
        use crate::solver::d::*;
        solve(Reader::new(BufReader::new(file("d"))));
    }

    #[test]
    fn e_run() {
        use crate::solver::e::*;
        solve(Reader::new(BufReader::new(file("e"))));
    }

    #[test]
    fn f_run() {
        use crate::solver::f::*;
        solve(Reader::new(BufReader::new(file("f"))));
    }

    fn file(alphabet: &str) -> std::fs::File {
        let mut path = std::env::current_dir().unwrap();
        path.push("sample");
        path.push(format!("{}.txt", alphabet));
        std::fs::File::open(path).unwrap()
    }
}
