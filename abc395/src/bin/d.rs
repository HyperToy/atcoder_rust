use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, q: u32,
    }
    let mut f = (0..n).collect::<Vec<_>>(); // 鳩 -> 巣
    let mut g = (0..n).collect::<Vec<_>>(); // 巣 -> ラベル
    let mut h = (0..n).collect::<Vec<_>>(); // ラベル -> 鳩
    for _ in 0..q {
        input! { t: u8 }
        match t {
            1 => {
                input! { a: Usize1, b: Usize1 }
                f[a] = h[b];
            }
            2 => {
                input! { a: Usize1, b: Usize1 }
                let p = h[a]; // ラベルaの巣
                let q = h[b]; // ラベルbの巣
                h[a] = q;
                h[b] = p;
                g[p] = b;
                g[q] = a;
            }
            3 => {
                input! { a: Usize1 }
                println!("{}", g[f[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}
