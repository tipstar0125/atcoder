#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use amplify::confinement::Collection;
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
            M: usize,
            ABXY: [(Usize1, Usize1, isize, isize); M]
        }

        let mut G = vec![vec![]; N];
        for &(a, b, x, y) in &ABXY {
            G[a].push((b, x, y));
            G[b].push((a, -x, -y));
        }
        let mut ans = vec![(-1, -1); N];
        ans[0] = (0, 0);
        let mut visited = vec![false; N];
        let mut Q = VecDeque::new();
        visited[0] = true;
        Q.push_back(0);
        while let Some(pos) = Q.pop_front() {
            for &(next, nx, ny) in &G[pos] {
                if !visited[next] {
                    visited[next] = true;
                    ans[next] = (ans[pos].0 + nx, ans[pos].1 + ny);
                    Q.push_back(next);
                }
            }
        }

        for row in ans {
            if row == (-1, -1) {
                println!("undecidable");
            } else {
                println!("{} {}", row.0, row.1);
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
