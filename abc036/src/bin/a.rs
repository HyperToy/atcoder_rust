use proconio::*;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!("{}", (b + a - 1) / a);
}
