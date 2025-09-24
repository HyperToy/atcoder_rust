use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
    }
    let mut answer = 0i64;
    let (mut even, mut odd) = (1, 0);
    let mut sum = 0;
    for i in 0..n {
        sum += if t[i] == '0' { 1 } else { 0 };
        if sum % 2 == 0 {
            answer += even;
            even += 1;
        } else {
            answer += odd;
            odd += 1;
        }
    }
    println!("{}", answer);
}
