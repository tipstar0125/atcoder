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
            T: usize,
            M: usize,
            AB: [(Usize1, Usize1); M]
        }

        let teams = vec![vec![0]];
        let mut ans = 0_usize;
        dfs(0, teams, N, T, &AB, &mut ans);
        println!("{}", ans);
    }
}

fn dfs(
    pos: usize,
    teams: Vec<Vec<usize>>,
    N: usize,
    T: usize,
    AB: &Vec<(usize, usize)>,
    ans: &mut usize,
) {
    if teams.len() == T && pos + 1 == N {
        let mut ok = true;
        for t in teams.iter() {
            if t.len() >= 2 {
                for v in t.iter().combinations(2) {
                    let a = *v[0];
                    let b = *v[1];
                    if AB.contains(&(a, b)) || AB.contains(&(b, a)) {
                        ok = false;
                    }
                }
            }
        }
        if ok {
            *ans += 1;
        }
    }

    if pos + 1 == N {
        return;
    }
    for i in 0..teams.len() {
        let mut next_teams = teams.clone();
        next_teams[i].push(pos + 1);
        dfs(pos + 1, next_teams, N, T, AB, ans);
    }
    if teams.len() + 1 > T {
        return;
    }
    let mut next_teams = teams;
    next_teams.push(vec![pos + 1]);
    dfs(pos + 1, next_teams, N, T, AB, ans);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
