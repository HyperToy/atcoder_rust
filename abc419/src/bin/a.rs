use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!(
        "{}",
        match s.as_str() {
            "red" => "SSS",
            "blue" => "FFF",
            "green" => "MMM",
            _ => "Unknown",
        }
    )
}
