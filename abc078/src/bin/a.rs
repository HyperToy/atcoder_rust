use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: char, b: char,
    }
    println!(
        "{}",
        match a.cmp(&b) {
            Ordering::Greater => '>',
            Ordering::Less => '<',
            Ordering::Equal => '=',
        }
    );
}
