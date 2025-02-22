use proconio::input;

fn main() {
    input! {
        (w, b): (usize, usize),
    }
    let mut s = String::new();
    while s.len() < 2 * (w + b) {
        s += "wbwbwwbwbwbw";
    }
    let mut ok = false;
    for i in 0..s.len() - (w + b) {
        let mut now_w = 0;
        let mut now_b = 0;
        for j in 0..w + b {
            if s.chars().nth(i + j).unwrap() == 'w' {
                now_w += 1;
            } else {
                now_b += 1;
            }
            if now_w == w && now_b == b {
                ok = true;
            }
        }
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
