use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [i32; n],
    }
    let mut sum = vec![0; n + 1];
    for i in 0..n {
        sum[i + 1] = sum[i] + w[i];
    }
    let mut answer = std::i32::MAX;
    for i in 1..n {
        let now = (sum[i] * 2 - sum[n]).abs();
        answer = answer.min(now);
    }
    println!("{}", answer);
}
