fn main() {
    proconio::input! {
        n: u32,
    }
    println! (
        "{}",
        if n / 100 == n % 10 {
            "Yes"
        } else {
            "No"
        }
    )
}
