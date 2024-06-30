use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }
    let p = (0..n)
        .map(|_| {
            input! {
                c: usize, p: [Usize1; c],
            }
            p
        })
        .collect_vec();
    let mut answer = vec![];
    rec(0, &mut answer, &p, &mut vec![false; n]);
    println!("{}", answer.into_iter().map(|x| x + 1).join(" "));
}
fn rec(i: usize, answer: &mut Vec<usize>, p: &Vec<Vec<usize>>, already: &mut Vec<bool>) {
    if p[i].is_empty() {
        return;
    }
    for &j in &p[i] {
        if already[j] {
            continue;
        }
        rec(j, answer, p, already);
        already[j] = true;
        answer.push(j);
    }
}
