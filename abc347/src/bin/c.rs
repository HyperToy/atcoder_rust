use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, (a, b): (i64, i64),
        d: [i64; n],
    }
    let d = d
        .iter()
        .map(|x| x % (a + b))
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let mut first_event = std::i64::MAX;
    let mut last_event = std::i64::MIN;

    for x in &d {
        first_event = first_event.min(*x);
        last_event = last_event.max(*x);
    }
    let mut longest_free = 0;
    for i in 1..d.len() {
        longest_free = longest_free.max(d[i] - d[i - 1] - 1);
    }

    println!(
        "{}",
        if last_event - first_event + 1 <= a || longest_free >= b {
            "Yes"
        } else {
            "No"
        }
    );
}
