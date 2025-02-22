use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
    }
    println!(
        "{}",
        (0..3u32.pow(n))
            .map(|i| {
                (0..3u32.pow(n))
                    .map(move |j| if check((i, j), n) { '#' } else { '.' })
                    .join("")
            })
            .join("\n")
    );
}
fn check((i, j): (u32, u32), k: u32) -> bool {
    if k == 0 {
        return true;
    }
    let l = 3u32.pow(k - 1);
    if l <= i && i < l * 2 && l <= j && j < l * 2 {
        false
    } else {
        check((i % l, j % l), k - 1)
    }
}
