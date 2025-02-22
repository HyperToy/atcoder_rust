use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        s: [Chars; n],
        q: usize,
        queries: [(Usize1, Usize1); q],
    }
    let mut dist = vec![vec![None; n]; n];
    for i in 0..n {
        dist[i][i] = Some((0, a[i]));
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                dist[i][j] = Some((1, a[i] + a[j]));
            }
        }
    }
    // Floyd–Warshall
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let Some((x1, y1)) = dist[i][k] {
                    if let Some((x2, y2)) = dist[k][j] {
                        let now_dist = x1 + x2;
                        let now_value = y1 + y2 - a[k];
                        dist[i][j] = match dist[i][j] {
                            None => Some((now_dist, now_value)),
                            Some((x, y)) => {
                                if x > now_dist || (x == now_dist && y < now_value) {
                                    Some((now_dist, now_value))
                                } else {
                                    dist[i][j]
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(u, v)| match dist[u][v] {
                None => "Impossible".to_string(),
                Some((x, y)) => format!("{} {}", x, y),
            })
            .join("\n")
    );
}
