use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }

    let mut max = 100;
    for i in 0..n - 1 {
        let mut t = a[i];
        let mut c = 0;
        loop {
            if t % 2_i32 == 0 {
                t = t / 2_i32;
                c += 1;
            //println!("{}", t)
            } else {
                break;
            }
        }
        max = cmp::min(max, c);
    }
    println!("{}", max)
}
