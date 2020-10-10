#[cfg(test)]
mod tests {
    #[test]
    fn a_run() {
        a::solve(a::Reader::new(std::io::BufReader::new(file("a"))));
    }

    #[test]
    fn b_run() {
        b::solve(b::Reader::new(std::io::BufReader::new(file("b"))));
    }

    #[test]
    fn c_run() {
        c::solve(c::Reader::new(std::io::BufReader::new(file("c"))));
    }

    #[test]
    fn d_run() {
        d::solve(d::Reader::new(std::io::BufReader::new(file("d"))));
    }

    #[test]
    fn e_run() {
        e::solve(e::Reader::new(std::io::BufReader::new(file("e"))));
    }

    #[test]
    fn f_run() {
        f::solve(f::Reader::new(std::io::BufReader::new(file("f"))));
    }

    use super::*;
}

fn file(alphabet: &str) -> std::fs::File {
    let mut path = std::env::current_dir().unwrap();
    path.push("sample");
    path.push(format!("{}.txt", alphabet));
    std::fs::File::open(path).unwrap()
}

fn main() {}

mod a;
mod b;
mod c;
mod d;
mod e;
mod f;
