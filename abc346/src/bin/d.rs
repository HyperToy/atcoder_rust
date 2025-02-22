use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [i64; n],
    }
    let s = s
        .iter()
        .map(|c| if *c == '1' { 1 } else { 0 })
        .collect::<Vec<_>>();
    // 010... or 101... にするためのコスト
    let mut sum = vec![[0; 2]; n + 1];
    for i in 0..n {
        let parity = (i + s[i]) % 2;
        for p in 0..2 {
            sum[i + 1][p] = sum[i][p] + if p == parity { c[i] } else { 0 };
        }
    }
    let mut answer = std::i64::MAX;
    for i in 1..n {
        for p in 0..2 {
            answer = answer.min((sum[n][p] - sum[i][p]) + (sum[i][1 - p] - sum[0][1 - p]));
        }
    }
    println!("{}", answer);
}
