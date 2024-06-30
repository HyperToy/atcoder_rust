use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, t: i64,
        s: Chars,
        x: [i64; n],
    }
    let mut stay = Vec::new();
    let mut moving = Vec::new();
    for i in 0..n {
        if s[i] == '0' {
            stay.push(x[i]);
        } else {
            moving.push(x[i]);
        }
    }
    if stay.is_empty() || moving.is_empty() {
        println!("0");
        return;
    }
    stay.sort();
    let lower_bound = |x| {
        if x < stay[0] {
            return 0;
        }
        let mut ok = 0;
        let mut ng = stay.len();
        while ok + 1 < ng {
            let wj = (ok + ng) / 2;
            if x >= stay[wj] {
                ok = wj;
            } else {
                ng = wj
            }
        }
        ok + 1
    };
    println!(
        "{}",
        moving
            .into_iter()
            .map(|x| lower_bound(x + t * 2) - lower_bound(x))
            .sum::<usize>()
    );
}
