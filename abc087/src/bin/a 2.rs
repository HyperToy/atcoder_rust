use proconio::*;

fn main() {
    input! {
        x: u32,
        a: u32,
        b: u32,
    }
    println!("{}", (x - a) % b);
}
