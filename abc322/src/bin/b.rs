use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize, _m: usize,
        s: Chars,
        t: Chars,
    }
    println!(
        "{}",
        if is_prefix(&s, &t) { 0 } else { 2 } + if is_suffix(&s, &t) { 0 } else { 1 }
    );
}
fn is_prefix(s: &Vec<char>, t: &Vec<char>) -> bool {
    s.iter().enumerate().all(|(i, c)| t[i] == *c)
}
fn is_suffix(s: &Vec<char>, t: &Vec<char>) -> bool {
    let n = s.len();
    let m = t.len();
    s.iter().enumerate().all(|(i, c)| t[m - n + i] == *c)
}
