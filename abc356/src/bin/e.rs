use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let a = a.into_iter().sorted().collect::<Vec<_>>();
    let m = *a.last().unwrap();
    let mut c = vec![0; m + 1];
    for &x in &a {
        c[x] += 1;
    }
    let mut s = c.clone();
    for i in 1..=m {
        s[i] += s[i - 1];
    }
    let mut answer = 0;
    for (d, c) in c.into_iter().enumerate() {
        if c == 0 {
            continue;
        }
        for n in 1..=m / d {
            answer += n * f(&s, d, n) * c;
        }
        answer -= c * (c - 1) / 2;
    }
    answer -= n;
    println!("{}", answer);
}
fn f(c: &Vec<usize>, d: usize, n: usize) -> usize {
    let m = c.len() - 1;
    c[m.min(d * (n + 1) - 1)] - c[d * n - 1]
}
