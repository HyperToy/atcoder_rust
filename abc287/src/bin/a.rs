use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!(
        "{}",
        if s.iter().filter(|&s| s == "For").count() > n / 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
