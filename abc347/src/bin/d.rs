use proconio::input;

fn main() {
    input! {
        mut a: i32, mut b: i32, c: i64,
    }
    let c_digit = digit(c);
    let mut x = vec![0; 60];
    let mut y = vec![0; 60];
    for i in 0..60 {
        if c_digit[i] == 1 {
            if a > b {
                x[i] = 1;
                a -= 1;
            } else {
                y[i] = 1;
                b -= 1;
            }
        }
    }
    for i in 0..60 {
        if c_digit[i] == 0 && a > 0 && b > 0 {
            x[i] = 1;
            y[i] = 1;
            a -= 1;
            b -= 1;
        }
    }
    println!(
        "{}",
        if a == 0 && b == 0 {
            format!("{} {}", number(&x), number(&y))
        } else {
            format!("-1")
        }
    )
}

fn digit(mut x: i64) -> Vec<i64> {
    let mut res = vec![0; 60];
    for i in 0..60 {
        res[i] = x % 2;
        x /= 2;
    }
    res
}
fn number(v: &Vec<i64>) -> i64 {
    let mut res = 0;
    for i in (0..60).rev() {
        res *= 2;
        res += v[i];
    }
    res
}
