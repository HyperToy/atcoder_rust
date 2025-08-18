use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    println!("{}", solve(s));
}

fn solve(s: Vec<char>) -> usize {
    s.iter()
        .tuple_windows()
        .filter(|(a, b, c)| (a, b, c) == (&&'#', &&'.', &&'#'))
        .count()
}
