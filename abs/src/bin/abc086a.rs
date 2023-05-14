use proconio::*;

fn main() {
    input! {
        a: i32, b:i32,
    }
    if a % 2 == 1 && b % 2 == 1 {
        println!("Odd");
    } else {
        println!("Even");
    }
}
