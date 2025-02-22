use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    println!(
        "{}",
        if w.into_iter()
            .any(|w| ["and", "not", "that", "the", "you",].contains(&w.as_str()))
        {
            "Yes"
        } else {
            "No"
        }
    );
}
