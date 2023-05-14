use proconio::*;

fn main() {
    input! {
        n: u32,
        h: u32,
        w: u32,
    }
    println!("{}", (n - h + 1) * (n - w + 1));
}
