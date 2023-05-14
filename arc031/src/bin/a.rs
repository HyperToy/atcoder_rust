use proconio::*;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut ok: bool = true;
    for i in 0..s.len() {
        if s[i] != s[s.len() - 1 - i] {
            ok = false;
        }
    }
    println!(
        "{}",
        if ok {
            "YES"
        } else {
            "NO"
        }
    )
}
