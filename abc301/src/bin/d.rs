use proconio::marker::Bytes;
use proconio::*;

fn main() {
    input! {
        s: Bytes,
        n: i64,
    }
    let mut ans = 0i64;
    for i in 0..s.len() {
        ans <<= 1;
        if s[i] == b'1' {
            ans |= 1;
        }
    }
    for i in 0..s.len() {
        let x = 1 << (s.len() - (i + 1));
        if s[i] == b'?' && ans + x <= n {
            ans |= x;
        }
    }
    if ans > n {
        ans = -1;
    }
    println!("{}", ans);
}
