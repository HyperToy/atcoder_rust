use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u32; n],
    }
    let mut stack = vec![h[n - 1]];
    let mut answer = vec![0];
    for i in (0..n - 1).rev() {
        answer.push(stack.len());
        while stack.last().is_some_and(|&x| x < h[i]) {
            stack.pop();
        }
        stack.push(h[i]);
    }
    println!("{}", answer.iter().rev().join(" "));
}
