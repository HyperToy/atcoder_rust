use proconio::{marker::Chars, *};

// refactor
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ok = false;
    for i in 0..n - 1 {
        match (s[i], s[i + 1]) {
            ('a', 'b') | ('b', 'a') => {
                ok = true;
            }
            _ => (),
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
