use proconio::marker::Chars;
use proconio::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut max = -1;
    let mut cnt = 0;
    let mut skewer = false;
    for i in 0..n {
        if s[i] == 'o' {
            cnt += 1;
        } else {
            if cnt > max {
                max = cnt;
            }
            cnt = 0;
            skewer = true;
        }
    }
    if cnt > max {
        max = cnt;
    }
    println!("{}", if skewer && max > 0 { max } else { -1 });
}
