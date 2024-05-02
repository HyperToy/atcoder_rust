use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [[i32; w]; h],
        b: [[i32; w]; h],
    }
    println!(
        "{}",
        (0..h)
            .permutations(h)
            .cartesian_product((0..w).permutations(w))
            .filter(|(row, col)| {
                (0..h)
                    .cartesian_product(0..w)
                    .all(|(i, j)| a[row[i]][col[j]] == b[i][j])
            })
            .map(|(row, col)| (0..h)
                .cartesian_product(0..h)
                .filter(|(l, r)| l < r && row[*l] > row[*r])
                .count()
                + (0..w)
                    .cartesian_product(0..w)
                    .filter(|(l, r)| l < r && col[*l] > col[*r])
                    .count())
            .map(|count| count as isize)
            .min()
            .unwrap_or(-1)
    );
}
