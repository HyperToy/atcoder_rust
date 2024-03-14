use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: Chars,
        n: usize,
    }
    let len = t.len();
    let mut ss = Vec::new();
    for _ in 0..n {
        input! {
            a: usize, s: [Chars; a],
        }
        ss.push(s);
    }
    let mut dp = vec![vec![std::i32::MAX - 1; len + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=len {
            // i 番目の袋を使わない
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);

            // i 番目の袋のどれかを使って、 t
            for s in &ss[i] {
                for k in j + 1..=len {
                    // t の j 番目まで + s == t の k番目まで か
                    if same(&t[0..j], s, &t[0..k]) {
                        dp[i + 1][k] = dp[i + 1][k].min(dp[i][j] + 1);
                    }
                }
            }
        }
    }
    println!(
        "{}",
        if dp[n][len] < std::i32::MAX - 1 {
            dp[n][len]
        } else {
            -1
        }
    )
}

fn same(tj: &[char], s: &Vec<char>, tk: &[char]) -> bool {
    if tj.len() + s.len() != tk.len() {
        return false;
    }
    for i in 0..tj.len() {
        if tj[i] != tk[i] {
            return false;
        }
    }
    for i in 0..s.len() {
        if s[i] != tk[tj.len() + i] {
            return false;
        }
    }
    true
}
