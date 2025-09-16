use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(char, usize); n],
    }
    println!("{}", solve(&a));
}

fn solve(a: &Vec<(char, usize)>) -> String {
    if a.iter()
        .map(|(_, l)| l)
        .fold(0usize, |sum, v| sum.saturating_add(*v))
        > 100
    {
        return "Too Long".to_string();
    }
    a.iter()
        .map(|(c, l)| std::iter::repeat(*c).take(*l))
        .flatten()
        .join("")
}
