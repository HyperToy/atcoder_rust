use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    print_yes_no(a.iter().tuple_windows().all(|(a, b)| a < b));
}

fn print_yes_no(b: bool) {
    println!("{}", if b { "Yes" } else { "No" });
}
