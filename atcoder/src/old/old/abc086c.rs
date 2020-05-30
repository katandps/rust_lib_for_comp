use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(i32, i32, i32); n]
    }

    let mut t = 0;
    let mut x = 0;
    let mut y = 0;

    for i in 0..n {
        let p = points[i];

        let d = num::abs(x - p.1) + num::abs(y - p.2);
        let time = p.0 - t;
        if d > time || d % 2 != time % 2 {
            println!("No");
            return;
        }
        t = p.0;
        x = p.1;
        y = p.2;
    }
    println!("Yes")
}
