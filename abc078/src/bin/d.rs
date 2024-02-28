use proconio::input;

fn main() {
    input! {
        n: usize, z: i32, w: i32,
        a: [i32; n],
    }

    // 初期手札を山札と同一視する
    let mut v = vec![z, w];
    v.extend(a);
    let n = n + 2;

    // dp[i][j] := X が a[i]、 Y が a[j] を持っている状態からスタートしたときの最終スコア
    // i < j のとき X の手番、 i > j のとき Y の手番
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        dp[n - 1][i] = (v[n - 1] - v[i]).abs();
        dp[i][n - 1] = (v[n - 1] - v[i]).abs();
    }
    for i in (0..n - 1).rev() {
        let mut min = std::i32::MAX;
        let mut max = 0;
        for k in i + 1..n {
            if min > dp[i][k] {
                min = dp[i][k];
            };
            if max < dp[k][i] {
                max = dp[k][i];
            };
        }
        for j in (0..i).rev() {
            dp[i][j] = min;
            dp[j][i] = max;
        }
    }
    println!("{}", dp[0][1]);
}
