use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut answer = 0;
    let mut x = 1;
    while x * 2 <= n {
        let y = n - x;
        let now = divisors(x) * divisors(y);
        answer += now * if x != y { 2 } else { 1 };
        x += 1;
    }
    println!("{}", answer);
}

fn divisors(x: i64) -> i64 {
    let mut res = 0;
    let mut i = 1;
    while i * i <= x {
        if x % i == 0 {
            res += 1;
            if i * i != x {
                res += 1;
            }
        }
        i += 1;
    }
    res
}
