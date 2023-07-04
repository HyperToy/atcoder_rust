use proconio::*;

fn main() {
    input! {
        n: usize, t: i32,
        c: [i32; n],
        r: [i32; n],
    }
    // 存在判定
    let target = if c.iter().any(|x| x == &t) { t } else { c[0] };
    let mut winner = 0;
    let mut max = 0;
    for i in 0..n {
        if c[i] == target && r[i] > max {
            winner = i + 1;
            max = r[i];
        }
    }
    println!("{}", winner);
}
