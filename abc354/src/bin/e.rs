use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n],
    }
    // bit DP
    // dp[S] := S で表される状態から先手番でスタートしたとき、先手必勝であるか
    let mut dp = vec![None; 1 << n];
    println!(
        "{}",
        if dfs(n, &ab, (1 << n) - 1, &mut dp) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

// メモ化再帰
fn dfs(n: usize, ab: &Vec<(i32, i32)>, state: usize, dp: &mut Vec<Option<bool>>) -> bool {
    // メモがあれば返す
    if let Some(x) = dp[state] {
        return x;
    }
    let mut res = false;
    for i in 0..n {
        if state & (1 << i) == 0 {
            continue;
        }
        for j in 0..i {
            if state & (1 << j) == 0 {
                continue;
            }
            if ab[i].0 != ab[j].0 && ab[i].1 != ab[j].1 {
                continue;
            }
            let state = state ^ (1 << i) ^ (1 << j); // i と j を除いた盤面
            res |= !dfs(n, ab, state, dp); // 1つでも必敗盤面を渡せるならば、必勝盤面
        }
    }
    dp[state] = Some(res);
    res
}
