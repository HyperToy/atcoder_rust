use ac_library::{Max, Min, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, k: usize,
        p: [Usize1; n],
    }
    let p_inv = p
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .sorted()
        .map(|(_, i)| i)
        .collect_vec();
    let max = Segtree::<Max<_>>::from(p_inv.clone());
    let min = Segtree::<Min<_>>::from(p_inv);
    println!(
        "{}",
        (0..=n - k)
            .map(|l| (l, l + k))
            .map(|(l, r)| max.prod(l..r) - min.prod(l..r))
            .min()
            .unwrap()
    );
}
