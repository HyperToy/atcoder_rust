use proconio::input;

fn main() {
    input! {
        month: i32, day: i32,
        y: i32, m: i32, d: i32,
    }
    println!(
        "{} {} {}",
        if m == month && d == day { y + 1 } else { y },
        if d == day {
            if m == month {
                1
            } else {
                m + 1
            }
        } else {
            m
        },
        if d == day { 1 } else { d + 1 }
    );
}
