use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut ok = false;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut t = Vec::new();
            for c in s[i].clone() {
                t.push(c);
            }
            for c in s[j].clone() {
                t.push(c);
            }
            if is_kaibun(&t) {
                ok = true;
                break;
            }
        }
        if ok {
            break;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}

fn is_kaibun(s: &Vec<char>) -> bool {
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
