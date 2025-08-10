use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut poss = vec![vec![]; n];
    for (i, &a) in a.iter().enumerate() {
        poss[a].push(i);
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..poss[i].len() {
            let now = poss[i][j];
            let next = if j + 1 == poss[i].len() {
                n
            } else {
                poss[i][j + 1]
            };
            ans += (now + 1) * (next - now);
        }
    }
    println!("{}", ans);
}
