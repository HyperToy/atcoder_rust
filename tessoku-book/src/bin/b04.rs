use proconio::*;

fn main() {
    input! {
        n: String,
    }
    println!("{}", i32::from_str_radix(n.as_str(), 2).unwrap());
}
