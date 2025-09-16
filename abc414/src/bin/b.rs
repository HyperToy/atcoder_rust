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
    if a.iter().any(|(_, l)| *l > 100) {
        return "Too Long".to_string();
    }
    if a.iter().map(|(_, l)| l).sum::<usize>() > 100 {
        return "Too Long".to_string();
    }
    a.iter()
        .map(|(c, l)| std::iter::repeat(*c).take(*l).collect::<String>())
        .join("")
}
