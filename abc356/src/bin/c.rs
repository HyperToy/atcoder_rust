use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize, k: u32,
    }
    let mut res = Vec::new();
    for _ in 0..m {
        input! {
            c: usize,
            a: [Usize1; c], r: char,
        }
        let mut now = 0;
        for i in a {
            now |= 1 << i;
        }
        res.push((
            now,
            match r {
                'o' => true,
                'x' => false,
                _ => unreachable!(),
            },
        ));
    }
    println!(
        "{}",
        (0..(1 << n))
            .filter(|&mask| {
                (0..m).all(|j| (((mask & (res[j].0)) as i32).count_ones() >= k) == res[j].1)
            })
            .count()
    );
}
