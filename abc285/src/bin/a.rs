use proconio::input;

fn main() {
    input! {
        a: usize, b: usize,
    }
    println!("{}", if a == b / 2 { "Yes" } else { "No" });
}
