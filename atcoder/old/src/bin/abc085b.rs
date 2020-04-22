use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n]
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(d[i]);
    }
    println!("{}", set.len())
}
