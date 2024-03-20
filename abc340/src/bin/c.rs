use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    println!("{}", Solver::new().solve(n));
}

struct Solver {
    memo: HashMap<i64, i64>,
}
impl Solver {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }
    fn solve(&mut self, n: i64) -> i64 {
        if self.memo.contains_key(&n) {
            return *self.memo.get(&n).unwrap();
        }
        let answer = if n == 1 {
            0
        } else {
            n + self.solve(n / 2) + self.solve((n + 1) / 2)
        };
        self.memo.insert(n, answer);
        answer
    }
}
