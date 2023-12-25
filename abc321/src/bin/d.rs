use proconio::*;

fn main() {
    input! {
        n: usize, m: usize, p: usize,
        a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();
    let mut sum = vec![0; m + 1];
    for j in 0..m {
        sum[j + 1] = sum[j] + b[j];
    }

    let mut answer = 0;
    for i in 0..n {
        if a[i] > p {
            answer += p * m;
            continue;
        }
        // b[j] > p - a[i] となる最小の j を求める
        let mut ng = -1;
        let mut ok = m as isize;
        while ng + 1 < ok {
            let wj = (ng + ok) / 2;
            if b[wj as usize] > p - a[i] {
                ok = wj;
            } else {
                ng = wj;
            }
        }
        let j = ok as usize;
        answer += j * a[i] + p * (m - j) + sum[j];
    }
    println!("{}", answer);
}
