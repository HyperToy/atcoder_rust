use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let answer = solve(&h);
    println!("{}", answer.iter().join(" "));
}

fn solve(h: &[u32]) -> Vec<usize> {
    let n = h.len();
    let mut stack = vec![];
    let mut answer = vec![];
    for i in (0..n).rev() {
        answer.push(stack.len());
        while stack.last().is_some_and(|&x| x < h[i]) {
            stack.pop();
        }
        stack.push(h[i]);
    }
    answer.into_iter().rev().collect()
}
