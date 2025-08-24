use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        m: u32,
    }
    let answer = solve(m);
    println!("{}\n{}", answer.len(), answer.iter().join(" "));
}

fn solve(m: u32) -> Vec<u32> {
    decompose_into_powers(m, 10)
}

fn decompose_into_powers(remaining: u32, power: u32) -> Vec<u32> {
    match (remaining, power) {
        (0, _) => vec![],
        (r, p) => {
            let current_power = 3_u32.pow(p);
            if r < current_power {
                decompose_into_powers(r, p - 1)
            } else {
                let current_powers = vec![p];
                let remaining_powers = decompose_into_powers(r - current_power, p);
                [current_powers, remaining_powers].concat()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(1), vec![0]);
        assert_eq!(solve(6), vec![1, 1]);
        assert_eq!(solve(100), vec![4, 2, 2, 0]);
        assert_eq!(
            solve(59048),
            vec![9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0],
        );
    }
}
