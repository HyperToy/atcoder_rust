use proconio::{marker::Chars, *};

fn main() {
    input! {
        s: Chars,
    }
    let mut b1 = 8;
    let mut b2 = 8;
    let mut r1 = 8;
    let mut r2 = 8;
    let mut k = 8;
    for i in 0..8 {
        match s[i] {
            'B' => {
                if b1 == 8 {
                    b1 = i;
                } else {
                    b2 = i;
                }
            }
            'R' => {
                if r1 == 8 {
                    r1 = i;
                } else {
                    r2 = i;
                }
            }
            'K' => {
                k = i;
            }
            _ => (),
        }
    }
    println!(
        "{}",
        if (b2 - b1) % 2 == 1 && r1 < k && k < r2 {
            "Yes"
        } else {
            "No"
        }
    );
}
