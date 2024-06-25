use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        es: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    for &(u, v) in &es {
        if a[u] == a[v] {
            dsu.merge(u, v);
        }
    }
    let es = es
        .into_iter()
        .map(|(u, v)| (dsu.leader(u), dsu.leader(v)))
        .filter(|&(u, v)| u != v)
        .map(|(u, v)| if a[u] < a[v] { (u, v) } else { (v, u) })
        .sorted_by(|&(u1, v1), &(u2, v2)| (a[u1], a[v1]).cmp(&(a[u2], a[v2])))
        .dedup()
        .collect_vec();

    let start = dsu.leader(0);
    let goal = dsu.leader(n - 1);
    let mut dp = vec![0; n];
    dp[start] = 1;
    for (u, v) in es {
        if dp[u] > 0 {
            dp[v] = dp[v].max(dp[u] + 1);
        }
    }

    println!("{}", dp[goal]);
}
