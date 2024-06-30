use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut exist = false;
    for w in 1..s.len() {
        for c in 0..w {
            let j = t.len() * w + c;
            if j < s.len() {
                continue;
            }
            let mut ok = true;
            for i in 0..t.len() {
                let j = i * w + c;
                ok &= j < s.len() && s[j] == t[i];
            }
            exist |= ok;
        }
    }
    println!("{}", if exist { "Yes" } else { "No" });
}
