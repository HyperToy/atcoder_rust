use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        s: [Chars; 3],
    }
    let answer = (0..m * 3)
        .permutations(3)
        .filter(|v| s[0][v[0] % m] == s[1][v[1] % m] && s[0][v[0] % m] == s[2][v[2] % m])
        .map(|v| v[0].max(v[1]).max(v[2]))
        .min();
    println!("{}", if let Some(x) = answer { x as isize } else { -1 });
}
