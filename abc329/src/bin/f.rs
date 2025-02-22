use itertools::Itertools;
use proconio::{marker::Usize1, *};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, q: usize,
        c: [i32; n],
        queries: [(Usize1, Usize1); q],
    }
    let mut count = vec![HashSet::new(); n];
    c.into_iter().enumerate().for_each(|(i, c)| {
        count[i].insert(c);
    });
    let mut p = (0..n).collect::<Vec<_>>();
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(a, b)| {
                if count[p[a]].len() > count[p[b]].len() {
                    (p[a], p[b]) = (p[b], p[a]);
                }
                for c in count[p[a]].clone() {
                    count[p[b]].insert(c);
                }
                count[p[a]].clear();
                count[p[b]].len()
            })
            .join(" ")
    );
}
