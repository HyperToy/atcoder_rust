use std::io::{stdin, stdout, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();
    let mut left = 1;
    let mut right = n;
    while left + 1 < right {
        let middle = (left + right) / 2;
        println!("? {}", middle);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let s: i32 = input.trim().parse().unwrap();
        if s == 0 {
            left = middle;
        } else {
            right = middle;
        }
    }
    println!("! {}", left);
}
