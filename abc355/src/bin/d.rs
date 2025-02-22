use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        intervals: [(i64, i64); n],
    }
    let mut events = BTreeMap::new();
    for &(l, r) in &intervals {
        events.entry(l).or_insert((0, 0)).0 += 1i64;
        events.entry(r).or_insert((0, 0)).1 += 1i64;
    }
    let mut active_intervals = 0;
    let mut count = 0;
    for (_, (start, end)) in events {
        count += active_intervals * start;
        count += start * (start - 1) / 2;
        active_intervals += start;
        active_intervals -= end;
    }
    println!("{}", count);
}
