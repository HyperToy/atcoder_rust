use proconio::*;

fn main() {
    input! {
        n: usize,
        fuses: [(f64, f64); n],
    }
    let mut total_time = 0.;
    for fuse in fuses.iter().copied() {
        total_time += fuse.0 / fuse.1;
    }
    total_time /= 2.;
    let mut time = 0.;
    let mut len = 0.;
    for fuse in fuses {
        if time + fuse.0 / fuse.1 > total_time {
            len += (total_time - time) * fuse.1;
            break;
        }
        time += fuse.0 / fuse.1;
        len += fuse.0;
    }
    println!("{}", len);
}
