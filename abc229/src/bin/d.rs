use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        s: Chars,
        mut k: usize,
    }
    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    // shaku-tori
    while r < s.len() {
        if s[r] == 'X' {
            r += 1;
        } else if k > 0 {
            k -= 1;
            r += 1;
        } else {
            if r - l > ans {
                ans = r - l;
            }
            if s[l] != 'X' {
                k += 1;
            }
            l += 1;
        }
    }
    if r - l > ans {
        ans = r - l;
    }
    println!("{}", ans);
}
