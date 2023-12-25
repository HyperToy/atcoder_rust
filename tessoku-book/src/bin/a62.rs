use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut g = vec![Vec::new(); n];
    for (u, v) in es {
        g[u].push(v);
        g[v].push(u);
    }
    let mut stack = Vec::new();
    let mut seen = vec![false; n];
    let start = 0;
    stack.push(start);
    seen[start] = true;
    while !stack.is_empty() {
        let u = stack.pop().unwrap();
        for &v in &g[u] {
            if !seen[v] {
                stack.push(v);
                seen[v] = true;
            }
        }
    }
    println!(
        "The graph {} connected.",
        if seen.iter().all(|x| *x) {
            "is"
        } else {
            "is not"
        }
    );
}
