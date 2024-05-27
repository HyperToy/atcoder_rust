use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut answer = 0;
    let mut even = true;
    let mut min_diff = std::i64::MAX;
    for &(a, b) in &ab {
        answer += a.max(b);
        if a > b {
            even ^= true;
        }
        min_diff = min_diff.min((a - b).abs());
    }
    if !even {
        answer -= min_diff;
    }
    println!("{}", answer);
}
