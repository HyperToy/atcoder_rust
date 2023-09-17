use proconio::*;

fn main() {
    input! {
        n: usize,
        st: [(i32, usize); n],
    }
    let mut answer = Vec::new();
    for i in 0..10000 {
        let mut ok = true;
        for (s, t) in st.clone() {
            let mut same_count = 0;
            same_count += if s % 10 == i % 10 { 1 } else { 0 }; // 1の位
            same_count += if s / 10 % 10 == i / 10 % 10 { 1 } else { 0 }; // 10の位
            same_count += if s / 100 % 10 == i / 100 % 10 { 1 } else { 0 }; // 100の位
            same_count += if s / 1000 % 10 == i / 1000 % 10 { 1 } else { 0 }; // 1000の位
            match t {
                1 => {
                    ok &= same_count == 4;
                }
                2 => {
                    ok &= same_count == 3;
                }
                3 => {
                    ok &= same_count < 3;
                }
                _ => unreachable!(),
            }
        }
        if ok {
            answer.push(i);
        }
    }
    println!(
        "{}",
        if answer.len() == 1 {
            format!("{:04}", answer[0])
        } else {
            "Can't Solve".to_string()
        }
    );
}
