use proconio::*;

fn main() {
    input! {
        s: String,
    }
    println!(
        "{}",
        match s.as_str() {
            "Monday"    => 5,
            "Tuesday"   => 4,
            "Wednesday" => 3,
            "Thursday"  => 2,
            "Friday"    => 1,
            "Saturday"  => 0,
            "Sunday"    => 0,
            _ => unreachable!(),
        }
    )
}
