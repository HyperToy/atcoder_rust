use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        m: usize,
        b: [i64; m],
        l: usize,
        c: [i64; l],
        q: usize,
        x: [i64; q],
    }
    let mut possibles = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                possibles.insert(a[i] + b[j] + c[k]);
            }
        }
    }
    let mut answer = vec![false; q];
    for i in 0..q {
        answer[i] = possibles.contains(&x[i]);
    }
    println!(
        "{}",
        answer
            .iter()
            .map(|b| if *b { "Yes" } else { "No" })
            .join(" ")
    );
}
