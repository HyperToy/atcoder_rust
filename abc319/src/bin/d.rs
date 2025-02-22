use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        l: [usize; n],
    }
    let mut ng = l.iter().max().unwrap() - 1;
    let mut ok = l.iter().sum::<usize>() + n - 1;
    while ng + 1 < ok {
        let wj = (ng + ok) / 2;
        let mut h = 1;
        let mut now = 0;
        for &x in &l {
            if now + x > wj {
                h += 1;
                now = 0;
            }
            now += x + 1;
        }
        if h <= m {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{ok}");
}
