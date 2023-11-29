use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, q: usize,
        mut a: [usize; n],
        queries: [(Usize1, usize); q],
    }
    let mut count = vec![0; n + 1];
    let mut zero = BTreeSet::new();
    for i in 0..n {
        if a[i] <= n {
            count[a[i]] += 1;
        }
    }
    for i in 0..=n {
        if count[i] == 0 {
            zero.insert(i);
        }
    }
    for (i, x) in queries {
        if a[i] <= n {
            count[a[i]] -= 1;
            if count[a[i]] == 0 {
                zero.insert(a[i]);
            }
        }
        if x <= n {
            count[x] += 1;
            zero.remove(&x);
        }
        a[i] = x;
        println!("{}", zero.first().unwrap());
    }
}
