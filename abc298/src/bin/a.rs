use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }
    println!(
        "{}",
        if s.iter().all(|&c| c != 'x') && s.iter().any(|&c| c == 'o') {
            "Yes"
        } else {
            "No"
        }
    );
}
