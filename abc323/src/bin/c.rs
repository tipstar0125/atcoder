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
            A: [usize; M],
            S: [Chars; N]
        }

        let mut score = vec![0; N];
        for i in 0..N {
            let mut s = 0;
            for j in 0..M {
                if S[i][j] == 'o' {
                    s += A[j];
                }
            }
            score[i] = s + i + 1;
        }

        let m = *score.iter().max().unwrap();
        for i in 0..N {
            let mut diff = m as isize - score[i] as isize;
            let mut cnt = 0;
            let mut v = vec![];
            for j in 0..M {
                if S[i][j] == 'x' {
                    v.push(A[j]);
                }
            }
            v.sort();
            v.reverse();
            while diff > 0 {
                diff -= v[cnt] as isize;
                cnt += 1;
            }
            println!("{}", cnt);
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
