use proconio::*;

fn main() {
    input! {
        n: usize,
        cities: [(f64, f64); n],
    }
    let mut dist = vec![vec![0.; n]; n];
    for i in 0..n {
        for j in 0..n {
            dist[i][j] = calc_distance(cities[i], cities[j])
        }
    }
    let mut dp = vec![vec![1_000_000_000.; n]; 1 << n];
    dp[0][0] = 0.;
    for s in 0..(1 << n) {
        for v in 0..n {
            for u in 0..n {
                if s != 0 && (s & (1 << u)) == 0 {
                    continue;
                }
                if (s & (1 << v)) == 0 && u != v {
                    if dp[s | (1 << v)][v] > dp[s][u] + dist[u][v] {
                        dp[s | (1 << v)][v] = dp[s][u] + dist[u][v];
                    }
                }
            }
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
}

fn calc_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.) + (a.1 - b.1).powf(2.)).sqrt()
}
