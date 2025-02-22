use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    let a_max = 300_000;
    let mut seg_tree = SegTree::new(a_max + 1, 0);
    seg_tree.update(a[0], 1);
    for i in 1..n {
        let l = a[i] - a[i].min(k);
        let r = (a_max + 1).min(a[i] + k + 1);
        let x = seg_tree.query(l, r);
        seg_tree.update(a[i], x + 1);
    }
    println!("{}", seg_tree.query(0, a_max + 1));
}

// todo: snippet
struct SegTree {
    data: Vec<i32>,
    size: usize,
}
impl SegTree {
    fn new(n: usize, init_val: i32) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        let data = vec![init_val; size * 2];
        Self { data, size }
    }
    fn update(&mut self, pos: usize, val: i32) {
        let mut pos = pos + self.size;
        self.data[pos] = val;
        while pos > 1 {
            pos /= 2;
            self.data[pos] = self.data[pos * 2].max(self.data[pos * 2 + 1]);
        }
    }
    fn query_sub(&self, l: usize, r: usize, a: usize, b: usize, k: usize) -> i32 {
        if r <= a || b <= l {
            return std::i32::MIN;
        }
        if l <= a && b <= r {
            return self.data[k];
        }
        let m = (a + b) / 2;
        let val_l = self.query_sub(l, r, a, m, k * 2);
        let val_r = self.query_sub(l, r, m, b, k * 2 + 1);
        val_l.max(val_r)
    }
    // [l, r)
    fn query(&self, l: usize, r: usize) -> i32 {
        self.query_sub(l, r, 0, self.size, 1)
    }
}
