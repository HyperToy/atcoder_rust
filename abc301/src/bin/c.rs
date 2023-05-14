use proconio::*;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
        t: Bytes,
    }
    let count = |a| {
        let mut cnt = [0; 27];
        for c in a {
            if c == b'@' {
                cnt[26] += 1;
            } else {
                cnt[(c - b'a') as usize] += 1;
            }
        }
        cnt
    };
    let mut a = count(s);
    let mut b = count(t);
    let mut ok = true;
    let c = "atcoder";
    for i in 0..26 {
        if a[i] == b[i] {
            continue;
        }
        let p = b'a' + i as u8;
        if c.bytes().all(|c| c != p) {
            ok = false;
            break;
        }
        if a[i] > b[i] {
            b[26] -= a[i] - b[i];
        } else {
            a[26] -= b[i] - a[i];
        }
    }
    if a[26] < 0 || b[26] < 0 {
        ok = false;
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
