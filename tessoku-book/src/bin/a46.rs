use lib::{parse_input, ChangeMinMax, Input, Output};
use rand::Rng;

fn main() {
    // input
    let input = parse_input();

    let state = generate_initial_solution(&input);
    let state = annealing(&input, state, 0.9, 3e6, 3e5);

    // output
    let output = Output::new(state.city_order.clone(), &input);
    println!("{}", output);
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

fn annealing(
    input: &Input,
    initial_solution: State,
    duration: f64,
    temp0: f64,
    temp1: f64,
) -> State {
    let mut solution = initial_solution;
    let mut best_solution = solution.clone();
    let mut current_cost = solution.calc_cost(&input);
    let mut best_cost = current_cost;
    let init_cost = current_cost;

    let mut all_iter = 0;
    let mut valid_iter = 0;
    let mut accepted_count = 0;
    let mut update_count = 0;
    let mut rng = rand::thread_rng();

    let duration_inv = 1.0 / duration;
    let since = std::time::Instant::now();

    let mut inv_temp = 1.0 / temp0; // 温度の逆数
    let mut time = 0.0;

    loop {
        all_iter += 1;
        if (all_iter & ((1 << 10) - 1)) == 0 {
            time = (std::time::Instant::now() - since).as_secs_f64() * duration_inv;
            let temp = f64::powf(temp0, 1.0 - time) * f64::powf(temp1, time);
            inv_temp = 1.0 / temp;

            if time >= 1.0 {
                break;
            }
        }
        let l: usize = rng.gen_range(1, input.city_count);
        let r: usize = rng.gen_range(l, input.city_count);

        let current_cost_partial = input.cities[solution.city_order[l]]
            .calc_sq_dist(&input.cities[solution.city_order[l - 1]])
            + input.cities[solution.city_order[r]]
                .calc_sq_dist(&input.cities[solution.city_order[r + 1]]);
        let new_cost_partial = input.cities[solution.city_order[l]]
            .calc_sq_dist(&input.cities[solution.city_order[r + 1]])
            + input.cities[solution.city_order[r]]
                .calc_sq_dist(&input.cities[solution.city_order[l - 1]]);

        // 増加するコスト。
        let cost_diff = new_cost_partial - current_cost_partial;

        if cost_diff <= 0 || rng.gen_bool(f64::exp(-cost_diff as f64 * inv_temp)) {
            solution.reverse_range(l, r);
            // 解の更新
            current_cost = solution.calc_cost(&input);
            accepted_count += 1;

            if best_cost.change_min(current_cost) {
                best_solution = solution.clone();
                update_count += 1;
            }
        }
        valid_iter += 1;
    }

    eprintln!("===== annealing =====");
    eprintln!("init cost  : {:.2}", init_cost);
    eprintln!("cost       : {:.2}", best_cost);
    eprintln!("all iter   : {}", all_iter);
    eprintln!("valid iter : {}", valid_iter);
    eprintln!("accepted   : {}", accepted_count);
    eprintln!("updated    : {}", update_count);
    eprintln!("");

    best_solution
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
    fn reverse_range(&mut self, mut l: usize, mut r: usize) {
        while l < r {
            let tmp = self.city_order[l];
            self.city_order[l] = self.city_order[r];
            self.city_order[r] = tmp;
            l += 1;
            r -= 1;
        }
    }
}

mod lib {
    use itertools::Itertools;
    use proconio::input;
    use std::fmt::Display;

    pub trait ChangeMinMax {
        fn change_min(&mut self, v: Self) -> bool;
        fn change_max(&mut self, v: Self) -> bool;
    }

    impl<T: PartialOrd> ChangeMinMax for T {
        fn change_min(&mut self, v: T) -> bool {
            *self > v && {
                *self = v;
                true
            }
        }
        fn change_max(&mut self, v: T) -> bool {
            *self < v && {
                *self = v;
                true
            }
        }
    }

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
