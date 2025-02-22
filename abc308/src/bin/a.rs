use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        s: [i32; 8],
    }
    println!(
        "{}",
        if s == s.clone().into_iter().sorted().collect::<Vec<_>>()
            && s.iter().all(|&x| 100 <= x && x <= 675 && x % 25 == 0)
        {
            "Yes"
        } else {
            "No"
        }
    );
}
