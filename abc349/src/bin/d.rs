use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (l, r): (u64, u64),
    }
    let mut p = l;
    let mut answer = Vec::new();
    while p < r {
        let i = std::cmp::min(max_exponent1(r - p), max_exponent2(p));
        answer.push((p, p + 2u64.pow(i)));
        p += 2u64.pow(i);
    }
    println!(
        "{}\n{}",
        answer.len(),
        answer
            .iter()
            .map(|(l, r)| format!("{} {}", l, r))
            .join("\n")
    );
}
// 2^i <= limit なる最大の i
fn max_exponent1(limit: u64) -> u32 {
    let mut ok = 0;
    let mut ng = 61;
    while ok + 1 < ng {
        let wj = (ok + ng) / 2;
        if 2u64.pow(wj) <= limit {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    ok
}
// target % 2^i == 0 なる最大の i
fn max_exponent2(target: u64) -> u32 {
    let mut ok = 0;
    let mut ng = 61;
    while ok + 1 < ng {
        let wj = (ok + ng) / 2;
        if target % 2u64.pow(wj) == 0 {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    ok
}
