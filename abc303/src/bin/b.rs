use proconio::*;

fn main() {
    input! {
        n: usize, m: usize,
        a: [[usize; n]; m],
    }
    let mut ans = 0;
    for p in 1..=n {
        for q in 1..p {
            let mut discord = true;
            for i in 0..m {
                for j in 1..n {
                    // match (a[i][j - 1], a[i][j]) {
                    //     (p, q) | (q, p) => discord = false,
                    //     _ => (),
                    // }
                    if p == a[i][j - 1] && q == a[i][j] || p == a[i][j] && q == a[i][j - 1] {
                        discord = false;
                    }
                }
            }
            if discord {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
