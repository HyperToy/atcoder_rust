use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let t = if t[2] == 'X' { t[0..2].to_vec() } else { t };
    let n = s.len();
    let mut pos = 0;
    for i in 0..n {
        if s[i] == t[pos].to_ascii_lowercase() {
            pos += 1;
        }
        if pos == t.len() {
            break;
        }
    }
    println!("{}", if pos == t.len() { "Yes" } else { "No" });
}
