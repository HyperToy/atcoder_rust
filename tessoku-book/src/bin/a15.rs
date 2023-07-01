use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let compressed = compress(&a);
    println!("{}", compressed.iter().map(|x| x + 1).join(" "));
}

fn compress(a: &Vec<i32>) -> Vec<usize> {
    let mut v = a.clone();
    v.sort();
    v.dedup();

    let get_order = |x| {
        let mut ok = 0;
        let mut ng = v.len();
        while ng - ok > 1 {
            let wj = (ok + ng) / 2;
            if v[wj] <= x {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        ok
    };

    let mut compressed = Vec::new();
    for i in 0..a.len() {
        compressed.push(get_order(a[i]));
    }
    compressed
}
