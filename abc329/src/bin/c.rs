use proconio::{marker::Bytes, *};

fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut count = [0; 26];
    let mut now = 1;
    count[(s[0] - b'a') as usize] = 1;
    for i in 1..n {
        if s[i] != s[i - 1] {
            count[(s[i - 1] - b'a') as usize] = count[(s[i - 1] - b'a') as usize].max(now);
            now = 1;
        } else {
            now += 1;
        }
        if i == n - 1 {
            count[(s[i] - b'a') as usize] = count[(s[i] - b'a') as usize].max(now);
        }
    }
    println!("{}", count.iter().sum::<u64>());
}
