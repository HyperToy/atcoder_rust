use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize, m: usize,
        trains: [(i64, i64, i64, i64, Usize1, Usize1); m],
    }
    let mut train_info = vec![Vec::new(); n];
    for train in Train::from(trains) {
        train_info[train.to].push(train);
    }
    let mut answer = vec![std::i64::MIN; n];
    let mut queue = BinaryHeap::new();
    answer[n - 1] = std::i64::MAX;
    queue.push((answer[n - 1], n - 1));
    while !queue.is_empty() {
        let (time_limit, now_station_id) = queue.pop().unwrap();
        if answer[now_station_id] > time_limit {
            continue;
        }
        for e in train_info[now_station_id].iter() {
            let next_station_id = e.from;
            if let Some(next_time) = e.get_last_train(time_limit) {
                if answer[next_station_id] < next_time {
                    answer[next_station_id] = next_time;
                    queue.push((answer[next_station_id], next_station_id));
                }
            }
        }
    }
    println!(
        "{}",
        answer
            .iter()
            .take(n - 1)
            .map(|v| if *v > std::i64::MIN {
                v.to_string()
            } else {
                "Unreachable".to_string()
            })
            .join(" ")
    )
}

#[derive(Clone)]
struct Train {
    from: usize,
    to: usize,
    first_train: i64,
    interval: i64,
    count: i64,
    distance: i64,
}
impl Train {
    fn from(trains: Vec<(i64, i64, i64, i64, usize, usize)>) -> Vec<Self> {
        trains
            .into_iter()
            .map(|(t, d, k, c, a, b)| Self::new(t, d, k, c, a, b))
            .collect()
    }
    fn new(
        first_train: i64,
        interval: i64,
        count: i64,
        distance: i64,
        from: usize,
        to: usize,
    ) -> Self {
        Self {
            first_train,
            interval,
            count,
            distance,
            from,
            to,
        }
    }
    // time_limit までに駅 to に到着する列車の出発時刻のうち、最後のものを求める
    fn get_last_train(&self, time_limit: i64) -> Option<i64> {
        if time_limit < self.first_train + self.distance {
            None
        } else {
            Some(
                self.first_train
                    + self.interval
                        * (self.count - 1)
                            .min((time_limit - self.first_train - self.distance) / self.interval),
            )
        }
    }
}
