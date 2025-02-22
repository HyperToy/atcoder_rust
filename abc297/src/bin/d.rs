use proconio::*;

fn main() {
    input! {
        a: i64, b: i64,
    }
    println!("{}", solve(a, b));
}

fn solve(mut a: i64, mut b: i64) -> i64 {
    let mut answer = 0;
    while a != b {
        if a < b {
            // (a, b) = (b, a);
            let tmp = a;
            a = b;
            b = tmp;
        }
        if a % b == 0 {
            answer += a / b - 1;
            a = b;
        } else {
            answer += a / b;
            a = a % b;
        }
    }
    answer
}
