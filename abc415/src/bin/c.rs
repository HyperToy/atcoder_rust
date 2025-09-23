use itertools::Itertools;
use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        t: usize,
        queries: [(usize, Chars); t],
    }
    println!(
        "{}",
        queries
            .into_iter()
            .map(|(n, s)| {
                (
                    n,
                    std::iter::once(true).chain(convert(&s)).collect::<Vec<_>>(),
                )
            })
            .map(solve)
            .map(yes_no)
            .join("\n")
    );
}

fn convert(s: &Vec<char>) -> Vec<bool> {
    s.iter()
        .map(|&c| match c {
            '0' => true,
            '1' => false,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

fn solve((n, s): (usize, Vec<bool>)) -> bool {
    let mut q = VecDeque::new();
    let mut reachable = vec![false; s.len()];
    let start = 0;
    let goal = 2usize.pow(n as u32) - 1;
    reachable[start] = true;
    q.push_back(start);
    while let Some(u) = q.pop_front() {
        for i in 0..n {
            if u >> i & 1 == 1 {
                continue;
            }
            let v = u | (1 << i);
            if reachable[v] {
                continue;
            }
            if !s[v] {
                continue;
            }
            reachable[v] = true;
            q.push_back(v);
        }
    }
    reachable[goal]
}

fn yes_no(b: bool) -> &'static str {
    if b {
        "Yes"
    } else {
        "No"
    }
}
