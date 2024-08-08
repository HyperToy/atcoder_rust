use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        r: usize, c: usize,
        mut b: [Chars; r],
    }
    for i in 0..r {
        for j in 0..c {
            if let Some(d) = b[i][j].to_digit(10) {
                for ii in 0..r {
                    for jj in 0..c {
                        if dist((i, j), (ii, jj)) <= d as usize && b[ii][jj] == '#' {
                            b[ii][jj] = '.';
                        }
                    }
                }
                b[i][j] = '.';
            }
        }
    }
    println!("{}", b.iter().map(|r| r.iter().join("")).join("\n"));
}
fn dist(p: (usize, usize), q: (usize, usize)) -> usize {
    diff(p.0, q.0) + diff(p.1, q.1)
}
fn diff(a: usize, b: usize) -> usize {
    a.max(b) - a.min(b)
}
