use lib::{parse_input, Input, Output};

fn main() {
    // input
    let input = parse_input();

    let state = generate_initial_solution(&input);

    // output
    let output = Output::new(state.clone(), &input);
    println!("{}", output);
}

fn generate_initial_solution(input: &Input) -> Vec<usize> {
    // 常に「一番近い都市」に向かう貪欲
    let mut state = vec![0];
    let mut seen = vec![false; input.city_count];
    seen[0] = true;
    let mut pre_idx = 1;
    for _ in 1..input.city_count {
        let pre_point = input.cities[pre_idx];
        let mut next_idx = 0;
        let mut min_sq_dist = std::i32::MAX;
        for i in 1..input.city_count {
            if seen[i] {
                continue;
            }
            let next_point = input.cities[i];
            let now_sq_dist = pre_point.calc_sq_dist(&next_point);
            if now_sq_dist < min_sq_dist {
                min_sq_dist = now_sq_dist;
                next_idx = i;
            }
        }
        state.push(next_idx);
        seen[next_idx] = true;
        pre_idx = next_idx;
    }
    state.push(0);
    state
}

mod lib {
    use itertools::Itertools;
    use proconio::input;
    use std::fmt::Display;

    #[derive(Clone, Debug)]
    pub struct Input {
        pub city_count: usize,
        pub cities: Vec<Point>,
    }
    pub fn parse_input() -> Input {
        input! {
            n: usize,
            cities: [(i32, i32); n],
        }
        Input {
            city_count: n,
            cities: cities.iter().map(|&(x, y)| Point::new(x, y)).collect(),
        }
    }
    #[derive(Clone, Debug)]
    pub struct Output<'a> {
        pub city_order: Vec<usize>,
        input: &'a Input,
    }
    impl<'a> Output<'a> {
        pub fn new(city_order: Vec<usize>, input: &'a Input) -> Self {
            Self { city_order, input }
        }
        /*
        pub fn calc_score(&self, input: &Input) -> i64 {
            let is_connected = self.get_connection_status(input);
            let broadcasted = self.get_broadcasted_count(input, &is_connected);
            if broadcasted < input.resident_count {
                return broadcasted as i64;
            }

            self.calc_cost(input)
        }
        */
    }

    impl<'a> Display for Output<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.city_order
                    .iter()
                    .map(|p| (p + 1).to_string())
                    .join(" ")
            )?;
            Ok(())
        }
    }

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
