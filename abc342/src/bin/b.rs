use proconio::{input, marker::Usize1};
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    }
    let mut inv = vec![0; n];
    for i in 0..n {
        inv[p[i]] = i;
    }
    for (a, b) in queries {
        println!(
            "{}",
            1 + match inv[a].cmp(&inv[b]) {
                Ordering::Greater => b,
                Ordering::Less => a,
                _ => unimplemented!(),
            }
        );
    }
}
