use proconio::*;

fn main() {
    input! {
        n: usize,
        mut schedules: [(u32, u32); n],
    }
    // 比較関数を作成してソート
    schedules.sort_by(|a, b| a.1.cmp(&b.1));
    let mut now = 0;
    let mut answer = 0;
    for (l, r) in schedules {
        if now <= l {
            answer += 1;
            now = r;
        }
    }
    println!("{}", answer);
}
