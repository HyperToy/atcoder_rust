use proconio::marker::Chars;
use proconio::*;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, m: usize,
        mut h: i32, k: i32,
        s: Chars,
        ps: [(i32, i32); m],
    }
    let mut now = (0, 0);
    let ps: HashSet<(i32, i32)> = ps.into_iter().collect();
    let mut already: HashSet<(i32, i32)> = HashSet::new();
    let mut ok = true;
    for i in 0..n {
        h -= 1;
        let dir = direction(&s[i]);
        now = (now.0 + dir.0, now.1 + dir.1);
        if h < 0 {
            ok = false;
            break;
        }
        if h < k && !already.contains(&now) && ps.contains(&now) {
            already.insert(now);
            h = k;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}

fn direction(dir: &char) -> (i32, i32) {
    match dir {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => unreachable!(),
    }
}
