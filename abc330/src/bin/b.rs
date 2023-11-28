use itertools::Itertools;
use proconio::*;

fn main() {
    input! {
        n: usize, (l, r): (i32, i32),
        a: [i32; n],
    }
    println!(
        "{}",
        a.into_iter()
            .map(|x| if x < l {
                l
            } else if r < x {
                r
            } else {
                x
            })
            .join(" ")
    );
}
