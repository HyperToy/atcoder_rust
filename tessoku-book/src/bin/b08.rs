use proconio::*;

fn main() {
    input! {
        n: usize,
        ps: [(usize, usize); n],
        q: usize,
        qs: [((usize, usize), (usize, usize)); q],
    }
    let mut sum = vec![vec![0; 1501]; 1501];
    for (x, y) in ps {
        sum[x][y] += 1;
    }
    for i in 0..1501 {
        for j in 1..1501 {
            sum[i][j] += sum[i][j - 1];
        }
    }
    for j in 0..1501 {
        for i in 1..1501 {
            sum[i][j] += sum[i - 1][j];
        }
    }
    for ((a, b), (c, d)) in qs {
        println!(
            "{}",
            sum[c][d] + sum[a - 1][b - 1] - sum[c][b - 1] - sum[a - 1][d]
        );
    }
}
