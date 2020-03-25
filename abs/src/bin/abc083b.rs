use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32
    }

    let mut count = 0;
    for i in 1..n + 1 {
        let mut t = i;
        let mut s = 0;
        while t > 0 {
            s += t % 10;
            t /= 10;
        }
        if s >= a && s <= b {
            count += i;
        }
    }
    println!("{}", count)
}
