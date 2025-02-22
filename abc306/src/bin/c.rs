use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; 3 * n],
    }
    let mut count = vec![0; n];
    let mut pos = vec![(0, 0); n];
    for i in 0..n * 3 {
        count[a[i]] += 1;
        if count[a[i]] == 2 {
            pos[a[i]] = (i, a[i]);
        }
    }
    println!("{}", pos.iter().sorted().map(|(_, a)| a + 1).join(" "));
}
