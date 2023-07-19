use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        vs: [(Usize1, Usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in vs {
        g[u].push(v);
        g[v].push(u);
    }
    for u in 0..n {
        print!("{}: {}", u + 1, '{');
        g[u].sort();
        for v in 0..g[u].len() {
            print!(
                "{}{}",
                g[u][v] + 1,
                if v == g[u].len() - 1 { "" } else { ", " }
            );
        }
        println!("{}", '}');
    }
}
