use proconio::*;

fn main() {
    input! {
        n: u64,
    }
    println!("{}", n / 3 + n / 5 - n / 15);
}
