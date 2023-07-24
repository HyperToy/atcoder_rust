use itertools::Itertools;
use proconio::{marker::Usize1, *};

// functional graph の閉路を示す
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut g = vec![Vec::new(); n];
    let mut rg = vec![Vec::new(); n];
    for i in 0..n {
        g[i].push(a[i]);
        rg[a[i]].push(i);
    }
    let mut seen = vec![false; n];
    let (mut stack, _, _) = dfs(0, &g, &mut seen);
    stack.reverse();
    println!("{}", stack.len());
    println!("{}", stack.iter().map(|x| x + 1).join(" "));
}

fn dfs(pos: usize, g: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> (Vec<usize>, usize, bool) {
    if seen[pos] {
        return (vec![pos], pos, true);
    }
    seen[pos] = true;
    for next in g[pos].clone() {
        let (mut stack, last, mut to_add) = dfs(next, g, seen);
        if !stack.is_empty() {
            if last == pos {
                to_add = false;
            }
            if to_add {
                stack.push(pos);
            }
            return (stack, last, to_add);
        }
    }
    (Vec::new(), 0, false)
}
