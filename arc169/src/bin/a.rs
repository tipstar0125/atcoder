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
            mut A: [isize; N],
            P: [Usize1; N - 1]
        }

        let mut G = vec![vec![]; N];
        let mut indegs = vec![0; N];
        let mut B = vec![0; N];
        for i in 1..N {
            let p = P[i - 1];
            G[i].push(p);
            indegs[p] += 1;
        }
        let mut Q = VecDeque::new();
        let mut ans = 0;
        for i in 0..N {
            if indegs[i] == 0 {
                Q.push_back(i);
            }
        }
        while let Some(pos) = Q.pop_front() {
            for &next in &G[pos] {
                B[next] += A[pos] + B[pos];
                indegs[next] -= 1;
                if indegs[next] == 0 {
                    if B[next] == 0 {
                        Q.push_back(next);
                    } else {
                        ans += B[next];
                    }
                }
            }
        }
        if A[0] == 0 && ans == 0 {
            println!("0");
        } else if ans > 0 || (ans == 0 && A[0] > 0) {
            println!("+");
        } else {
            println!("-");
        }
    }
}

fn dfs(pos: usize, G: &[Vec<usize>], ans: &mut isize, A: &[isize], B: &[isize]) {
    if G[pos].is_empty() {
        *ans += A[pos];
    }
    if B[pos] != 0 {
        for &next in &G[pos] {
            dfs(next, G, ans, A, B);
        }
    } else {
        *ans += A[pos];
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
