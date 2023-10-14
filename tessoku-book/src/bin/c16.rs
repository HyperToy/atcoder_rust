use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize, k: i32,
        mut flights: [(Usize1, i32, Usize1, i32); m],
    }
    for i in 0..m {
        flights[i].3 += k;
    }

    // (時刻, 路線番号または空港番号, 出発か到着か)
    // 2: 出発, 1: 到着, 0: 最初と最後
    // ここで、出発の方が番号が大きい理由は、同じ時刻のときに到着をより早くするため
    let mut list = Vec::new();
    for i in 0..m {
        list.push((flights[i].1, i, 2)); // 出発
        list.push((flights[i].3, i, 1)); // 到着
    }
    for i in 0..n {
        list.push((-1, i, 0)); // 各空港 (始点)
        list.push((2_000_000_100, i, 0)); // 各空港 (終着)
    }
    list.sort();

    // 頂点番号の情報
    // 路線index -> そいつが list のどの index にいるか
    let mut vert_s = vec![0; m]; // 路線i の到着
    let mut vert_t = vec![0; m]; // 路線i の出発
    for i in 0..list.len() {
        match list[i].2 {
            2 => {
                vert_s[list[i].1] = i;
            }
            1 => {
                vert_t[list[i].1] = i;
            }
            _ => {}
        }
    }

    // それぞれの空港が，どの頂点番号を使うか
    let mut airport = vec![Vec::new(); n];
    for i in 0..list.len() {
        match list[i].2 {
            0 => {
                airport[list[i].1].push(i + 1);
            }
            1 => {
                airport[flights[list[i].1].2].push(i + 1);
            }
            2 => {
                airport[flights[list[i].1].0].push(i + 1);
            }
            _ => {}
        }
    }

    let l = 2 * (n + m + 1);
    let mut g = vec![Vec::new(); l];
    for i in 0..m {
        g[vert_t[i] + 1].push((vert_s[i] + 1, 1));
    }
    for i in 0..n {
        for j in 1..airport[i].len() {
            let u = airport[i][j - 1];
            let v = airport[i][j];
            g[v].push((u, 0));
        }
    }
    for i in 0..n {
        g[airport[i][0]].push((0, 0));
        g[l - 1].push((airport[i][airport[i].len() - 1], 0));
    }

    let mut dp = vec![0; l];
    for i in 1..l {
        for (u, score) in g[i].clone() {
            dp[i] = dp[i].max(dp[u] + score);
        }
    }
    println!("{}", dp[l - 1]);
}
