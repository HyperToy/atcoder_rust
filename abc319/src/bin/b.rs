use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    println!(
        "{}",
        (0..=n)
            .map(|i| {
                let mut s = 0;
                for j in 1..=9 {
                    if n % j == 0 && i % (n / j) == 0 {
                        s = j;
                        break;
                    }
                }
                if s == 0 {
                    "-".to_string()
                } else {
                    s.to_string()
                }
            })
            .join("")
    );
}
