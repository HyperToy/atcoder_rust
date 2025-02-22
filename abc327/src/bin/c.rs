use proconio::*;
use std::collections::HashSet;

// refactor
fn main() {
    input! {
        a: [[i32; 9]; 9],
    }
    let mut ok = true;
    for i in 0..9 {
        let mut set = HashSet::new();
        for j in 0..9 {
            set.insert(a[i][j]);
        }
        if set.len() != 9 {
            ok = false;
        }
    }
    for j in 0..9 {
        let mut set = HashSet::new();
        for i in 0..9 {
            set.insert(a[i][j]);
        }
        if set.len() != 9 {
            ok = false;
        }
    }
    let mut sets = vec![vec![HashSet::new(); 3]; 3];
    for i in 0..9 {
        for j in 0..9 {
            sets[i / 3][j / 3].insert(a[i][j]);
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if sets[i][j].len() != 9 {
                ok = false;
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
