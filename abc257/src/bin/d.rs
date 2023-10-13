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
            XYP: [(isize, isize, isize); N]
        }

        fn eval(s: isize, N: usize, XYP: &Vec<(isize, isize, isize)>) -> bool {
            let mut G = vec![vec![]; N];
            for i in 0..N {
                let (x0, y0, p) = XYP[i];
                for j in 0..N {
                    if i == j {
                        continue;
                    }
                    let (x1, y1, _) = XYP[j];
                    let d = (x0 - x1).abs() + (y0 - y1).abs();
                    if p * s >= d {
                        G[i].push(j);
                    }
                }
            }

            for i in 0..N {
                let mut Q = VecDeque::new();
                let mut visited = vec![false; N];
                Q.push_back(i);
                visited[i] = true;
                while let Some(pos) = Q.pop_front() {
                    for &next in &G[pos] {
                        if !visited[next] {
                            visited[next] = true;
                            Q.push_back(next);
                        }
                    }
                }
                let ok = visited.iter().all(|b| *b);
                if ok {
                    return true;
                }
            }
            false
        }

        let mut ng = -1;
        let mut ok = 1e10 as isize;
        while ok - ng > 1 {
            let m = (ok + ng) / 2;
            if eval(m, N, &XYP) {
                ok = m;
            } else {
                ng = m;
            }
        }
        println!("{}", ok);
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
