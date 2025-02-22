use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ok = true;
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            ok = false;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
