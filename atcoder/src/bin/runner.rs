#[cfg(test)]
mod tests {
    #[test]
    fn a_run() {
        a::solve(a::StdinReader::new(std::io::BufReader::new(file("a"))));
    }

    #[test]
    fn b_run() {
        b::solve(b::StdinReader::new(std::io::BufReader::new(file("b"))));
    }

    #[test]
    fn c_run() {
        c::solve(c::StdinReader::new(std::io::BufReader::new(file("c"))));
    }

    #[test]
    fn d_run() {
        d::solve(d::StdinReader::new(std::io::BufReader::new(file("d"))));
    }

    #[test]
    fn e_run() {
        e::solve(e::StdinReader::new(std::io::BufReader::new(file("e"))));
    }

    #[test]
    fn f_run() {
        f::solve(f::StdinReader::new(std::io::BufReader::new(file("f"))));
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
