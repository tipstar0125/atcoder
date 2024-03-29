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
            let INF = 1_usize << 60;
            let mut dist = vec![vec![INF; N]; N];
            let mut Q = VecDeque::new();
            Q.push_back((0, N - 1));
            dist[0][N - 1] = 0;
            while let Some((a, b)) = Q.pop_front() {
                let mut a_red = vec![];
                let mut a_blue = vec![];
                for &na in &G[a] {
                    if C[na] == 0 {
                        a_red.push(na);
                    } else {
                        a_blue.push(na);
                    }
                }
                let mut b_red = vec![];
                let mut b_blue = vec![];
                for &nb in &G[b] {
                    if C[nb] == 0 {
                        b_red.push(nb);
                    } else {
                        b_blue.push(nb);
                    }
                }
                for &na in &a_red {
                    for &nb in &b_blue {
                        if dist[a][b] + 1 < dist[na][nb] {
                            dist[na][nb] = dist[a][b] + 1;
                            Q.push_back((na, nb));
                        }
                    }
                }
                for &na in &a_blue {
                    for &nb in &b_red {
                        if dist[a][b] + 1 < dist[na][nb] {
                            dist[na][nb] = dist[a][b] + 1;
                            Q.push_back((na, nb));
                        }
                    }
                }
            }

            if dist[N - 1][0] < INF {
                println!("{}", dist[N - 1][0]);
            } else {
                println!("-1");
            }
        }
    }
}
#[derive(Debug, Clone)]
struct KthSum {
    N: usize,
    K: usize,
    S: isize,
    large: BTreeSet<(isize, usize)>,
    small: BTreeSet<(isize, usize)>,
}

impl KthSum {
    fn new(N: usize, K: usize) -> Self {
        let mut large = BTreeSet::new();
        let mut small = BTreeSet::new();
        for i in 0..K {
            large.insert((0, i));
        }
        for i in K..N {
            small.insert((0, i));
        }
        KthSum {
            N,
            K,
            S: 0,
            large,
            small,
        }
    }
    fn balance(&mut self) {
        while self.large.len() > self.K {
            let p = *self.large.iter().next().unwrap();
            self.large.remove(&p);
            self.small.insert(p);
            self.S -= p.0;
        }
        if self.small.is_empty() {
            return;
        }
        while self.large.iter().next().unwrap().0 < self.small.iter().next_back().unwrap().0 {
            let large_min = *self.large.iter().next().unwrap();
            let small_max = *self.small.iter().next_back().unwrap();
            self.large.remove(&large_min);
            self.small.remove(&small_max);
            self.large.insert(small_max);
            self.small.insert(large_min);
            self.S += small_max.0 - large_min.0;
        }
    }
    fn add(&mut self, p: (isize, usize)) {
        self.large.insert(p);
        self.S += p.0;
    }
    fn remove(&mut self, p: (isize, usize)) {
        if self.large.remove(&p) {
            self.S -= p.0;
        } else {
            self.small.remove(&p);
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
