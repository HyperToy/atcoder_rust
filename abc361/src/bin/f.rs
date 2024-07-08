use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    const M: usize = 60;
    let mut f = [0; M];
    let mut answer = 1;
    for b in (2..M).rev() {
        f[b] = calc(b, n)
            - (2..M)
                .filter_map(|i| if i * b < M { Some(f[i * b]) } else { None })
                .sum::<usize>();
        answer += f[b];
    }
    println!("{}", answer);
}

// a^b <= n < (a+1)^b となる a
fn calc(b: usize, n: usize) -> usize {
    let mut ac = 0;
    let mut wa = n + 1;
    while ac + 1 < wa {
        let wj = (ac + wa) / 2;
        let judge = || {
            let mut x = 1;
            for _ in 0..b {
                if wj > n / x {
                    return false;
                }
                x *= wj;
            }
            x <= n
        };
        if judge() {
            ac = wj;
        } else {
            wa = wj;
        }
    }
    ac - 1
}
