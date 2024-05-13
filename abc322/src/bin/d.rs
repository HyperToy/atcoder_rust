use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut ps: [[Chars; 4]; 3],
    }
    let mut ok = false;
    for _ in 0..4 {
        for _ in 0..4 {
            ok |= dfs(0, &mut vec![vec![false; 4]; 4], &ps);
            ps[1] = rotate(&ps[1]);
        }
        ps[2] = rotate(&ps[2]);
    }
    println!("{}", if ok { "Yes" } else { "No" });
}

fn rotate(p: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut res = vec![vec!['.'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            res[3 - j][i] = p[i][j];
        }
    }
    res
}

fn dfs(i: usize, exist: &mut Vec<Vec<bool>>, ps: &Vec<Vec<Vec<char>>>) -> bool {
    if i == 3 {
        let mut ok = true;
        for u in 0..4 {
            for v in 0..4 {
                ok &= exist[u][v];
            }
        }
        return ok;
    }
    let mut ok = false;
    for di in -3..=3 {
        for dj in -3..=3 {
            let mut ex2 = exist.clone();
            ok |= can_put(&mut ex2, &ps[i], di, dj) && dfs(i + 1, &mut ex2, &ps);
        }
    }
    ok
}

fn can_put(exist: &mut Vec<Vec<bool>>, p: &Vec<Vec<char>>, di: isize, dj: isize) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if p[i][j] == '#' {
                let ni = i as isize + di;
                let nj = j as isize + dj;
                if !inside(ni, nj) {
                    return false;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if exist[ni][nj] {
                    return false;
                }
                exist[ni][nj] = true;
            }
        }
    }
    return true;
}

fn inside(i: isize, j: isize) -> bool {
    0 <= i && i < 4 && 0 <= j && j < 4
}
