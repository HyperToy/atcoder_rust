use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        x: [Usize1; m],
    }
    // i と i+1 を繋ぐ橋を封鎖したときのコスト
    let mut v = vec![0; n + 1];
    for i in 0..m - 1 {
        let from = x[i];
        let to = x[i + 1];
        // ??? todo
        add(from, to, dist(to, from, n), n, &mut v);
        add(to, from, dist(from, to, n), n, &mut v);
    }
    let mut answer = std::i64::MAX;
    for i in 0..n {
        v[i + 1] += v[i];
        answer = answer.min(v[i]);
    }
    println!("{}", answer);
}
fn dist(from: usize, to: usize, n: usize) -> i64 {
    if from <= to {
        (to - from) as i64
    } else {
        (to + n - from) as i64
    }
}
fn add(from: usize, to: usize, num: i64, n: usize, v: &mut Vec<i64>) {
    if from <= to {
        v[from] += num;
        v[to] -= num;
    } else {
        v[from] += num;
        v[n] -= num;
        v[0] += num;
        v[to] -= num;
    }
}
