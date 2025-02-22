use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let v = a.iter().copied().sorted().collect::<Vec<_>>();
    let mut sum = vec![0; n];
    for i in 0..n {
        sum[i] = v[i] + if i > 0 { sum[i - 1] } else { 0 };
    }

    println!(
        "{}",
        a.into_iter()
            .map(|a| {
                let mut ok = 0;
                let mut ng = n;
                while ok + 1 < ng {
                    let wj = (ok + ng) / 2;
                    if v[wj] > a {
                        ng = wj;
                    } else {
                        ok = wj
                    }
                }
                sum[n - 1] - sum[ok]
            })
            .join(" ")
    );
}
