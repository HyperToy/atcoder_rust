use proconio::{marker::Chars, *};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ok = false;
    for i in 1..n - 1 {
        if s[i - 1] == s[i] && s[i] == s[i + 1] {
            ok = true;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
