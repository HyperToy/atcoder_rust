fn main() {
    proconio::input! {
        n: u32,
        k: u32,
    }
    println!(
        "{}",
        if k == 1 {
            0
        } else {
            n - k
        }
    );
}
