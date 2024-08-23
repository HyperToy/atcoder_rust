use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[[i32; n]; n]; n],
        q: usize,
        queries: [((usize, usize), (usize, usize), (usize, usize)); q],
    }
    let mut s = vec![vec![vec![0; n + 1]; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            for k in 1..=n {
                s[i][j][k] = a[i - 1][j - 1][k - 1]
                    + (s[i - 1][j][k] + s[i][j - 1][k] + s[i][j][k - 1])
                    - (s[i - 1][j - 1][k] + s[i][j - 1][k - 1] + s[i - 1][j][k - 1])
                    + (s[i - 1][j - 1][k - 1]);
            }
        }
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|((lx, rx), (ly, ry), (lz, rz))| ((lx - 1, rx), (ly - 1, ry), (lz - 1, rz)))
            .map(|((lx, rx), (ly, ry), (lz, rz))| s[rx][ry][rz]
                - (s[lx][ry][rz] + s[rx][ly][rz] + s[rx][ry][lz])
                + (s[lx][ly][rz] + s[rx][ly][lz] + s[lx][ry][lz])
                - (s[lx][ly][lz]))
            .join("\n")
    );
}
