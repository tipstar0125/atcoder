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
            T: usize
        }

        for _ in 0..T {
            input! {
                N: usize,
                M: usize,
                C: [usize; N],
                UV: [(Usize1, Usize1); M]
            }

            let mut G = vec![vec![]; N];
            for &(u, v) in &UV {
                G[u].push(v);
                G[v].push(u);
            }
            let mut visited = vec![false; N];
            let mut color = vec![];
            let mut colors = BTreeSet::new();
            let INF = 1_usize << 60;
            let mut ans = INF;
            visited[0] = true;
            // color.push(C[0]);
            dfs(0, &G, &C, &mut visited, &mut color, &mut colors, &mut ans);
            if ans == INF {
                println!("-1");
            } else {
                println!("{}", ans);
            }
        }
    }
}

fn dfs(
    pos: usize,
    G: &Vec<Vec<usize>>,
    C: &Vec<usize>,
    visited: &mut Vec<bool>,
    color: &mut Vec<usize>,
    colors: &mut BTreeSet<Vec<usize>>,
    ans: &mut usize,
) {
    let N = G.len();
    if pos == N - 1 {
        let mut rev_color = color.clone();
        rev_color.pop();
        rev_color.reverse();
        rev_color.push(C[0]);
        rev_color = rev_color.iter().map(|x| x ^ 1).collect();
        colors.insert(rev_color);
        if colors.contains(color) {
            *ans = min!(*ans, color.len());
        }
        return;
    }

    for &next in &G[pos] {
        if !visited[next] {
            visited[next] = true;
            color.push(C[next]);
            dfs(next, G, C, visited, color, colors, ans);
            color.pop();
            visited[next] = false;
        }
    }
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
