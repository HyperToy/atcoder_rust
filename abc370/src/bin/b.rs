use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n * (n + 1) / 2],
    }
    let mut g = vec![vec![0; n]; n];
    for (a, (i, j)) in a
        .into_iter()
        .zip((0..n).map(|i| (0..=i).map(move |j| (i, j))).flatten())
    {
        g[i][j] = a;
        g[j][i] = a;
    }
    println!("{}", (0..n).fold(0, |a, i| g[a][i]) + 1);
}
