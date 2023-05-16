use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut cnt = [0; 200001];
    for x in a {
        cnt[x] += 1;
    }
    let mut ans: i64 = 0;
    for i in 1..=200000 {
        for j in (i..=200000).step_by(i) {
            ans += cnt[i] * cnt[j] * cnt[j / i];
        }
    }
    println!("{}", ans);
}
