use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        b: [[i32; n]; n],
    }
    let turn = |(i, j)| (n - 1 - j, i);
    println!(
        "{}",
        if (0..4).any(|x| {
            (0..n)
                .cartesian_product(0..n)
                .map(|p| {
                    let mut q = p;
                    for _ in 0..x {
                        q = turn(q)
                    }
                    (q, p)
                })
                .all(|((i, j), (k, l))| a[i][j] == 0 || b[k][l] == 1)
        }) {
            "Yes"
        } else {
            "No"
        }
    );
}
