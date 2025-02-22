use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize, k: u32,
    }
    let res = (0..m)
        .map(|_| {
            input! {
                c: usize,
                a: [Usize1; c],
                r: char,
            }
            (
                a.into_iter().map(|i| 1 << i).sum::<i32>(),
                match r {
                    'o' => true,
                    'x' => false,
                    _ => unreachable!(),
                },
            )
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        (0..(1 << n))
            .filter(|&mask| {
                (0..m).all(|j| ((mask & res[j].0 as i32).count_ones() >= k) == res[j].1)
            })
            .count()
    );
}
