use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    println!(
        "{}",
        t.into_iter()
            .enumerate()
            .fold((String::new(), 0, ""), |(answer, target, sep), (i, c)| {
                if c == s[target] {
                    (answer + sep + (i + 1).to_string().as_str(), target + 1, " ")
                } else {
                    (answer, target, sep)
                }
            })
            .0
    );
}
