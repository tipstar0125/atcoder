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
        }
        let mut v = vec![];
        let mut x = 1;
        for _ in 0..12 {
            v.push(x);
            x *= 10;
            x += 1;
        }
        let mut ans = vec![];
        dfs(0, 0, 3, &v, &mut vec![], &mut ans);
        ans.sort();
        println!("{}", ans[N - 1])
    }
}

fn dfs(depth: usize, s: usize, r: usize, v: &[usize], c: &mut Vec<usize>, ans: &mut Vec<usize>) {
    if depth == r {
        ans.push(c.iter().sum::<usize>());
        return;
    }
    for i in s..v.len() {
        let x = v[i];
        c.push(x);
        dfs(depth + 1, i, r, v, c, ans);
        c.pop();
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
