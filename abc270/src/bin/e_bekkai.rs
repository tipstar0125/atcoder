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
            mut K: usize,
            A: [usize; N]
        }

        let mut B = vec![];
        for (i, &a) in A.iter().enumerate() {
            B.push((a, i));
        }
        B.sort();

        let mut S = 0_usize;
        let mut amari = 0_usize;
        for i in 0..N {
            let (mut x, idx) = B[i];
            x -= S;
            let c = N - i;
            if x * c <= K {
                K -= x * c;
                S += x;
                B[i] = (0, idx);
            } else {
                let ave = K / c;
                amari = K % c;
                x -= ave;
                K -= ave;
                B[i] = (x, idx);
                for j in i + 1..N {
                    let (mut y, idx2) = B[j];
                    y -= S;
                    y -= ave;
                    K -= ave;
                    B[j] = (y, idx2);
                }
                break;
            }
        }
        B.sort_by_key(|x| x.1);
        let mut ans = vec![];
        for i in 0..N {
            if B[i].0 > 0 && amari > 0 {
                ans.push(B[i].0 - 1);
                amari -= 1;
                K -= 1;
            } else {
                ans.push(B[i].0);
            }
        }
        assert!(K == 0);
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
