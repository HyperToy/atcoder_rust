use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [i64; n],
        b: [usize; m],
    }
    let mut seg_tree = SegTree::new(n + 1, 0);
    for i in 0..n {
        seg_tree.add(i, a[i]);
        seg_tree.add(i + 1, -a[i]);
    }

    for b in b {
        let cnt = seg_tree.query_range_sum(0, b + 1);
        seg_tree.add(b, -cnt);
        seg_tree.add(b + 1, cnt);

        let cnt_p = cnt / (n as i64);
        let cnt_r = cnt % (n as i64);

        // (b + 1..=b + cnt_r) の範囲に +1
        // 0..n の範囲に +cnt_o
        if b + 1 == n {
            seg_tree.add(0, 1);
            seg_tree.add(cnt_r as usize, -1)
        } else {
            seg_tree.add(b + 1, 1);
            if (b + cnt_r as usize) < n {
                seg_tree.add(b + cnt_r as usize + 1, -1);
            } else {
                seg_tree.add(n, -1);
                seg_tree.add(0, 1);
                seg_tree.add((b + cnt_r as usize + 1) % n, -1);
            }
        }
        if cnt as usize >= n {
            seg_tree.add(0, cnt_p);
            seg_tree.add(n, -cnt_p);
        }
    }
    let mut answer = Vec::new();
    for i in 0..n {
        answer.push(seg_tree.query_range_sum(0, i + 1));
    }
    println!("{}", answer.iter().join(" "));
}

#[derive(Debug)]
struct SegTree {
    data: Vec<i64>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: i64) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![init_val; size * 2];
        Self { data, size }
    }
    fn add(&mut self, pos: usize, val: i64) {
        let mut pos = pos + self.size;
        self.data[pos] += val;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2] + self.data[pos * 2 + 1];
        }
    }
    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if l <= a && b <= r {
            return self.data[k];
        }
        let m = (a + b) / 2;
        let val_l = self.query_sub(l, r, a, m, k * 2);
        let val_r = self.query_sub(l, r, m, b, k * 2 + 1);
        val_l + val_r
    }
    fn query_range_sum(&self, l: usize, r: usize) -> i64 {
        self.query_sub(l, r, 0, self.size, 1)
    }
}
