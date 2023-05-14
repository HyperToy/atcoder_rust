use proconio::*;

fn main() {
    input! {
        mut d: u8,
    }
    print!("Christmas");
    while d < 25 {
        print!(" Eve");
        d += 1;
    }
    println!();
}
