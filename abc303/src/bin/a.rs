use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let mut ok = true;
    for i in 0..n {
        if s[i] != t[i] && !similar(&s[i], &t[i]) {
            ok = false;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}

fn similar(x: &char, y: &char) -> bool {
    match (x, y) {
        ('l', '1') | ('1', 'l') | ('o', '0') | ('0', 'o') => true,
        _ => false,
    }
}
