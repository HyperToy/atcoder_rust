use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        a: [[u8; w]; h],
    }
    println!(
        "{}",
        a.into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|b| if b == 0 { '.' } else { (b + b'A' - 1) as char })
                    .join("")
            })
            .join("\n")
    );
}
