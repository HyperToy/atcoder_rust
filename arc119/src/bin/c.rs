use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    println!("{}", solve(a));
}

fn solve(a: Vec<i64>) -> usize {
    use itertools::Itertools;
    (0..1)
        .chain(a.into_iter())
        .enumerate()
        .map(|(i, x)| x * if i % 2 == 0 { 1 } else { -1 })
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .counts()
        .iter()
        .map(|(_, v)| v * (v - 1) / 2)
        .sum::<usize>()
}
