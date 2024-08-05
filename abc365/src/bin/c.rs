use proconio::input;

fn main() {
    input! {
        n: usize, m: i64,
        a: [i64; n],
    }
    if a.iter().sum::<i64>() <= m {
        println!("infinite");
        return;
    }
    let mut ok = 0;
    let mut ng = m;
    while ok + 1 < ng {
        let wj = (ok + ng) / 2;
        if a.iter().map(|&x| x.min(wj)).sum::<i64>() <= m {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok);
}
