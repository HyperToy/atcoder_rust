use proconio::input;
use std::collections::HashSet;

// TODO: refactoring
fn main() {
    input! {
        n: usize, (h, w): (usize, usize),
        tiles: [(usize, usize); n],
    }
    let mut ok = false;
    let mut seen = HashSet::new();
    'outer: for use_mask in 0..(1 << n) {
        let mut count = 0;
        let mut tile_area = 0;
        let mut use_tiles = Vec::new();
        for i in 0..n {
            if (use_mask >> i) & 1 == 1 {
                count += 1;
                tile_area += area(tiles[i]);
                use_tiles.push(tiles[i]);
            }
        }
        if tile_area != h * w {
            continue;
        }
        use_tiles.sort_by(|a, b| (b.0 * b.1).cmp(&(a.0 * a.1)));
        if seen.contains(&use_tiles) {
            continue;
        }
        for rev_mask in 0..(1 << count) {
            let mut crr_tiles = use_tiles.clone();
            for i in 0..count {
                if (rev_mask >> i) & 1 == 1 {
                    crr_tiles[i] = (use_tiles[i].1, use_tiles[i].0);
                }
            }
            if seen.contains(&crr_tiles) {
                continue;
            }
            seen.insert(crr_tiles.clone());
            let mut solver = DfsSolver::new(h, w, crr_tiles);
            solver.dfs(0);
            if solver.ok {
                ok = true;
                break 'outer;
            }
        }
        seen.insert(use_tiles);
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
fn area(t: (usize, usize)) -> usize {
    t.0 * t.1
}

struct DfsSolver {
    h: usize,
    w: usize,
    tiles: Vec<(usize, usize)>,
    used: Vec<Vec<bool>>,
    ok: bool,
}
impl DfsSolver {
    fn new(h: usize, w: usize, tiles: Vec<(usize, usize)>) -> Self {
        Self {
            h,
            w,
            tiles,
            used: vec![vec![false; w]; h],
            ok: false,
        }
    }
    fn dfs(&mut self, tile_id: usize) {
        if self.ok {
            return;
        }
        if tile_id == self.tiles.len() {
            if self.all() {
                self.ok = true;
            }
            return;
        }
        let tile = self.tiles[tile_id];
        if tile.0 > self.h || tile.1 > self.w {
            return;
        }
        for i in 0..=self.h - tile.0 {
            for j in 0..=self.w - tile.1 {
                if !self.check(tile, (i, j)) {
                    continue;
                }
                self.change(tile, (i, j), true);
                self.dfs(tile_id + 1);
                if self.ok {
                    return;
                }
                self.change(tile, (i, j), false);
            }
        }
    }
    fn check(&self, tile: (usize, usize), point: (usize, usize)) -> bool {
        for i in 0..tile.0 {
            for j in 0..tile.1 {
                if self.used[point.0 + i][point.1 + j] {
                    return false;
                }
            }
        }
        true
    }
    fn change(&mut self, tile: (usize, usize), point: (usize, usize), flag: bool) {
        for i in 0..tile.0 {
            for j in 0..tile.1 {
                self.used[point.0 + i][point.1 + j] = flag;
            }
        }
    }
    fn all(&self) -> bool {
        for i in 0..self.h {
            for j in 0..self.w {
                if !self.used[i][j] {
                    return false;
                }
            }
        }
        true
    }
}
