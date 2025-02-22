use proconio::input;
use std::collections::BinaryHeap;

// refactor Reverse を使う
fn main() {
    input! {
        n: usize,
        td: [(i64, i64); n],
    }
    let mut waiting = td
        .into_iter()
        .map(|(t, d)| (-t, -(t + d)))
        .collect::<BinaryHeap<_>>();
    let mut in_progress = BinaryHeap::new();
    let mut now = 1;
    let mut answer = 0;
    while !waiting.is_empty() || !in_progress.is_empty() {
        if in_progress.is_empty() {
            // 次が来るまで時間を進める
            now = -waiting.peek().unwrap().0
        }
        // start == now であるものを waiting から in_progress に詰め替える
        while !waiting.is_empty() && now == -waiting.peek().unwrap().0 {
            let (start, end) = waiting.pop().unwrap();
            in_progress.push((end, start));
        }
        // in_progress を end が小さい順に調べる。 now > end は捨てる
        while !in_progress.is_empty() {
            let (end, _start) = in_progress.pop().unwrap();
            if now > -end {
                continue;
            }
            now += 1;
            answer += 1;
            break;
        }
    }
    println!("{}", answer);
}
