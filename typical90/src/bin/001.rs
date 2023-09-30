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
            L: usize,
            K: usize,
            A: [usize; N]
        }

        let mut B = vec![];
        B.push(A[0]);
        for i in 1..N {
            B.push(A[i] - A[i - 1]);
        }
        B.push(L - A[N - 1]);

        let mut ok = 1;
        let mut ng = L;
        while ng - ok > 1 {
            let m = (ok + ng) / 2;
            if f(m, &B, K) {
                ok = m;
            } else {
                ng = m;
            }
        }
        println!("{}", ok);
    }
}
fn f(score: usize, B: &Vec<usize>, K: usize) -> bool {
    let mut cnt = 1;
    let mut s = 0;
    for b in B {
        if s < score {
            s += b;
        } else {
            s = *b;
            cnt += 1;
        }
    }
    if s < score {
        cnt -= 1;
    }
    cnt >= K + 1
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
