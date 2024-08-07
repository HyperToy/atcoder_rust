use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!(
        "{}",
        (0..30)
            .map(|d| {
                let b = a.iter().map(|&x| (x >> d) & 1).collect_vec();
                let c = b
                    .iter()
                    .scan(0, |s, &x| {
                        *s = *s ^ x;
                        Some(*s)
                    })
                    .collect_vec();
                let now = (c.iter().filter(|&&x| x == 0).count() + 1)
                    * c.iter().filter(|&&x| x == 1).count()
                    - b.iter().sum::<usize>();
                now << d
            })
            .sum::<usize>()
    );
}
