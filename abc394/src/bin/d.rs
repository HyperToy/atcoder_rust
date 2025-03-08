use proconio::{input, marker::Chars};

fn main() {
    input! { s: Chars }
    let mut st = Vec::new();
    for c in s {
        if st.last().is_some_and(|&x| fit(x, c)) {
            st.pop();
        } else {
            st.push(c);
        }
    }
    println!("{}", if st.len() == 0 { "Yes" } else { "No" });
}
fn fit(x: char, c: char) -> bool {
    match (x, c) {
        ('(', ')') | ('[', ']') | ('<', '>') => true,
        _ => false,
    }
}
