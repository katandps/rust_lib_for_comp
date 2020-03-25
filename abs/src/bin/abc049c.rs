use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut index = s.len();
    loop {
        if index == 0 {
            break;
        }

        if index >= 7 && &s[index - 7..index] == "dreamer" {
            index -= 7;
            continue;
        }
        if index >= 6 && &s[index - 6..index] == "eraser" {
            index -= 6;
            continue;
        }
        if index >= 5 && &s[index - 5..index] == "dream" {
            index -= 5;
            continue;
        }
        if index >= 5 && &s[index - 5..index] == "erase" {
            index -= 5;
            continue;
        }

        println!("NO");
        return;
    }
    println!("YES")
}
