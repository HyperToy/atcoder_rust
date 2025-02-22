use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    println!(
        "{}",
        if s.windows(2)
            .take(n - 2)
            .all(|v| v[0] != "sweet" || v[1] != "sweet")
        {
            "Yes"
        } else {
            "No"
        }
    );
}
