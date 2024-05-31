use ac_library::ModInt998244353 as Mint;
use proconio::input;

fn main() {
    input! {
        n: usize, a: usize, b: usize, p: usize, q: usize,
    }
    let mut dp = vec![vec![vec![None; 2]; n + q]; n + p];
    println!("{}", dfs(&mut dp, n, a, b, p, q, true));
}
// メモ化再帰
fn dfs(
    dp: &mut Vec<Vec<Vec<Option<Mint>>>>,
    n: usize,
    a: usize,
    b: usize,
    p: usize,
    q: usize,
    takahashi: bool,
) -> Mint {
    let flg = if takahashi { 0 } else { 1 };
    if let Some(x) = dp[a][b][flg] {
        x
    } else if a >= n {
        dp[a][b][flg] = Some(Mint::from(1));
        Mint::from(1)
    } else if b >= n {
        dp[a][b][flg] = Some(Mint::from(0));
        Mint::from(0)
    } else {
        let mut res = Mint::from(0);
        if takahashi {
            for i in 1..=p {
                res += dfs(dp, n, a + i, b, p, q, false) / p;
            }
        } else {
            for j in 1..=q {
                res += dfs(dp, n, a, b + j, p, q, true) / q;
            }
        }
        dp[a][b][flg] = Some(res);
        res
    }
}
