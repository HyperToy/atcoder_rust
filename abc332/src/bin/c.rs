use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize, m: i32,
        s: Chars,
    }
    let mut logo_min = 0;
    let mut muji_now = m;
    let mut logo_now = 0;
    for c in s {
        match c {
            '0' => {
                logo_min = logo_min.min(logo_now);
                muji_now = m;
                logo_now = 0;
            }
            '1' => {
                if muji_now > 0 {
                    muji_now -= 1;
                } else {
                    logo_now -= 1;
                }
            }
            '2' => {
                logo_now -= 1;
            }
            _ => unreachable!(),
        }
    }
    logo_min = logo_min.min(logo_now);
    println!("{}", -logo_min);
}
