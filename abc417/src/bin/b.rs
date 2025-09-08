use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        b: [i32; m],
    }
    let mut answer = vec![];
    let mut s = b.into_iter().sorted().rev().collect::<Vec<_>>();
    for &a in &a {
        while s.last().is_some_and(|&x| x < a) {
            s.pop();
        }
        if s.last() == Some(&a) {
            s.pop();
        } else {
            answer.push(a);
        }
    }
    if !answer.is_empty() {
        println!("{}", answer.into_iter().join(" "));
    }
}
