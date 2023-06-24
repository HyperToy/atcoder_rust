use lib::{parse_input, Input, Output};

fn main() {
    // input
    let input = parse_input();

    let state = generate_initial_solution(&input);

    // output
    let output = Output::new(state.city_order.clone(), &input);
    println!("{}", output);
    eprintln!("cost: {:.0}", state.calc_cost(&input));
}

fn generate_initial_solution(input: &Input) -> State {
    // 常に「一番近い都市」に向かう貪欲
    let mut city_order = Vec::new();
    let mut seen = vec![false; input.city_count];
    let start_city = 0;
    city_order.push(start_city);
    seen[start_city] = true;
    let mut now_city = start_city;
    for _ in 1..input.city_count {
        let pre_point = input.cities[now_city];
        let mut next_city = 0;
        let mut min_sq_dist = std::i32::MAX;
        for i in 0..input.city_count {
            if seen[i] {
                continue;
            }
            let next_point = input.cities[i];
            let sq_dist = pre_point.calc_sq_dist(&next_point);
            if sq_dist < min_sq_dist {
                min_sq_dist = sq_dist;
                next_city = i;
            }
        }
        city_order.push(next_city);
        seen[next_city] = true;
        now_city = next_city;
    }
    city_order.push(start_city);
    State::new(city_order)
}

#[derive(Debug, Clone)]
struct State {
    city_order: Vec<usize>,
}
impl State {
    fn new(city_order: Vec<usize>) -> Self {
        let state = Self { city_order };
        state
    }
    fn calc_cost(&self, input: &Input) -> f64 {
        let mut cost = 0.;
        for i in 0..input.city_count {
            let sq_dist = input.cities[self.city_order[i]]
                .calc_sq_dist(&input.cities[self.city_order[i + 1]]);
            cost += (sq_dist as f64).sqrt();
        }
        cost.into()
    }
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
