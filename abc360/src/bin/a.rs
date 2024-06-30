use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    println!(
        "{}",
        if s.iter().position(|&c| c == 'R') < s.iter().position(|&c| c == 'M') {
            "Yes"
        } else {
            "No"
        }
    );
}
