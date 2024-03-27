use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize, k: Usize1,
        mut s: Chars,
    }
    s[k] = match s[k] {
        'A' => 'a',
        'B' => 'b',
        'C' => 'c',
        _ => unreachable!(),
    };
    println!("{}", s.iter().join(""));
}
