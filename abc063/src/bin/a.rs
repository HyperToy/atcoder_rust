use proconio::*;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    if a + b < 10 {
        println!("{}", a + b);
    } else {
        println!("error");
    }
}
