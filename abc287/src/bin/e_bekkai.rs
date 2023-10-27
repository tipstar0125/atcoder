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
            S: [Chars; N]
        }

        let mut ans = vec![0; N];
        dfs(0, (0..N).collect_vec(), &S, &mut ans);
        println!("{}", ans.iter().join("\n"));
    }
}

fn dfs(now: usize, v: Vec<usize>, S: &Vec<Vec<char>>, ans: &mut Vec<usize>) {
    let mut mp = vec![vec![]; 26];
    for i in v {
        ans[i] = now;
        if S[i].len() == now {
            continue;
        }
        for j in 0..26 {
            if S[i][now] as u8 - b'a' == j as u8 {
                mp[j].push(i);
            }
        }
    }
    for i in 0..26 {
        if mp[i].len() < 2 {
            continue;
        }
        dfs(now + 1, mp[i].clone(), S, ans);
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
