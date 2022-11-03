#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::{BTreeSet, HashMap};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[macro_export]
macro_rules! max {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::max($x, max!($( $y ),+))
        }
    }
#[macro_export]
macro_rules! min {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::min($x, min!($( $y ),+))
        }
    }
#[macro_export]
macro_rules! abs {
    ($x: expr) => {
        if $x < 0_isize {
            $x * (-1)
        } else {
            $x
        }
    };
}
#[macro_export]
macro_rules! absf {
    ($x: expr) => {
        if $x < 0.0 {
            $x * (-1.0)
        } else {
            $x
        }
    };
}

#[derive(Debug, Clone)]
struct UnionFind {
    parent: Vec<isize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n + 1],
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

    fn unite(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
        }
        self.size -= 1;
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
}

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            H: usize,
            W: usize,
            mut rs: usize,
            mut cs: usize,
            N: usize,
            rc: [(usize, usize); N],
            Q: usize,
            dl: [(char, usize); Q]
        }

        let mut map_row = HashMap::new(); // for LR
        let mut map_col = HashMap::new(); // for UD

        for &(r, c) in &rc {
            map_row.entry(r).or_insert(BTreeSet::new()).insert(c);
            map_row.entry(r).or_insert(BTreeSet::new()).insert(0);
            map_row.entry(r).or_insert(BTreeSet::new()).insert(W + 1);
            map_col.entry(c).or_insert(BTreeSet::new()).insert(r);
            map_col.entry(c).or_insert(BTreeSet::new()).insert(0);
            map_col.entry(c).or_insert(BTreeSet::new()).insert(H + 1);
        }

        for &(d, l) in &dl {
            match d {
                'L' => {
                    if map_row.get(&rs).is_some() {
                        let set = &map_row[&rs];
                        let wall = set.range(..cs).next_back().unwrap();
                        if cs - wall <= l {
                            cs = wall + 1;
                        } else {
                            cs -= l;
                        }
                    } else if cs <= l {
                        cs = 1;
                    } else {
                        cs -= l;
                    }
                }
                'R' => {
                    if map_row.get(&rs).is_some() {
                        let set = &map_row[&rs];
                        let wall = set.range(cs..).next().unwrap();
                        if wall - cs <= l {
                            cs = wall - 1;
                        } else {
                            cs += l;
                        }
                    } else if cs + l > W {
                        cs = W;
                    } else {
                        cs += l;
                    }
                }
                'U' => {
                    if map_col.get(&cs).is_some() {
                        let set = &map_col[&cs];
                        let wall = set.range(..rs).next_back().unwrap();
                        if rs - wall <= l {
                            rs = wall + 1;
                        } else {
                            rs -= l;
                        }
                    } else if rs <= l {
                        rs = 1;
                    } else {
                        rs -= l;
                    }
                }
                'D' => {
                    if map_col.get(&cs).is_some() {
                        let set = &map_col[&cs];
                        let wall = set.range(rs..).next().unwrap();
                        if wall - rs <= l {
                            rs = wall - 1;
                        } else {
                            rs += l;
                        }
                    } else if rs + l > H {
                        rs = H;
                    } else {
                        rs += l;
                    }
                }
                _ => unreachable!(),
            }
            println!("{} {}", rs, cs);
        }
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
