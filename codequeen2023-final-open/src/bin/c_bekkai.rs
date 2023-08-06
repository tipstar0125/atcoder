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
            S: Usize1,
            T: Usize1,
            UV: [(Usize1, Usize1); N-1]
        }

        let mut G = vec![vec![]; N];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
        }

        let mut visited = vec![false; N];
        visited[S] = true;
        let mut finish = false;
        dfs(S, &G, &mut visited, T, &mut finish);

        let INF = 1_usize << 60;
        let mut ans = vec![INF; N];
        let mut Q = VecDeque::new();
        for (i, &v) in visited.iter().enumerate() {
            if v {
                ans[i] = 1;
                Q.push_back(i);
            }
        }

        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                if ans[next] == INF {
                    ans[next] = ans[pos] + 1;
                    Q.push_back(next);
                }
            }
        }
        println!("{}", ans.iter().join(" "));
    }
}

fn dfs(pos: usize, G: &Vec<Vec<usize>>, visited: &mut Vec<bool>, goal: usize, finish: &mut bool) {
    if *finish {
        visited[pos] = false;
        return;
    }
    if pos == goal {
        *finish = true;
        return;
    }
    for &next in &G[pos] {
        if !visited[next] {
            visited[next] = true;
            dfs(next, G, visited, goal, finish);
            if !*finish {
                visited[next] = false;
            }
        }
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
