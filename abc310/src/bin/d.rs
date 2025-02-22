use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, t: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let ab = ab.into_iter().collect::<HashSet<_>>();
    let ng = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| ab.contains(&(i, j)) || ab.contains(&(j, i)))
                .collect_vec()
        })
        .collect_vec();
    let mut teams = vec![Vec::new(); t];
    println!("{}", dfs(0, t, n, &mut teams, &ng));
}
fn dfs(pos: usize, t: usize, n: usize, teams: &mut Vec<Vec<usize>>, ng: &Vec<Vec<bool>>) -> usize {
    if pos == n {
        return if teams.iter().all(|team| team.len() > 0) {
            1
        } else {
            0
        };
    }
    let mut res = 0;
    for j in 0..t {
        if teams[j].iter().any(|&i| ng[i][pos]) {
            continue;
        }
        teams[j].push(pos);
        res += dfs(pos + 1, t, n, teams, ng);
        teams[j].pop();
        if teams[j].len() == 0 {
            break;
        }
    }
    res
}
