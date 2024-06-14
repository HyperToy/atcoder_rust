use itertools::Itertools;
use proconio::input;

const N: usize = 3;
const ALL: f64 = 362880.; // 9!

fn main() {
    input! {
        c: [[u32; N;]; N],
    }
    println!(
        "{}",
        (0..N * N)
            .permutations(N * N)
            .filter(|order| !order
                .iter()
                .map(|x| (x / N, x % N))
                .scan([[false; N]; N], |seen, (i, j)| {
                    seen[i][j] = true;
                    Some((seen.clone(), (i, j)))
                })
                .any(|(seen, (i, j))| {
                    let (i1, i2) = ((i + 1) % N, (i + 2) % N);
                    let (j1, j2) = ((j + 1) % N, (j + 2) % N);
                    // 縦, 横, 斜め
                    check(&seen, &c, (i1, j), (i2, j))
                        || check(&seen, &c, (i, j1), (i, j2))
                        || i == j && check(&seen, &c, (i1, j1), (i2, j2))
                        || i == N - j - 1 && check(&seen, &c, (i2, j1), (i1, j2))
                }))
            .count() as f64
            / ALL
    );
}
fn check(seen: &[[bool; N]; N], c: &Vec<Vec<u32>>, p: (usize, usize), q: (usize, usize)) -> bool {
    seen[p.0][p.1] && seen[q.0][q.1] && c[p.0][p.1] == c[q.0][q.1]
}
