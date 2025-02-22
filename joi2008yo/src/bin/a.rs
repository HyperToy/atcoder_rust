use proconio::*;

fn main() {
    input! {
        n: u32,
    }
    let payment = 1000;
    let coins = vec![500, 100, 50, 10, 5, 1];
    let mut rem_change = payment - n;
    let mut answer = 0;
    for coin in coins {
        answer += rem_change / coin;
        rem_change %= coin;
    }
    println!("{}", answer);
}
