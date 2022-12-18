#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use std::collections::VecDeque;

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
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
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
}
#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            M: usize,
            uv: [(usize, usize); M]
        }

        let mut G = vec![vec![]; N + 1];
        for &(u, v) in &uv {
            G[u].push(v);
            G[v].push(u);
        }

        let mut color = vec![0; N + 1];
        let mut ans = N * (N - 1) / 2 - M;
        for i in 1..=N {
            if color[i] == 0 {
                color[i] = 1;
                let (mut black_num, white_num) = dfs(i, &G, &mut color);
                if black_num == -1 {
                    println!("0");
                    return;
                }
                black_num += 1;
                ans -= black_num as usize * (black_num as usize - 1) / 2;
                ans -= white_num as usize * (white_num as usize - 1) / 2;
            }
        }

        println!("{}", ans);
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

fn dfs(pos: usize, G: &Vec<Vec<usize>>, color: &mut Vec<isize>) -> (isize, isize) {
    let mut ret = (0_isize, 0_isize);
    for &next in &G[pos] {
        if color[pos] == color[next] {
            return (-1, -1);
        }
        if color[next] != 0 {
            continue;
        }
        color[next] = -color[pos];
        if color[next] == 1 {
            ret.0 += 1;
        } else {
            ret.1 += 1;
        }
        let (black_num, white_num) = dfs(next, G, color);
        if black_num == -1 {
            return (-1, -1);
        }
        ret.0 += black_num;
        ret.1 += white_num;
    }
    ret
}
