use proconio::*;

fn main() {
    input! {
        a: usize, b: usize,
    }
    let mut count = 0;
    for i in a..=b {
        if 100 % i == 0 {
            count += 1;
        }
    }
    println!("{}", if count > 0 { "Yes" } else { "No" });
}
