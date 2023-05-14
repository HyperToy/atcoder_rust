use proconio::*;

fn main() {
    input! {
        n: u32,
    }
    println!("{}", n * 800 - (n / 15) * 200);
}
