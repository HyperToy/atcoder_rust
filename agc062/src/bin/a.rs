use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            _n: usize,
            s: Chars,
        }
        println!("{}", solve(s));
    }
}

fn solve(s: Vec<char>) -> char {
    let n = s.len();
    let mut b_exists = false;
    let mut ba_exists = false;
    for i in 1..n {
        if s[i - 1] == 'B' && s[i] == 'A' {
            ba_exists = true;
        }
        if s[i] == 'B' {
            b_exists = true;
        }
    }
    if !b_exists || ba_exists {
        'A'
    } else {
        'B'
    }
}
