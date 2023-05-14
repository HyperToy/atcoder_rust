use proconio::*;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    s.dedup();
    for c in s {
        print!("{}{}", c, c);
    }
    println!();
}
