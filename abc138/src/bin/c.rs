use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [i32; n],
    }
    let v = v.iter().sorted().rev().collect::<Vec<_>>();
    let last = **v.last().unwrap();
    let mut mul = 1.;
    let mut answer = 0.;
    for &x in v {
        mul *= 2.;
        answer += x as f64 / mul;
    }
    answer += last as f64 / mul;
    println!("{}", answer);
}
