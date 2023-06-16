#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use num_traits::PrimInt;
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
    fn solve(&mut self) {
        let mut stdin =
            proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            N: usize,
            _M: usize
        }

        let mut G = vec![vec![]; N];
        let mut visited = vec![false; N];
        let mut used = vec![false; N];
        let mut first = true;

        input! {
            k: usize,
            v: [Usize1; k]
        }
        for &vi in &v {
            G[0].push(vi);
            G[vi].push(0);
        }
        used[0] = true;
        dfs(0, &mut G, &mut visited, &mut used, &mut first, N);
    }
}

fn dfs(
    pos: usize,
    G: &mut Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    used: &mut Vec<bool>,
    first: &mut bool,
    N: usize,
) -> bool {
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    if *first {
        *first = false;
    } else {
        println!("{}", pos + 1);
        if pos + 1 == N {
            input! {
                _s: String
            }
            return true;
        } else {
            input! {
                k: usize,
                v: [Usize1; k]
            }
            for &vi in &v {
                G[pos].push(vi);
                G[vi].push(pos);
            }
        }
    }
    visited[pos] = true;
    let mut next_list = vec![];
    for &next in &G[pos] {
        if !used[next] {
            used[next] = true;
            next_list.push(next);
        }
    }

    for &next in &next_list {
        if dfs(next, G, visited, used, first, N) {
            return true;
        } else {
            println!("{}", pos + 1);
            input! {
                k: usize,
                _v: [Usize1; k]
            }
        }
    }
    return false;
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
