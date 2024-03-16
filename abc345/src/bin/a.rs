use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let l = s.len();

    let mut ok = true;
    for i in 0..l {
        if i == 0 && s[i] != '<' || i == l - 1 && s[i] != '>' || 0 < i && i < l - 1 && s[i] != '=' {
            ok = false;
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
