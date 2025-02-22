use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        mut n: u32,
    }
    let v = (0..4)
        .filter_map(|i| if n >> i & 1 == 1 { Some(1 << i) } else { None })
        .collect_vec();
    println!("{}\n{}", v.len(), v.into_iter().join("\n"));
}
