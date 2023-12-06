use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut answer = 1;
    for r in 1..=s.len() {
        for l in 0..r {
            if is_kaibun(&s, l, r) {
                answer = answer.max(r - l);
            }
        }
    }
    println!("{}", answer);
}

fn is_kaibun(s: &Vec<char>, l: usize, r: usize) -> bool {
    let mut l = l;
    let mut r = r - 1;
    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}
