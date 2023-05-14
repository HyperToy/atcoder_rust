use proconio::*;

fn main() {
    input! {
        n: u32,
    }
    println!("{}", n * 2);
    if n % 4 != 0 {
        print!("{}", n % 4);
    }
    for _ in 0..n / 4 {
        print!{"{}", 4};
    }
    println!();
}
