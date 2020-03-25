use proconio::input;

fn main() {
    input! {
    a: i32,b:i32,c:i32,x:i32
    }

    let mut count = 0;
    for a_i in 0..a + 1 {
        for b_i in 0..b + 1 {
            for c_i in 0..c + 1 {
                if a_i * 500 + b_i * 100 + c_i * 50 == x {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count)
}
