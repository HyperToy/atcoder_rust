use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    println!(
        "{}",
        if a.iter().unique().count() == 1 {
            "Yes"
        } else {
            "No"
        }
    );
}
