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
    // 各手番で、 i と j の大小関係が入れ替わる
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        dp[n - 1][i] = (v[n - 1] - v[i]).abs();
        dp[i][n - 1] = (v[n - 1] - v[i]).abs();
    }
    for i in (0..n - 1).rev() {
        let min = *dp[i][i + 1..n].iter().min().unwrap();
        let max = dp[i + 1..n].iter().map(|v| v[i]).max().unwrap();
        for j in 0..i {
            dp[i][j] = min;
            dp[j][i] = max;
        }
    }
    println!("{}", dp[0][1]);
}
