use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let answer = a
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i))
        .sorted_by_key(|(a, _)| *a)
        .group_by(|(a, _)| *a)
        .into_iter()
        .map(|(_, positions)| positions.map(|(_, i)| i).chain(std::iter::once(n)))
        .map(|positions| {
            positions
                .tuple_windows()
                .map(|(now, next)| (now + 1) * (next - now))
        })
        .flatten()
        .sum::<usize>();
    println!("{}", answer);
}
