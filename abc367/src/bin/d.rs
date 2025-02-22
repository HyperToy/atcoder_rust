use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }
    let mut v = vec![0; n * 2 - 1];
    for i in 0..n * 2 - 2 {
        v[i + 1] = (v[i] + a[i % n]) % m;
    }
    let mut answer: u64 = 0;
    let mut s = vec![0; m];
    for i in 0..n * 2 - 1 {
        if i >= n - 1 {
            answer += s[v[i]];
            s[v[i - (n - 1)]] -= 1;
        }
        s[v[i]] += 1;
    }
    println!("{}", answer);
}
