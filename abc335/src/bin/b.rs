use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut v = Vec::new();
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k > n {
                    break;
                }
                v.push((i, j, k));
            }
        }
    }
    println!(
        "{}",
        v.iter()
            .map(|(x, y, z)| format!("{} {} {}", x, y, z))
            .join("\n")
    );
}
