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
    (0..=10)
        .rev()
        .fold((vec![], m), |(mut v, mut m), i| {
            let p = 3_u32.pow(i);
            while m >= p {
                m -= p;
                v.push(i);
            }
            (v, m)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(6), vec![1, 1]);
        assert_eq!(solve(100), vec![4, 2, 2, 0]);
        assert_eq!(
            solve(59048),
            vec![9, 9, 8, 8, 7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0],
        );
    }
}
