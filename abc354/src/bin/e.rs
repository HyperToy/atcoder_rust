use proconio::input;

// wa
fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n],
    }
    let mut g = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..i {
            if ab[i].0 == ab[i].0 || ab[i].1 == ab[j].1 {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    let mut used = vec![false; n];
    println!(
        "{}",
        if dfs_takahashi(n, &g, &mut used) {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}

fn dfs_takahashi(n: usize, g: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> bool {
    let mut valid_move_exist = false;
    for i in 0..n {
        if used[i] {
            continue;
        }
        for &j in &g[i] {
            if used[j] {
                continue;
            }
            // 可能な手を全探索
            used[i] = true;
            used[j] = true;
            valid_move_exist &= !dfs_aoki(n, g, used);
            used[i] = false;
            used[j] = false;
        }
    }
    !valid_move_exist
}
fn dfs_aoki(n: usize, g: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> bool {
    let mut valid_move_exist = false;
    for i in 0..n {
        if used[i] {
            continue;
        }
        for &j in &g[i] {
            if used[j] {
                continue;
            }
            // 可能な手を全探索
            used[i] = true;
            used[j] = true;
            valid_move_exist &= !dfs_takahashi(n, g, used);
            used[i] = false;
            used[j] = false;
        }
    }
    !valid_move_exist
}
