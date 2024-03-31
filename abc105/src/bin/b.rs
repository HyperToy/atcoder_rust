use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: i32,
    }
    println!(
        "{}",
        if (0..=(n / 4))
            .cartesian_product(0..=(n / 7))
            .any(|(cake, donut)| cake * 4 + donut * 7 == n)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
