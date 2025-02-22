use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        (h, w): (usize, usize), m: usize,
        queries: [(u8, Usize1, i32); m],
    }
    let mut rows = vec![false; h];
    let mut row_rem = h;
    let mut cols = vec![false; w];
    let mut col_rem = w;
    let mut count = BTreeMap::new();
    for (t, a, x) in queries.into_iter().rev() {
        match t {
            1 => {
                if !rows[a] && col_rem > 0 {
                    row_rem -= 1;
                    *(count.entry(x).or_insert(0)) += col_rem;
                    rows[a] = true;
                }
            }
            2 => {
                if !cols[a] && row_rem > 0 {
                    col_rem -= 1;
                    *(count.entry(x).or_insert(0)) += row_rem;
                    cols[a] = true;
                }
            }
            _ => unreachable!(),
        }
    }
    if row_rem > 0 && col_rem > 0 {
        *(count.entry(0).or_insert(0)) += row_rem * col_rem;
    }
    println!(
        "{}\n{}",
        count.len(),
        count.iter().map(|(i, c)| format!("{} {}", i, c)).join("\n")
    );
}
