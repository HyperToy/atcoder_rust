use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }
    println!(
        "{}",
        (0..n)
            .map(|i| {
                (0..n)
                    .map(move |j| rotate_by_count(n, (i, j), distance_from_edge(n, (i, j))))
                    .map(|(i, j)| a[i][j])
                    .join("")
            })
            .join("\n")
    );
}

fn distance_from_edge(n: usize, (i, j): (usize, usize)) -> usize {
    usize::min(usize::min(i + 1, n - i), usize::min(j + 1, n - j)) % 4
}

fn rotate_by_count(n: usize, p: (usize, usize), count: usize) -> (usize, usize) {
    let r = |p: (usize, usize)| rotate(n, p);
    match count {
        0 => p,
        1 => r(p),
        2 => r(r(p)),
        3 => r(r(r(p))),
        _ => unreachable!(),
    }
}

fn rotate(n: usize, (i, j): (usize, usize)) -> (usize, usize) {
    (n - 1 - j, i)
}
