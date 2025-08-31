use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        t: Chars,
        a: Chars,
    }
    println!(
        "{}",
        yes_no(t.iter().zip(a.iter()).any(|(t, a)| t == a && t == &'o'))
    );
}

fn yes_no(b: bool) -> String {
    if b { "Yes" } else { "No" }.to_string()
}
