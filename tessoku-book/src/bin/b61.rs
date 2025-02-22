use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (a, b) in es {
        g[a].push(b);
        g[b].push(a);
    }
    let max = g.iter().map(|v| v.len()).max().unwrap();
    let mut answer = 0;
    for i in 0..n {
        if max == g[i].len() {
            answer = i + 1;
            break;
        }
    }
    println!("{}", answer);
}
