use proconio::*;

fn main() {
    input! {
        (n, l, k): (usize, usize, usize),
        ac: [(usize, i64); n],
    }
    println!(
        "{}",
        solve(
            l,
            k,
            ac.into_iter()
                .map(|(a, c)| GasStation { pos: a, cost: c })
                .collect::<Vec<_>>()
        )
    );
}

fn solve(distance: usize, capacity: usize, gas_stations: Vec<GasStation>) -> i64 {
    let mut rmq = SegTree::new(distance, 1i64 << 60);
    for station in gas_stations {
        rmq.update(
            station.pos,
            station.cost.min(rmq.query(station.pos, station.pos + 1)),
        );
    }
    let mut res = 0;
    for i in 1..=(distance - capacity) {
        let now = rmq.query(i, distance.min(i + capacity));
        if now == 1i64 << 60 {
            return -1;
        }
        res += now;
    }
    res
}

struct GasStation {
    pos: usize,
    cost: i64,
}

struct SegTree {
    data: Vec<i64>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: i64) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![init_val; size * 2];
        Self { data, size }
    }
    fn update(&mut self, pos: usize, val: i64) {
        let mut pos = pos + self.size;
        self.data[pos] = val;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2].min(self.data[pos * 2 + 1]);
        }
    }
    fn query(&self, l: usize, r: usize) -> i64 {
        self.query_sub(l, r, 0, self.size, 1)
    }
    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> i64 {
        if r <= a || b <= l {
            return 1i64 << 60;
        }
        if l <= a && b <= r {
            return self.data[k];
        }
        let m = (a + b) / 2;
        let val_l = self.query_sub(l, r, a, m, k * 2);
        let val_r = self.query_sub(l, r, m, b, k * 2 + 1);
        val_l.min(val_r)
    }
}
