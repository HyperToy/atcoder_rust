use itertools::Itertools;
use proconio::{input, marker::Chars, source::line::LineSource};
use std::io::{stdin, BufReader};

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    }
    let mut m = 0;
    while 1 << m < n {
        m += 1;
    }
    let mut v = vec![vec![false; n]; m];
    for i in 0..n {
        for j in 0..m {
            v[j][i] = i / (1 << j) % 2 == 1;
        }
    }
    println!(
        "{m}\n{}",
        v.iter()
            .map(|r| format!(
                "{} {}",
                r.iter().filter(|x| **x).count(),
                r.iter()
                    .enumerate()
                    .filter_map(|(i, x)| if *x { Some(i + 1) } else { None })
                    .join(" ")
            ))
            .join("\n")
    );
    input! {
        from &mut source,
        s: Chars,
    }
    let s = s.iter().map(|c| *c == '1').collect::<Vec<_>>();
    let mut res = vec![true; n];
    for i in 0..n {
        for j in 0..m {
            res[i] &= v[j][i] ^ !s[j];
        }
    }
    println!(
        "{}",
        res.into_iter()
            .enumerate()
            .find_map(|(i, x)| if x { Some(i + 1) } else { None })
            .unwrap()
    );
}
