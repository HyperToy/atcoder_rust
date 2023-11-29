use proconio::{input, marker::Chars};
use std::collections::VecDeque;

// refactor DRY
fn main() {
    input! {
        n: usize, m: usize,
        mut s: Chars,
        t: Chars,
    }
    let mut queue = VecDeque::new();
    for i in 0..n - m + 1 {
        if (0..m).any(|j| s[i + j] != '#') && (0..m).all(|j| s[i + j] == '#' || s[i + j] == t[j]) {
            queue.push_back(i);
        }
    }
    while !queue.is_empty() {
        let k = queue.pop_front().unwrap();
        for j in 0..m {
            s[k + j] = '#';
        }
        let from = if k >= m { k - m } else { 0 };
        let to = (k + m).min(n - m + 1);
        for i in from..to {
            if (0..m).any(|j| s[i + j] != '#')
                && (0..m).all(|j| s[i + j] == '#' || s[i + j] == t[j])
            {
                queue.push_back(i);
            }
        }
    }
    println!(
        "{}",
        if s.into_iter().all(|c| c == '#') {
            "Yes"
        } else {
            "No"
        }
    );
}
