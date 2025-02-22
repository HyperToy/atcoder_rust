use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
        s: [Chars; n],
    }
    let points = s
        .iter()
        .enumerate()
        .map(|(i, v)| {
            (i + 1)
                + v.iter()
                    .enumerate()
                    .filter(|(_j, c)| **c == 'o')
                    .map(|(j, _c)| a[j])
                    .sum::<usize>()
        })
        .collect::<Vec<_>>();
    let first = points.iter().max().unwrap();
    let second = points.iter().sorted().take(n - 1).max().unwrap();
    for i in 0..n {
        let first = if points[i] == *first { second } else { first };
        let unsolved = s[i]
            .iter()
            .enumerate()
            .filter(|(_j, c)| **c == 'x')
            .map(|(j, _c)| a[j])
            .sorted()
            .rev()
            .collect::<Vec<_>>();
        let mut now = points[i];
        let mut answer = 0;
        while now <= *first {
            now += unsolved[answer];
            answer += 1;
        }
        println!("{}", answer);
    }
}
