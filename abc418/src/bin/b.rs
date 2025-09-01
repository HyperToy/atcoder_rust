use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();
    let max_rate = (0..=n)
        .cartesian_product(0..=n)
        .filter(|(i, j)| i < j)
        .map(|(i, j)| filling_rate(&s[i..j]))
        // (a0 / a1) cmp (b0 / b1)
        .sorted_by(|&a, &b| (a.0 * b.1).cmp(&(b.0 * a.1)))
        .next_back()
        .map(|(x, t)| x as f64 / t as f64)
        .unwrap();
    println!("{:.9}", max_rate);
}

fn filling_rate(s: &[char]) -> (usize, usize) {
    let n = s.len();
    if n < 3 {
        return (0, 1);
    }
    if (s[0], s[n - 1]) != ('t', 't') {
        return (0, 1);
    }
    (s.iter().filter(|c| **c == 't').count() - 2, n - 2)
}
