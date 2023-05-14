use proconio::*;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!(
        "{}",
        if a.abs() < b.abs() {
            "Ant"
        } else if a.abs() > b.abs() {
            "Bug"
        } else {
            "Draw"
        }
    )
}
