use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize, s: Chars,
    }
    println!(
        "{}",
        yes_no(s.len() >= 3 && s[s.len() - 3..] == ['t', 'e', 'a'])
    );
}

fn yes_no(b: bool) -> String {
    if b { "Yes" } else { "No" }.to_string()
}
