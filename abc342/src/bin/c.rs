use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
        q: usize,
        queries: [(char, char); q],
    }
    let mut convert = (0..26).collect::<Vec<u8>>();
    let queries = queries
        .iter()
        .map(|(c, d)| (*c as u8 - b'a', *d as u8 - b'a'))
        .collect::<Vec<_>>();
    for (c, d) in queries {
        for x in &mut convert {
            if *x == c {
                *x = d;
            }
        }
    }
    println!(
        "{}",
        s.iter()
            .map(|c| (convert[(*c as u8 - b'a') as usize] + b'a') as char)
            .join("")
    );
}
