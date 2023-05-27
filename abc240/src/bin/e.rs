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

#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        input! {
            N: usize,
            UV: [(Usize1, Usize1); N - 1]
        }

        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
        }

        let mut cnt = 1_usize;
        let mut ans = vec![(0, 0); N];
        let mut visited = vec![false; N];
        visited[0] = true;
        dfs(0, &G, &mut visited, &mut ans, &mut cnt);

        for &a in &ans {
            println!("{} {}", a.0, a.1);
        }
    }
}

fn dfs(
    pos: usize,
    G: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    ans: &mut Vec<(usize, usize)>,
    cnt: &mut usize,
) -> (usize, usize) {
    let INF = 1_usize << 60;
    let mut lr = (INF, 0);
    for &next in &G[pos] {
        if !visited[next] {
            visited[next] = true;
            let (l, r) = dfs(next, G, visited, ans, cnt);
            lr.0 = min!(lr.0, l);
            lr.1 = max!(lr.1, r);
        }
    }

    if lr == (INF, 0) {
        lr = (*cnt, *cnt);
        *cnt += 1;
    }
    ans[pos] = lr;

    lr
}

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

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
