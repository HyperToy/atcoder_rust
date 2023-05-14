use proconio::*;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars,
    }
    let l: usize = s.len();
    s[l - 1] = '5';
    for c in s {
        print!("{}", c);
    }
    println!();
}
