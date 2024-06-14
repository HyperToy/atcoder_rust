use itertools::Itertools;
use proconio::input;

const N: usize = 3;
fn main() {
    input! {
        c: [[u32; N;]; N],
    }
    let n = 362880.; // 9!
    println!(
        "{}",
        (0..N * N)
            .permutations(N * N)
            .map(|order| order.iter().map(|x| (x / N, x % N)).collect_vec())
            .filter(|order| !disappointed(&c, order))
            .count() as f64
            / n
    );
}
fn disappointed(c: &Vec<Vec<u32>>, order: &Vec<(usize, usize)>) -> bool {
    let mut seen = [[false; N]; N];
    order.iter().any(|&(i, j)| {
        let (i1, j1) = ((i + 1) % N, (j + 1) % N);
        let (i2, j2) = ((i + 2) % N, (j + 2) % N);
        seen[i][j] = true;
        // 縦, 横, 斜め
        (seen[i1][j] && seen[i2][j] && c[i1][j] == c[i2][j])
            || (seen[i][j1] && seen[i][j2] && c[i][j1] == c[i][j2])
            || (i == j && seen[i1][j1] && seen[i2][j2] && c[i1][j1] == c[i2][j2])
            || (i == N - j - 1 && seen[i2][j1] && seen[i1][j2] && c[i2][j1] == c[i1][j2])
    })
}
