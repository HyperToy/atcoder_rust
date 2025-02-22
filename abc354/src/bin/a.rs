use proconio::input;

fn main() {
    input! {
        h: i64,
    }
    let mut plant = 0;
    let mut growth = 1;
    for i in 0.. {
        if plant > h {
            println!("{}", i);
            break;
        }
        plant += growth;
        growth *= 2;
    }
}
