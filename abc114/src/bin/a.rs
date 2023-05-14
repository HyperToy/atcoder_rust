fn main() {
    proconio::input! {
        x: u32,
    }
    println! {
        "{}",
        match x {
            3 | 5 | 7 => "YES",
            _ => "NO",
        }
    }
}
