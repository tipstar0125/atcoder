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
            K: usize,
            AB: [(Usize1, Usize1); M],
            PH: [(Usize1, isize); K]
        }

        let mut G = vec![vec![]; N];
        for &(a, b) in &AB {
            G[a].push(b);
            G[b].push(a);
        }

        let mut power = vec![-1; N + 1];
        let mut Q = BinaryHeap::new();

        for &(p, h) in &PH {
            power[p] = h;
            Q.push((power[p], p));
        }

        while !Q.is_empty() {
            let (_, pos) = Q.pop().unwrap();
            for &next in &G[pos] {
                if power[next] < power[pos] - 1 {
                    power[next] = power[pos] - 1;
                    Q.push((power[next], next));
                }
            }
        }

        let mut ans = vec![];
        for i in 0..N {
            if power[i] >= 0 {
                ans.push(i + 1);
            }
        }
        println!("{}", ans.len());
        println!("{}", ans.iter().join(" "));
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
