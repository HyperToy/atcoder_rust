use proconio::input;

fn main() {
    input! {
        l: usize, r: usize,
    }
    println!(
        "{}",
        if l == r {
            "Invalid"
        } else if l == 1 {
            "Yes"
        } else {
            "No"
        }
    );
}
