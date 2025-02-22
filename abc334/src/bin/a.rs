use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        (b, g): (usize, usize),
    }
    println!(
        "{}",
        match b.cmp(&g) {
            Ordering::Greater => "Bat",
            Ordering::Less => "Glove",
            _ => unreachable!(),
        }
    )
}
