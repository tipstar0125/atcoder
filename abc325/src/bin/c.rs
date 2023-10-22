#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};
use superslice::Ext;

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const DIJ8: [(usize, usize); 8] = [
    (!0, !0),
    (0, !0),
    (1, !0),
    (1, 0),
    (1, 1),
    (0, 1),
    (!0, 1),
    (!0, 0),
];

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            H: usize,
            W: usize,
            S: [Chars; H]
        }

        let mut uf = UnionFind::new(H * W);
        for i in 0..H {
            for j in 0..W {
                if S[i][j] == '.' {
                    continue;
                }
                for &(dr, dc) in &DIJ8 {
                    let nr = i.wrapping_add(dr);
                    let nc = j.wrapping_add(dc);
                    if nr >= H || nc >= W {
                        continue;
                    }
                    if S[nr][nc] == '.' {
                        continue;
                    }
                    let now = i * W + j;
                    let next = nr * W + nc;
                    uf.unite(now, next);
                }
            }
        }
        let mut ans = 0;
        for (k, _) in uf.all_group_members() {
            let r = k / W;
            let c = k % W;
            if S[r][c] == '.' {
                continue;
            }
            ans += 1
        }
        println!("{}", ans);
    }
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    roots: BTreeSet<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut roots = BTreeSet::new();
        for i in 0..n {
            roots.insert(i);
        }
        UnionFind {
            parent: vec![-1; n],
            roots,
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }
    fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            self.roots.remove(&root_y);
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            self.roots.remove(&root_x);
            Some((root_y, root_x))
        }
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn is_root(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
    fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    fn get_size(&self) -> usize {
        self.size
    }
    fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|i| self.find(*i) == root)
            .collect::<Vec<usize>>()
    }
    fn all_group_members(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut groups_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for x in 0..self.parent.len() {
            let r = self.find(x);
            groups_map.entry(r).or_default().push(x);
        }
        groups_map
    }
}
fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
