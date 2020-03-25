use proconio::input;

fn main() {
    input! {
    n:i32,
    y:i32
    }

    for n_i in 0..n + 1 {
        for m_i in 0..n - n_i + 1 {
            let k = n - n_i - m_i;
            if n_i * 10000 + m_i * 5000 + k * 1000 == y {
                println!("{} {} {}", n_i, m_i, k);
                return;
            }
        }
    }
    println!("-1 -1 -1")
}
