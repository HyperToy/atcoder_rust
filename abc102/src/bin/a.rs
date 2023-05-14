use proconio::*;

fn main() {
    input! {
        n: u32,
    }
    println!(
        "{}",
        match n % 2 {
            0 => n,
            1 => n * 2,
            _ => unreachable!(),
        }
    )
}
