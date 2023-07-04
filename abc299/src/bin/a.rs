use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut between = false;
    let mut ok = false;
    for i in 0..n {
        match s[i] {
            '|' => {
                between = !between;
            }
            '*' => {
                ok = between;
            }
            _ => (),
        }
    }
    println!("{}", if ok { "in" } else { "out" });
}
