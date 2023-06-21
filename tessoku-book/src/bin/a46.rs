use lib::Point;
use proconio::*;

fn main() {
    // input
    input! {
        n: usize,
        cities: [(i32, i32); n],
    }
    let cities: Vec<Point> = cities.iter().map(|x| Point::new(x.0, x.1)).collect();

    // 常に「一番近い年」に向かう貪欲
    let mut answer = vec![0];
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut pre_idx = 1;
    for _ in 1..n {
        let pre_point = cities[pre_idx];
        let mut next_idx = 0;
        let mut min_sq_dist = std::i32::MAX;
        for i in 1..n {
            if seen[i] {
                continue;
            }
            let next_point = cities[i];
            let now_sq_dist = pre_point.calc_sq_dist(&next_point);
            if now_sq_dist < min_sq_dist {
                min_sq_dist = now_sq_dist;
                next_idx = i;
            }
        }
        answer.push(next_idx);
        seen[next_idx] = true;
        pre_idx = next_idx;
    }

    // output
    for x in answer {
        println!("{}", x + 1);
    }
    println!("{}", 1);
}

mod lib {
    #[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn calc_sq_dist(&self, other: &Point) -> i32 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            dx * dx + dy * dy
        }
    }
}
