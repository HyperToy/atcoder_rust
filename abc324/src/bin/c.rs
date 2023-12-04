use itertools::Itertools;
use proconio::{input, marker::Chars};

// refactor
fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    }
    let answer = s
        .into_iter()
        .enumerate()
        .filter(|(_i, s)| similar(&t, s))
        .map(|(i, _s)| i + 1)
        .collect::<Vec<_>>();
    println!("{} {}", answer.len(), answer.iter().join(" "));
}

fn similar(t: &Vec<char>, s: &Vec<char>) -> bool {
    let mut diff_count = 0;
    if t.len() > s.len() {
        // insert
        if t.len() - s.len() > 1 {
            return false;
        }
        for i in 0..s.len() {
            if s[i] != t[i + diff_count] {
                diff_count += 1;
                if t.len() > i + diff_count && s[i] != t[i + diff_count] {
                    diff_count += 1
                }
                if diff_count > 1 {
                    break;
                }
            }
        }
    } else if t.len() < s.len() {
        // delete
        if s.len() - t.len() > 1 {
            return false;
        }
        for i in 0..t.len() {
            if s[i + diff_count] != t[i] {
                diff_count += 1;
                if s.len() > i + diff_count && s[i + diff_count] != t[i] {
                    diff_count += 1
                }
                if diff_count > 1 {
                    break;
                }
            }
        }
    } else {
        for i in 0..t.len() {
            if t[i] != s[i] {
                diff_count += 1;
            }
        }
    }
    diff_count <= 1
}
