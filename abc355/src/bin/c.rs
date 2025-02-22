use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, t: usize,
        a: [Usize1; t],
    }
    let mut card = vec![vec![false; n]; n];
    let mut answer = -1;
    for (t, x) in a.into_iter().enumerate() {
        let (i, j) = (x / n, x % n);
        card[i][j] = true;
        let bingo = (0..n).all(|i| card[i][j])
            || (0..n).all(|j| card[i][j])
            || (0..n).all(|i| card[i][i])
            || (0..n).all(|i| card[i][n - 1 - i]);
        if bingo {
            answer = (t + 1) as i32;
            break;
        }
    }
    println!("{}", answer);
}
