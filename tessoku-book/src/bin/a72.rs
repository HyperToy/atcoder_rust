use proconio::{marker::Chars, *};

fn main() {
    input! {
        h: usize, w: usize, k: usize,
        c: [Chars; h],
    }
    let mut answer = 0;
    for mask in 0..(1 << h) {
        let mut painted_row_count = 0;
        let mut is_painted = vec![false; h];
        for i in 0..h {
            if (mask >> i) & 1 == 1 {
                is_painted[i] = true;
                painted_row_count += 1;
            }
        }
        if painted_row_count > k {
            continue;
        }
        let mut brack_counts = vec![0; w];
        for j in 0..w {
            for i in 0..h {
                if is_painted[i] || (c[i][j] == '#') {
                    brack_counts[j] += 1;
                }
            }
        }
        brack_counts.sort();
        let mut now = 0;
        for j in 0..w {
            if j < k - painted_row_count {
                now += h;
            } else {
                now += brack_counts[j];
            }
        }
        answer = answer.max(now);
    }
    println!("{}", answer);
}
