use proconio::*;

fn main() {
    input! {
        n: usize, m: usize,
        p: [(i32, i32); n],
        q: [(i32, i32); m],
    }

    for i in 0..n {
        let mut ans = 0;
        let mut min = std::i32::MAX;
        for j in 0..m {
            let now: i32 = (p[i].0 - q[j].0).abs() + (p[i].1 - q[j].1).abs();
            if min > now {
                min = now;
                ans = j + 1;
            }
        }
        println!("{}", ans);
    }
}
