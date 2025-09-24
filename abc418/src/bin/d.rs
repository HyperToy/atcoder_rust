use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
    }
    let t = t
        .into_iter()
        .map(|c| match c {
            '1' => 1,
            '0' => 0,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + if t[i] == 0 { 1 } else { 0 };
    }
    let mut answer = 0i64;
    let (mut even, mut odd) = (1i64, 0i64);
    for i in 1..=n {
        if s[i] % 2 == 0 {
            answer += even;
            even += 1;
        } else {
            answer += odd;
            odd += 1;
        }
    }
    println!("{}", answer);
}
