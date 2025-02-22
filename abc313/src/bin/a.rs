use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [i32; n],
    }
    println!(
        "{}",
        (*p.iter().skip(1).max().unwrap_or(&0) - p[0] + 1).max(0)
    );
}
