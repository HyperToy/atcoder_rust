use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    println!("{}", yes_no(a.iter().tuple_windows().all(|(a, b)| a < b)));
}

fn yes_no(b: bool) -> String {
    if b { "Yes" } else { "No" }.to_string()
}
