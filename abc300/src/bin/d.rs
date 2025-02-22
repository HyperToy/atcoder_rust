use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut max: usize = 0;
    while 12 * (max + 1) * (max + 1) <= n {
        max += 1;
    }
    let prime_numbers = prime_numbers(max);
    let l = prime_numbers.len();
    let mut answer = 0usize;
    for i in 0..l {
        let c = prime_numbers[i];
        if c * c > n {
            break;
        }
        for j in 0..i {
            let b = prime_numbers[j];
            if b * c * c > n {
                break;
            }
            for k in 0..j {
                let a = prime_numbers[k];
                if a * a * b * c * c > n {
                    break;
                }
                answer += 1;
            }
        }
    }
    println!("{}", answer);
}
fn prime_numbers(max: usize) -> Vec<usize> {
    let mut is_prime = vec![true; max + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for a in 2..=max {
        if is_prime[a] {
            for i in 2..=(max / a) {
                is_prime[i * a] = false;
            }
        }
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(a, b)| if b { Some(a) } else { None })
        .collect_vec()
}
