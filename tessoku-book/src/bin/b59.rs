use proconio::{marker::Usize1, *};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let mut answer = 0;
    let mut rsq = SegTree::new(n, 0);
    for j in 0..n {
        answer += rsq.query(a[j] + 1, n);
        rsq.add(a[j], 1);
    }
    println!("{}", answer);
}

struct SegTree {
    data: Vec<usize>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![init_val; size * 2];
        Self { data, size }
    }
    fn add(&mut self, pos: usize, val: usize) {
        let mut pos = pos + self.size;
        self.data[pos] += val;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2] + self.data[pos * 2 + 1];
        }
    }

    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> usize {
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
    fn query(&self, l: usize, r: usize) -> usize {
        self.query_sub(l, r, 0, self.size, 1)
    }
}
