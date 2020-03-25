use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n]
    }

    let mut s = a;
    s.sort();
    s.reverse();
    let mut alice = 0;
    let mut bob = 0;
    for (i, j) in (0..n).enumerate() {
        if j % 2 == 0 {
            alice += s[i]
        } else {
            bob += s[i]
        }
    }
    println!("{}", alice - bob)
}
