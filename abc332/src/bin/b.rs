use proconio::input;

fn main() {
    input! {
        k: i32, g: i32, m: i32,
    }
    let mut a = 0;
    let mut b = 0;
    for _ in 0..k {
        if a == g {
            a = 0;
        } else if b == 0 {
            b = m;
        } else {
            let t = (g - a).min(b);
            a += t;
            b -= t;
        }
    }
    println!("{} {}", a, b);
}
