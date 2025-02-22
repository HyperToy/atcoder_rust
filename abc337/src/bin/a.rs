use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }
    let (takahashi, aoki) = xy
        .into_iter()
        .fold((0, 0), |(sx, sy), (x, y)| (sx + x, sy + y));
    println!(
        "{}",
        match takahashi.cmp(&aoki) {
            Ordering::Greater => "Takahashi",
            Ordering::Less => "Aoki",
            Ordering::Equal => "Draw",
        }
    );
}
