use proconio::*;
// refactor

#[allow(non_snake_case)]
fn main() {
    // 入力部分
    input! {
        N: usize, M: usize, K: i64,
        ASBT: [(usize, i64, usize, i64); M],
    }
    let mut A = vec![0; 100009];
    let mut B = vec![0; 100009];
    let mut S = vec![0; 100009];
    let mut T = vec![0; 100009];

    // (時刻, 出発か到着か, 路線番号または空港番号)
    // 出発 = 2／到着 = 1／最初と最後 = 0
    // ここで、出発の方が番号が大きい理由は、同じ時刻のときに到着をより早くするため
    let mut List = Vec::new();

    // 頂点番号の情報
    let mut VertS = vec![0; 100009]; // 路線 i の到着
    let mut VertT = vec![0; 100009]; // 路線 i の出発
    let mut Airport = vec![Vec::new(); 100009];

    // グラフおよび dp[i]
    let mut G = vec![Vec::new(); 400009];
    let mut dp = vec![0; 400009];

    for i in 0..M {
        A[i + 1] = ASBT[i].0;
        S[i + 1] = ASBT[i].1;
        B[i + 1] = ASBT[i].2;
        T[i + 1] = ASBT[i].3 + K;
    }

    // 頂点となり得る (空港, 時刻) の組を「時刻の早い順に」ソート
    for i in 1..=M {
        List.push((S[i], 2, i));
    }
    for i in 1..=M {
        List.push((T[i], 1, i));
    }
    for i in 1..=N {
        List.push((-1, 0, i));
    }
    for i in 1..=N {
        List.push((2_100_000_000, 0, i));
    }
    List.sort();

    // 各路線の頂点番号を求める
    // ここで、頂点番号は時刻の早い順に 1, 2, ..., List.size() となる
    for i in 0..List.len() {
        if List[i].1 == 2 {
            VertS[List[i].2] = i + 1;
        }
        if List[i].1 == 1 {
            VertT[List[i].2] = i + 1;
        }
    }

    // 各空港の頂点番号を求める（空港で待つことに対応する実線を求めるときに使う）
    for i in 0..List.len() {
        if List[i].1 == 0 {
            Airport[List[i].2].push(i + 1);
        }
        if List[i].1 == 1 {
            Airport[B[List[i].2]].push(i + 1);
        }
        if List[i].1 == 2 {
            Airport[A[List[i].2]].push(i + 1);
        }
    }

    // グラフを作る（辺が逆向きになっていることに注意！）
    for i in 1..=M {
        G[VertT[i]].push((VertS[i], 1)); // 路線に対応する辺(点線)
    }
    for i in 1..=N {
        for j in 0..(Airport[i].len() - 1) {
            let idx1 = Airport[i][j];
            let idx2 = Airport[i][j + 1];
            G[idx2].push((idx1, 0)); // 空港で待つことに対応する辺
        }
    }

    // グラフに始点（頂点 0）と終点（頂点 List.size()+1）を追加
    for i in 1..=N {
        G[Airport[i][0]].push((0, 0));
        G[List.len() + 1].push((Airport[i][Airport[i].len() - 1], 0));
    }

    // 動的計画法によって dp[i] の値を求める
    // 頂点番号は時刻の早い順になっているので、dp[1] から順に計算すれば良い
    dp[0] = 0;
    for i in 1..=(List.len() + 1) {
        for j in 0..G[i].len() {
            dp[i] = std::cmp::max(dp[i], dp[G[i][j].0] + G[i][j].1);
        }
    }

    // 出力
    println!("{}", dp[List.len() + 1]);
}
