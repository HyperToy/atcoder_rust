use proconio::*;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut ans: u32 = 700;
    for c in s {
        if c == 'o' {
            ans += 100;
        }
    }
    println!("{}", ans);
}
