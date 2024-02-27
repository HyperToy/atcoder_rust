use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        mut r: [usize; n],
        queries: [usize; q],
    }
    r.sort();
    let sum = sum(r);
    println!(
        "{}",
        queries
            .into_iter()
            .map(|x| {
                let mut ok = 0;
                let mut ng = n + 1;
                while ok + 1 < ng {
                    let wj = (ok + ng) / 2;
                    if sum[wj] <= x {
                        ok = wj;
                    } else {
                        ng = wj;
                    }
                }
                ok
            })
            .join(" ")
    );
}

fn sum(a: Vec<usize>) -> Vec<usize> {
    let n = a.len();
    let mut res = vec![0; n + 1];

    for i in 0..n {
        res[i + 1] = a[i] + res[i];
    }
    res
}
