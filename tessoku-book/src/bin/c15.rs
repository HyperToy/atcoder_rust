use proconio::*;

fn main() {
    input! {
        n: usize, k: usize,
        mut lr: [(usize, usize); n],
    }

    for i in 0..n {
        lr[i].1 += k;
    }
    let d = 86_400 * 2 + 1;

    let mut tmp = lr.clone();
    tmp.sort_by(|a, b| a.1.cmp(&b.1));
    let mut cnt_l = vec![0; d];
    let mut current_time = 0;
    let mut num = 0;
    for i in 0..n {
        if current_time <= tmp[i].0 {
            current_time = tmp[i].1;
            num += 1;
            cnt_l[current_time] = num;
        }
    }

    tmp.sort_by(|a, b| b.0.cmp(&a.0));
    let mut cnt_r = vec![0; d];
    let mut current_time = d - 1;
    let mut num = 0;
    for i in 0..n {
        if current_time >= tmp[i].1 {
            current_time = tmp[i].0;
            num += 1;
            cnt_r[current_time] = num;
        }
    }

    for i in 1..d {
        cnt_l[i] = cnt_l[i].max(cnt_l[i - 1]);
    }
    for i in (0..d - 1).rev() {
        cnt_r[i] = cnt_r[i].max(cnt_r[i + 1]);
    }

    for (l, r) in lr {
        println!("{}", cnt_l[l] + 1 + cnt_r[r]);
    }
}
