use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let n = s.len();
    println!(
        "{}",
        if s[0] != s[1] && s[0] != s[2] {
            1
        } else if s[n - 1] != s[n - 2] && s[n - 1] != s[n - 3] {
            n
        } else {
            let mut now = 0;
            for i in 1..s.len() - 1 {
                if s[i] != s[i - 1] && s[i] != s[i + 1] {
                    now = i + 1;
                }
            }
            now
        }
    );
}
