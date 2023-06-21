use proconio::*;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        qs: [(usize, usize); q],
    }
    let mut hit_count = vec![0; n + 2];
    for i in 0..n {
        hit_count[i + 1] = hit_count[i] + a[i];
    }
    for (l, r) in qs {
        let hit = hit_count[r] - hit_count[l - 1];
        let miss = r - (l - 1) - hit;
        println!(
            "{}",
            match hit.cmp(&miss) {
                Ordering::Less => "lose",
                Ordering::Equal => "draw",
                Ordering::Greater => "win",
            }
        );
    }
}
