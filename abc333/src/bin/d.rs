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
            UV: [(Usize1, Usize1); N - 1]
        }
        let mut G = vec![vec![]; N];
        let mut degs = vec![0; N];
        for &(u, v) in &UV {
            G[u].push(v);
            G[v].push(u);
            degs[u] += 1;
            degs[v] += 1;
        }

        let mut Q = VecDeque::new();
        let mut visited = vec![false; N];
        let mut cnt = vec![0; N];
        for i in 0..N {
            if degs[i] == 1 {
                if i == 0 {
                    println!("1");
                    return;
                }
                Q.push_back(i);
                cnt[i] = 1;
                visited[i] = true;
            }
        }
        let mut a = vec![];
        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                if next == 0 {
                    a.push(cnt[pos]);
                }
                if !visited[next] {
                    cnt[next] += cnt[pos];
                    degs[next] -= 1;
                    if degs[next] == 1 {
                        cnt[next] += 1;
                        Q.push(next);
                        visited[next] = true;
                    }
                }
            }
        }
        a.sort();
        let mut ans = a.iter().sum::<usize>();
        ans -= a[a.len() - 1];
        ans += 1;
        println!("{}", ans);
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
