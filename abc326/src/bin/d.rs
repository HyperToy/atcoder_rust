use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    }
    let ans = (0..n)
        .permutations(n)
        .cartesian_product((0..n).permutations(n))
        .cartesian_product((0..n).permutations(n))
        .map(|((a, b), c)| (a, b, c))
        .filter(|(a, b, c)| (0..n).all(|i| a[i] != b[i] && b[i] != c[i] && c[i] != a[i]))
        .map(|(a, b, c)| {
            (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| match j {
                            n if n == a[i] => 'A',
                            n if n == b[i] => 'B',
                            n if n == c[i] => 'C',
                            _ => '.',
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|grid| {
            let mut ok = true;
            for i in 0..n {
                for j in 0..n {
                    if grid[i][j] == '.' {
                        continue;
                    }
                    if grid[i][j] == r[i] {
                        break;
                    }
                    ok = false;
                }
            }
            ok
        })
        .filter(|grid| {
            let mut ok = true;
            for j in 0..n {
                for i in 0..n {
                    if grid[i][j] == '.' {
                        continue;
                    }
                    if grid[i][j] == c[j] {
                        break;
                    }
                    ok = false;
                }
            }
            ok
        })
        .take(1)
        .collect::<Vec<_>>();
    println!(
        "{} {}",
        if ans.len() > 0 { "Yes" } else { "No" },
        ans.get(0)
            .unwrap_or(&vec![])
            .iter()
            .map(|row| row.iter().join(""))
            .join(" ")
    );
}
