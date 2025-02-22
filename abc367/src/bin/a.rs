use proconio::input;

fn main() {
    input! {
        (shout, down, up): (i32, i32, i32),
    }
    let down = down + if up < down { 0 } else { 24 };
    let shout = shout + if up < shout { 0 } else { 24 };
    let ok = up < shout && shout < down;
    println!("{}", if ok { "Yes" } else { "No" });
}
