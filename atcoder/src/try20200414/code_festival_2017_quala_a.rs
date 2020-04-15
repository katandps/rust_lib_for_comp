use std::io::*;
use std::str::*;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().unwrap()
}

fn main() {
    let s: String = read();
    let v: Vec<char> = s.chars().collect();

    if v.len() < 4 {
        println!("No");
        return;
    }
    let c = vec!['Y', 'A', 'K', 'I'];
    for i in 0..4 {
        if v[i] != c[i] {
            println!("No");
            return;
        }
    }
    println!("Yes")
}
