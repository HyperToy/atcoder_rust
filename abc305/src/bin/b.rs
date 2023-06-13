use proconio::*;

fn main() {
    input! {
        p: char, q: char
    }
    let v: Vec<i32> = vec![0, 3, 4, 8, 9, 14, 23];
    println!(
        "{}",
        (v[(p as u8 - b'A') as usize] - v[(q as u8 - b'A') as usize]).abs()
    );
}
