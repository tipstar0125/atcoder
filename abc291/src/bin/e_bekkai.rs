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
            M: usize,
            XY: [(Usize1, Usize1); M]
        }

        let mut G = vec![vec![]; N];
        let mut indegs = vec![0; N];
        for &(x, y) in &XY {
            indegs[y] += 1;
            G[x].push(y);
        }
        let mut Q = VecDeque::new();
        let mut v = vec![];
        for i in 0..N {
            if indegs[i] == 0 {
                Q.push_back(i);
                v.push(i);
            }
        }

        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                indegs[next] -= 1;
                if indegs[next] == 0 {
                    v.push(next);
                    Q.push_back(next);
                }
            }
        }
        if v.len() != N {
            println!("No");
            return;
        }

        let mut ans = vec![0; N];
        for i in 1..N {
            if !G[v[i - 1]].contains(&v[i]) {
                println!("No");
                return;
            }
        }
        for i in 0..N {
            ans[v[i]] = i + 1;
        }
        println!("Yes");
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
