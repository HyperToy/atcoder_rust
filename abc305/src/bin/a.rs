use proconio::*;

fn main() {
    input! {
        n: i32,
    }
    println!(
        "{}",
        if n % 5 < 3 {
            n - (n % 5)
        } else {
            (n + 5) - ((n + 5) % 5)
        }
    );
}
