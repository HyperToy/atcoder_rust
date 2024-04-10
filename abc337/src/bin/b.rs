use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut ok = true;
    for i in 1..s.len() {
        if s[i - 1] == s[i]
            || (s[i - 1], s[i]) == ('A', 'B')
            || (s[i - 1], s[i]) == ('B', 'C')
            || (s[i - 1], s[i]) == ('A', 'C')
        {
            continue;
        }
        ok = false;
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
