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
        }

        let mut P = vec![vec![]; N];
        for i in 0..N {
            input! {
                c: usize,
            }
            if c == 0 {
                continue;
            }
            input! {
                p: [Usize1; c]
            }
            P[i] = p;
        }

        let mut books = BTreeSet::new();
        books.insert(0);
        let mut Q = VecDeque::new();
        Q.push_back(0);

        while let Some(pos) = Q.pop_front() {
            if P[pos].is_empty() {
                continue;
            }
            for &next in &P[pos] {
                if !books.contains(&next) {
                    books.push(next);
                    Q.push_back(next);
                }
            }
        }
        let mut indegs: BTreeMap<usize, usize> = BTreeMap::new();
        let mut G = vec![vec![]; N];
        for &b in books.iter() {
            indegs.insert(b, P[b].len());
            for &p in &P[b] {
                G[p].push(b);
            }
        }

        let mut ans = vec![];
        let mut QQ = VecDeque::new();
        for (k, v) in indegs.iter() {
            if *v == 0 {
                ans.push(*k);
                QQ.push_back(*k);
            }
        }

        while let Some(pos) = QQ.pop_front() {
            for &next in &G[pos] {
                *indegs.entry(next).or_default() -= 1;
                if indegs[&next] == 0 {
                    QQ.push_back(next);
                    ans.push(next);
                }
            }
        }
        let mut ans2 = vec![];
        for i in 0..ans.len() - 1 {
            ans2.push(ans[i] + 1);
        }

        println!("{}", ans2.iter().join(" "));
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
