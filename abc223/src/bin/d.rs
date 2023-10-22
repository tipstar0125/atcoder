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
            AB: [(Usize1, Usize1); M]
        }

        let mut G = vec![vec![]; N];
        let mut indegs = vec![0; N];
        for &(a, b) in &AB {
            G[a].push(b);
            indegs[b] += 1;
        }
        let mut Q = BinaryHeap::new();
        for i in 0..N {
            if indegs[i] == 0 {
                Q.push(Reverse(i));
            }
        }
        let mut ans = vec![];
        while let Some(Reverse(pos)) = Q.pop() {
            ans.push(pos + 1);
            for &next in &G[pos] {
                indegs[next] -= 1;
                if indegs[next] == 0 {
                    Q.push(Reverse(next));
                }
            }
        }
        if ans.len() == N {
            println!("{}", ans.iter().join(" "));
        } else {
            println!("-1");
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
