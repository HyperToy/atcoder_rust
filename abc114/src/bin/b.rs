use proconio::*;

fn diff(a: u32, b: u32) -> u32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() {
    input! {
        s: String
    }
    let target: u32 = 753;
    let mut mindiff: u32 = 10000;

    for i in 0..s.len() - 2 {
        let n: u32 = s[i..i + 3].parse().unwrap();
        let nowdiff: u32 = diff(n, target);
        if nowdiff < mindiff {
            mindiff = nowdiff;
        }
    }
    println!("{}", mindiff);
}
