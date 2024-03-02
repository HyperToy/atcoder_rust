use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut answer = 0;
    for x in 1..n {
        if x * 2 > n {
            break;
        }
        let y = n - x;
        let now = divisors(x) * divisors(y);
        answer += now * if x != y { 2 } else { 1 };
    }
    println!("{}", answer);
}

fn divisors(x: i64) -> i64 {
    let mut res = 0;
    for i in 1..=x {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            res += 1;
            if i * i != x {
                res += 1;
            }
        }
    }
    res
}
