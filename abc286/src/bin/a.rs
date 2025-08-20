use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, ((p, q), (r, s)): ((Usize1, Usize1), (Usize1, Usize1)),
        a: [i32; n],
    }
    println!(
        "{}",
        (0..n)
            .map(|i| match i {
                x if (p..=q).contains(&x) => a[r + x - p],
                x if (r..=s).contains(&x) => a[p + x - r],
                _ => a[i],
            })
            .join(" ")
    );
}
