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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
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
            A: [usize; N],
            Q: usize
        }

        let INF = 1_usize << 60;
        let mut front: BTreeMap<usize, usize> = BTreeMap::new();
        let mut back: BTreeMap<usize, usize> = BTreeMap::new();
        for i in 0..N {
            if i == 0 {
                front.entry(A[i]).or_default();
            } else {
                *front.entry(A[i]).or_default() = A[i - 1];
            }
        }
        for i in 0..N - 1 {
            *back.entry(A[i]).or_default() = A[i + 1];
        }
        *back.entry(A[N - 1]).or_default() = INF;

        for _ in 0..Q {
            input! {
                t: usize,
            }
            if t == 1 {
                input! {
                    x: usize,
                    y: usize
                }
                let tmp = if back.contains_key(&x) { back[&x] } else { INF };
                *back.entry(x).or_default() = y;
                *front.entry(y).or_default() = x;
                *front.entry(tmp).or_default() = y;
                *back.entry(y).or_default() = tmp;
            } else {
                input! {
                    x: usize,
                }
                *front.entry(back[&x]).or_default() = front[&x];
                *back.entry(front[&x]).or_default() = back[&x];
                *front.entry(x).or_default() = INF;
                *back.entry(x).or_default() = INF;
            }
        }
        let mut now = 0;
        for (k, v) in front.iter() {
            if *v == 0 {
                now = *k;
                break;
            }
        }
        let mut ans = vec![];
        while now != INF {
            ans.push(now);
            now = back[&now];
        }
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
