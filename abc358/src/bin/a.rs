use proconio::input;

fn main() {
    input! {
        s: [String; 2],
    }
    println!(
        "{}",
        if s == vec!["AtCoder".to_string(), "Land".to_string()] {
            "Yes"
        } else {
            "No"
        }
    );
}
