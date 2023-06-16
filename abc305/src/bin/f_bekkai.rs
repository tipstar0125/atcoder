#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use num_traits::PrimInt;
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
    fn solve(&mut self) {
        let mut stdin =
            proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
        macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            N: usize,
            _M: usize
        }

        let mut G = vec![vec![]; N];
        let mut visited = vec![false; N];
        let mut Q = vec![];

        input! {
            k: usize,
            v: [Usize1; k]
        }
        for &vi in &v {
            G[0].push(vi);
            G[vi].push(0);
        }
        Q.push(0);
        let mut first = true;

        while !Q.is_empty() {
            let pos = Q.pop().unwrap();

            if first {
                first = false;
            } else {
                println!("{}", pos + 1);
                if pos + 1 == N {
                    input! {
                        _s: String
                    }
                    return;
                } else {
                    input! {
                        k: usize,
                        v: [Usize1; k]
                    }
                    if !visited[pos] {
                        for &vi in &v {
                            G[pos].push(vi);
                            G[vi].push(pos);
                        }
                    }
                }
            }

            if !visited[pos] {
                for &next in &G[pos] {
                    if !visited[next] {
                        Q.push(pos);
                        Q.push(next);
                    }
                }
            }
            visited[pos] = true;
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
