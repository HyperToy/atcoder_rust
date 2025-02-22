use proconio::*;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        b: [i32; m],
    }
    let mut ng = 0;
    let mut ok = 1_000_000_001;
    while ng + 1 < ok {
        let wj = (ng + ok) / 2;
        let seller = a.iter().filter(|x| &&wj >= x).count();
        let buyer = b.iter().filter(|x| &&wj <= x).count();
        if seller >= buyer {
            ok = wj;
        } else {
            ng = wj;
        }
    }
    println!("{}", ok);
}
