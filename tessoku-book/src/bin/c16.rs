use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize, m: usize, k: i32,
        flights: [(Usize1, i32, Usize1, i32); m],
    }
    let flights = flights
        .into_iter()
        .enumerate()
        .map(|(index, (dep_port, dep_time, arr_port, arr_time))| Flight {
            dep_port,
            dep_time,
            arr_port,
            arr_time: arr_time + k,
            flight_id: index,
        })
        .collect::<Vec<_>>();

    let mut list = Vec::new();
    for flight in flights.clone() {
        list.push(FlightElement {
            time: flight.dep_time,
            kind: FlightKind::DEPARTURE(flight.flight_id),
            airport_id: flight.dep_port,
        }); // 出発
        list.push(FlightElement {
            time: flight.arr_time,
            kind: FlightKind::ARRIVAL(flight.flight_id),
            airport_id: flight.arr_port,
        }); // 到着
    }
    for airport_id in 0..n {
        list.push(FlightElement {
            time: -1,
            kind: FlightKind::AIRPORT,
            airport_id,
        }); // 各空港 (始点)
        list.push(FlightElement {
            time: 2_000_000_100,
            kind: FlightKind::AIRPORT,
            airport_id,
        }); // 各空港 (終着)
    }
    list.push(FlightElement {
        time: -2,
        kind: FlightKind::OTHER,
        airport_id: n,
    });
    list.push(FlightElement {
        time: 2_000_000_101,
        kind: FlightKind::OTHER,
        airport_id: n,
    });
    list.sort();

    // 頂点番号の情報
    // 路線index -> そいつが list のどの index にいるか
    let mut vert_s = vec![0; m]; // 路線i の到着
    let mut vert_t = vec![0; m]; // 路線i の出発
    for (node_id, flight) in list.clone().into_iter().enumerate() {
        match flight.kind {
            FlightKind::DEPARTURE(flight_id) => {
                vert_s[flight_id] = node_id;
            }
            FlightKind::ARRIVAL(flight_id) => {
                vert_t[flight_id] = node_id;
            }
            _ => {}
        }
    }

    // それぞれの空港が，どの頂点番号を使うか
    let mut airport = vec![Vec::new(); n + 1];
    for (node_id, flight_element) in list.clone().into_iter().enumerate() {
        airport[flight_element.airport_id].push(node_id);
    }

    // グラフを作る
    let l = 2 * (n + m + 1);
    let source = 0;
    let target = l - 1;
    let mut g = vec![Vec::new(); l];
    for flight_id in 0..m {
        g[vert_t[flight_id]].push((vert_s[flight_id], 1));
    }
    for i in 0..n {
        for j in 1..airport[i].len() {
            let u = airport[i][j - 1];
            let v = airport[i][j];
            g[v].push((u, 0));
        }
    }
    for airport_id in 0..n {
        g[airport_id + 1].push((source, 0));
        g[target].push((*airport[airport_id].last().unwrap(), 0));
    }

    let mut dp = vec![0; l];
    for i in 1..l {
        for (u, score) in g[i].clone() {
            dp[i] = dp[i].max(dp[u] + score);
        }
    }
    println!("{}", dp.last().unwrap());
}

#[derive(Clone)]
struct Flight {
    dep_port: usize,
    dep_time: i32,
    arr_port: usize,
    arr_time: i32,
    flight_id: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct FlightElement {
    time: i32,
    kind: FlightKind,
    airport_id: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Clone, Debug)]
enum FlightKind {
    AIRPORT,
    DEPARTURE(usize),
    ARRIVAL(usize),
    OTHER,
}

use std::cmp::Ordering;
impl Ord for FlightKind {
    fn cmp(&self, other: &Self) -> Ordering {
        use FlightKind::*;
        match (self, other) {
            (AIRPORT, AIRPORT) => Ordering::Equal,
            (ARRIVAL(_), ARRIVAL(_)) => Ordering::Equal,
            (DEPARTURE(_), DEPARTURE(_)) => Ordering::Equal,
            (OTHER, OTHER) => Ordering::Equal,
            (OTHER, _) => Ordering::Less,
            (AIRPORT, _) => Ordering::Less,
            (DEPARTURE(_), _) => Ordering::Greater,
            (ARRIVAL(_), _) => {
                if other == &AIRPORT {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        }
    }
}
